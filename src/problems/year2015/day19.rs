use crate::{Error, Solution};
use regex::Regex;
use std::collections::HashSet;

day!(Day19, 2015, 19, "Medicine for Rudolph");

impl Solution for Day19 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let (substitutions, molecule_str) = parse_input(input);
        let molecule_str = String::from(molecule_str);

        let mut new_molecules = HashSet::new();
        for substitution in substitutions {
            let positions = sub_string_positions(&molecule_str, substitution.0);

            for pos in positions {
                let mut new_molecule = molecule_str.clone();
                new_molecule.replace_range(pos..(pos + substitution.0.len()), substitution.1);
                new_molecules.insert(new_molecule);
            }
        }

        Ok(new_molecules.len().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, Error> {
        // wanted input molecule
        let mut molecule_str = String::from(parse_input(input).1);

        // solution using insights provided by u/askalski, confirmed by u/topaz2078 (creator of the AoC)
        // possible substitutions are
        // e => XX
        // X => XX
        // X => X Rn X Ar
        // X => X Rn X Y X Ar
        // X => X Rn X Y X Y X Ar
        // where X is any atom ("token"), for example Ca, Ti, P, B, ..., except Rn, Ar, Y
        // Rn can be replaced with ( and Ar with )
        // Y can be replaced with ,
        // That transforms substitutions to
        // e => XX
        // X => XX
        // X => X ( X )
        // X => X ( X , X )
        // X => X ( X , X , X )

        // first we need to replace Rn and Ar with ( and ), and Y with ,
        // then we need to replace all atoms/tokens with X

        // replace all Rn with (
        while let Some(pos) = molecule_str.find("Rn") {
            molecule_str.replace_range(pos..(pos + 2), "(");
        }

        // replace all Ar with )
        while let Some(pos) = molecule_str.find("Ar") {
            molecule_str.replace_range(pos..(pos + 2), ")");
        }

        // replace all Y with ,
        while let Some(pos) = molecule_str.find('Y') {
            molecule_str.replace_range(pos..(pos + 1), ",");
        }

        // replace all tokens with X
        let re = Regex::new(r"[A-Z][a-z]?").unwrap();
        molecule_str = re.replace_all(&molecule_str, "X").to_string();

        // Now from this information, we can calculate the number of steps needed to transform the molecule
        // If we apply X => XX substitution , the molecule gets reduced by 1 so the total number of steps is:
        // the length of the molecule - 1
        // If we apply X => X ( X ) substitution, the molecule gets reduced by 3 so the total number of steps is:
        // the length of the molecule - the number of ( or ) - 1
        // If we apply X => X ( X , X ) substitution, the molecule gets reduced by 5
        // If we apply X => X ( X , X , X ) substitution, the molecule gets reduced by 7
        // That is for each , the molecule gets reduced by 2
        // So the final equation for the number of steps is:
        // the length of the molecule - the number of ( or ) - 2 * the number of , - 1

        Ok((molecule_str.chars().count()
            - molecule_str.matches(['(', ')']).count()
            - 2 * molecule_str.matches(',').count()
            - 1)
        .to_string())
    }
}

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let mut substitutions = Vec::new();
    let mut full_string = "";

    for line in input.trim().lines() {
        if !line.is_empty() {
            if line.contains("=>") {
                let mut split = line.split(" => ");
                substitutions.push((split.next().unwrap(), split.next().unwrap()));
            } else {
                full_string = line;
            }
        }
    }

    (substitutions, full_string)
}

fn sub_string_positions(string: &str, sub_string: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = string.get(start..).unwrap().find(sub_string) {
        start += pos;
        positions.push(start);
        start += 1;
    }

    positions
}
