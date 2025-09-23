use crate::{Error, Solution};
use itertools::Itertools;

day!(Day21, 2015, 21, "RPG Simulator 20XX");

impl Solution for Day21 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let boss_stats = parse_input(input);
        let mut my_stats = [100, 0, 0];
        let mut least_gold = usize::MAX;

        let mut armor_combinations = vec![("", 0, 0, 0)];
        armor_combinations.extend(ARMOR);
        let mut ring_combinations = vec![vec![("", 0, 0, 0)]];
        ring_combinations.extend(RINGS.into_iter().map(|x| vec![x]));
        ring_combinations.extend(RINGS.into_iter().combinations(2));

        for weapon in WEAPONS {
            for armor in &armor_combinations {
                for rings in &ring_combinations {
                    let mut cost = weapon.1 + armor.1;
                    let mut damage = weapon.2 + armor.2;
                    let mut defense = weapon.3 + armor.3;

                    for ring in rings {
                        cost += ring.1;
                        damage += ring.2;
                        defense += ring.3;
                    }

                    my_stats[1] = damage;
                    my_stats[2] = defense;

                    if cost < least_gold && victory(my_stats, boss_stats) {
                        least_gold = cost;
                    }
                }
            }
        }

        Ok(least_gold.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        let boss_stats = parse_input(input);
        let mut my_stats = [100, 0, 0];
        let mut most_gold = 0;

        let mut armor_combinations = vec![("", 0, 0, 0)];
        armor_combinations.extend(ARMOR);
        let mut ring_combinations = vec![vec![("", 0, 0, 0)]];
        ring_combinations.extend(RINGS.into_iter().map(|x| vec![x]));
        ring_combinations.extend(RINGS.into_iter().combinations(2));

        for weapon in WEAPONS {
            for armor in &armor_combinations {
                for rings in &ring_combinations {
                    let mut cost = weapon.1 + armor.1;
                    let mut damage = weapon.2 + armor.2;
                    let mut defense = weapon.3 + armor.3;

                    for ring in rings {
                        cost += ring.1;
                        damage += ring.2;
                        defense += ring.3;
                    }

                    my_stats[1] = damage;
                    my_stats[2] = defense;

                    if cost > most_gold && !victory(my_stats, boss_stats) {
                        most_gold = cost;
                    }
                }
            }
        }

        Ok(most_gold.to_string())
    }
}

fn parse_input(input: &str) -> [usize; 3] {
    let mut boss = [0; 3];

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

fn victory(mut me: [usize; 3], mut boss: [usize; 3]) -> bool {
    let mut my_turn = true;

    loop {
        if my_turn {
            let mut taken_dmg = me[1].saturating_sub(boss[2]);
            if taken_dmg == 0 {
                taken_dmg = 1;
            };
            boss[0] = boss[0].saturating_sub(taken_dmg);
            if boss[0] == 0 {
                return true;
            }
        } else {
            let mut taken_dmg = boss[1].saturating_sub(me[2]);
            if taken_dmg == 0 {
                taken_dmg = 1;
            };
            me[0] = me[0].saturating_sub(taken_dmg);
            if me[0] == 0 {
                return false;
            }
        }
        my_turn = !my_turn;
    }
}

const WEAPONS: [(&str, usize, usize, usize); 5] = [
    ("Dagger", 8, 4, 0),
    ("Shortsword", 10, 5, 0),
    ("Warhammer", 25, 6, 0),
    ("Longsword", 40, 7, 0),
    ("Greataxe", 74, 8, 0),
];

const ARMOR: [(&str, usize, usize, usize); 5] = [
    ("Leather", 13, 0, 1),
    ("Chainmail", 31, 0, 2),
    ("Splintmail", 53, 0, 3),
    ("Bandedmail", 75, 0, 4),
    ("Platemail", 102, 0, 5),
];

const RINGS: [(&str, usize, usize, usize); 6] = [
    ("Damage +1", 25, 1, 0),
    ("Damage +2", 50, 2, 0),
    ("Damage +3", 100, 3, 0),
    ("Defense +1", 20, 0, 1),
    ("Defense +2", 40, 0, 2),
    ("Defense +3", 80, 0, 3),
];
