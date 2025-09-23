use crate::{Error, Solution};

day!(Day22, 2015, 22, "Wizard Simulator 20XX");

impl Solution for Day22 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let boss_stats = parse_input(input); // health, damage
        let my_stats = [50, 500]; // health, mana

        let (victory, mana) = least_mana_to_win(boss_stats, my_stats, true, [None; 3]);

        if !victory {
            Err(Error::NoSolution)
        } else {
            Ok(mana.to_string())
        }
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let boss_stats = parse_input(input); // health, damage
        let my_stats = [50, 500]; // health, mana

        let (victory, mana) = least_mana_to_win_hard(boss_stats, my_stats, true, [None; 3]);

        if !victory {
            Err(Error::NoSolution)
        } else {
            Ok(mana.to_string())
        }
    }
}

fn parse_input(input: &str) -> [usize; 2] {
    let mut boss = [0; 2]; // health, damage

    for (i, line) in input.trim().lines().enumerate() {
        boss[i] = line
            .split_whitespace()
            .next_back()
            .unwrap()
            .parse::<usize>()
            .unwrap();
    }

    boss
}

const SPELLS: [(&str, usize); 5] = [
    ("Magic Missile", 53), // instant 4 damage
    ("Drain", 73),         // instant 2 damage, 2 health
    ("Shield", 113),       // 6 turns, armor +7
    ("Poison", 173),       // 6 turns, 3 damage
    ("Recharge", 229),     // 5 turns, 101 mana
];

fn least_mana_to_win(
    mut boss_stats: [usize; 2],
    mut my_stats: [usize; 2],
    turn: bool,
    mut active_spells: [Option<((&str, usize), usize)>; 3],
) -> (bool, usize) {
    // (true if I win, mana spent)

    // check if shield spell is active
    let my_armor: usize = if let Some((spell, turns)) = active_spells[0] {
        assert_eq!(spell.0, "Shield");
        if turns == 0 {
            active_spells[0] = None;
        } else {
            active_spells[0] = Some((spell, turns - 1));
        }
        7
    } else {
        0
    };

    // check if poison spell is active
    if let Some((spell, turns)) = active_spells[1] {
        assert_eq!(spell.0, "Poison");
        boss_stats[0] = boss_stats[0].saturating_sub(3);
        if boss_stats[0] == 0 {
            return (true, 0);
        }
        if turns == 0 {
            active_spells[1] = None;
        } else {
            active_spells[1] = Some((spell, turns - 1));
        }
    }

    // check if the recharge spell is active
    if let Some((spell, turns)) = active_spells[2] {
        assert_eq!(spell.0, "Recharge");
        my_stats[1] += 101;
        if turns == 0 {
            active_spells[2] = None;
        } else {
            active_spells[2] = Some((spell, turns - 1));
        }
    }

    if turn {
        // my turn
        // check if I can cast any spell
        if my_stats[1] < SPELLS[0].1 {
            return (false, 0);
        }

        let mut least_mana = usize::MAX;
        let mut victory = false;

        // cast Magic Missile, there is already enough mana (checked)
        let mana = SPELLS[0].1;
        let new_my_stats = [my_stats[0], my_stats[1] - mana];
        let new_boss_stats = [boss_stats[0].saturating_sub(4), boss_stats[1]];
        if new_boss_stats[0] == 0 {
            return (true, mana); // return immediately because 53 is the least mana possible to spend
        } else {
            let (new_victory, mut new_mana) =
                least_mana_to_win(new_boss_stats, new_my_stats, !turn, active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Drain
        let mana = SPELLS[1].1;
        if my_stats[1] >= mana {
            let new_my_stats = [my_stats[0] + 2, my_stats[1] - mana];
            let new_boss_stats = [boss_stats[0].saturating_sub(2), boss_stats[1]];
            if new_boss_stats[0] == 0 {
                victory = true;
                if mana < least_mana {
                    least_mana = mana;
                }
            } else {
                let (new_victory, mut new_mana) =
                    least_mana_to_win(new_boss_stats, new_my_stats, !turn, active_spells);
                new_mana += mana;
                if victory {
                    // it was won already
                    if new_victory && new_mana < least_mana {
                        // if I also won check if it was with less mana
                        least_mana = new_mana;
                    }
                } else if new_victory {
                    // it was not won yet, but I won now
                    victory = true;
                    least_mana = new_mana;
                } else if new_mana < least_mana {
                    // it was not won yet, and I did not win now, but I spent less mana
                    least_mana = new_mana;
                }
            }
        }

        // cast Shield
        let mana = SPELLS[2].1;
        if active_spells[0].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[0] = Some((SPELLS[2], 6 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Poison
        let mana = SPELLS[3].1;
        if active_spells[1].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[1] = Some((SPELLS[3], 6 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Recharge
        let mana = SPELLS[4].1;
        if active_spells[2].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[2] = Some((SPELLS[4], 5 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        (victory, least_mana)
    } else {
        // boss turn
        let dmg = boss_stats[1].saturating_sub(my_armor);
        my_stats[0] = my_stats[0].saturating_sub(if dmg == 0 { 1 } else { dmg });
        if my_stats[0] == 0 {
            (false, 0)
        } else {
            least_mana_to_win(boss_stats, my_stats, !turn, active_spells)
        }
    }
}

fn least_mana_to_win_hard(
    mut boss_stats: [usize; 2],
    mut my_stats: [usize; 2],
    turn: bool,
    mut active_spells: [Option<((&str, usize), usize)>; 3],
) -> (bool, usize) {
    // (true if I win, mana spent)
    if turn {
        my_stats[0] = my_stats[0].saturating_sub(1);
        if my_stats[0] == 0 {
            return (false, 0);
        }
    }

    // check if shield spell is active
    let my_armor: usize = if let Some((spell, turns)) = active_spells[0] {
        assert_eq!(spell.0, "Shield");
        if turns == 0 {
            active_spells[0] = None;
        } else {
            active_spells[0] = Some((spell, turns - 1));
        }
        7
    } else {
        0
    };

    // check if poison spell is active
    if let Some((spell, turns)) = active_spells[1] {
        assert_eq!(spell.0, "Poison");
        boss_stats[0] = boss_stats[0].saturating_sub(3);
        if boss_stats[0] == 0 {
            return (true, 0);
        }
        if turns == 0 {
            active_spells[1] = None;
        } else {
            active_spells[1] = Some((spell, turns - 1));
        }
    }

    // check if recharge spell is active
    if let Some((spell, turns)) = active_spells[2] {
        assert_eq!(spell.0, "Recharge");
        my_stats[1] += 101;
        if turns == 0 {
            active_spells[2] = None;
        } else {
            active_spells[2] = Some((spell, turns - 1));
        }
    }

    if turn {
        // my turn
        // check if I can cast any spell
        if my_stats[1] < SPELLS[0].1 {
            return (false, 0);
        }

        let mut least_mana = usize::MAX;
        let mut victory = false;

        // cast Magic Missile, there is already enough mana (checked)
        let mana = SPELLS[0].1;
        let new_my_stats = [my_stats[0], my_stats[1] - mana];
        let new_boss_stats = [boss_stats[0].saturating_sub(4), boss_stats[1]];
        if new_boss_stats[0] == 0 {
            return (true, mana); // return immediately because 53 is the least mana possible to spend
        } else {
            let (new_victory, mut new_mana) =
                least_mana_to_win_hard(new_boss_stats, new_my_stats, !turn, active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Drain
        let mana = SPELLS[1].1;
        if my_stats[1] >= mana {
            let new_my_stats = [my_stats[0] + 2, my_stats[1] - mana];
            let new_boss_stats = [boss_stats[0].saturating_sub(2), boss_stats[1]];
            if new_boss_stats[0] == 0 {
                victory = true;
                if mana < least_mana {
                    least_mana = mana;
                }
            } else {
                let (new_victory, mut new_mana) =
                    least_mana_to_win_hard(new_boss_stats, new_my_stats, !turn, active_spells);
                new_mana += mana;
                if victory {
                    // it was won already
                    if new_victory && new_mana < least_mana {
                        // if I also won check if it was with less mana
                        least_mana = new_mana;
                    }
                } else if new_victory {
                    // it was not won yet, but I won now
                    victory = true;
                    least_mana = new_mana;
                } else if new_mana < least_mana {
                    // it was not won yet, and I did not win now, but I spent less mana
                    least_mana = new_mana;
                }
            }
        }

        // cast Shield
        let mana = SPELLS[2].1;
        if active_spells[0].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[0] = Some((SPELLS[2], 6 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win_hard(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Poison
        let mana = SPELLS[3].1;
        if active_spells[1].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[1] = Some((SPELLS[3], 6 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win_hard(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        // cast Recharge
        let mana = SPELLS[4].1;
        if active_spells[2].is_none() && my_stats[1] >= mana {
            let new_my_stats = [my_stats[0], my_stats[1] - mana];
            let mut new_active_spells = active_spells;
            new_active_spells[2] = Some((SPELLS[4], 5 - 1));
            let (new_victory, mut new_mana) =
                least_mana_to_win_hard(boss_stats, new_my_stats, !turn, new_active_spells);
            new_mana += mana;
            if victory {
                // it was won already
                if new_victory && new_mana < least_mana {
                    // if I also won check if it was with less mana
                    least_mana = new_mana;
                }
            } else if new_victory {
                // it was not won yet, but I won now
                victory = true;
                least_mana = new_mana;
            } else if new_mana < least_mana {
                // it was not won yet, and I did not win now, but I spent less mana
                least_mana = new_mana;
            }
        }

        (victory, least_mana)
    } else {
        // boss turn
        let dmg = boss_stats[1].saturating_sub(my_armor);
        my_stats[0] = my_stats[0].saturating_sub(if dmg == 0 { 1 } else { dmg });
        if my_stats[0] == 0 {
            (false, 0)
        } else {
            least_mana_to_win_hard(boss_stats, my_stats, !turn, active_spells)
        }
    }
}
