use crate::{Error, Solution};
use std::collections::HashMap;

day!(Day07, 2017, 7, "Recursive Circus");

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        Ok(Structure::new(input).root().name().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        Ok(Structure::new(input).fixed_weight().to_string())
    }
}

struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Option<Vec<Self>>,
}
impl<'a> Program<'a> {
    fn new(name: &'a str, weight: u32, children: Option<Vec<Self>>) -> Self {
        Self {
            name,
            weight,
            children,
        }
    }

    fn add_child(&mut self, child: Self) {
        match &mut self.children {
            Some(children) => children.push(child),
            None => self.children = Some(vec![child]),
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}
struct Structure<'a> {
    root: Program<'a>,
}
impl<'a> Structure<'a> {
    fn new(input: &'a str) -> Self {
        let mut programs_with_children = Vec::new();

        for line in input.lines() {
            match line.split_once(" -> ") {
                Some((name_weight, children)) => {
                    let (name, weight) = name_weight.split_once(" (").unwrap();
                    let weight = weight.trim_end_matches(')').parse().unwrap();
                    programs_with_children.push((
                        Program::new(name, weight, None),
                        Some(children.split(", ").collect::<Vec<_>>()),
                    ));
                }
                None => {
                    let (name, weight) = line.split_once(" (").unwrap();
                    let weight = weight.trim_end_matches(')').parse().unwrap();
                    programs_with_children.push((Program::new(name, weight, None), None));
                }
            }
        }

        while programs_with_children.len() > 1 {
            let mut i = 0;
            while i < programs_with_children.len() && programs_with_children.len() != 1 {
                if programs_with_children[i].1.is_none() {
                    let this_program = programs_with_children.remove(i).0;
                    let this_program_name = this_program.name;
                    let parent_ind = programs_with_children
                        .iter()
                        .position(|(_, c)| {
                            c.is_some() && c.as_ref().unwrap().contains(&this_program_name)
                        })
                        .unwrap();
                    programs_with_children[parent_ind].0.add_child(this_program);
                    programs_with_children[parent_ind]
                        .1
                        .as_mut()
                        .unwrap()
                        .retain(|&c| c != this_program_name);
                    if programs_with_children[parent_ind]
                        .1
                        .as_ref()
                        .unwrap()
                        .is_empty()
                    {
                        programs_with_children[parent_ind].1 = None;
                    }
                } else {
                    i += 1;
                }
            }
        }

        assert!(programs_with_children[0].1.is_none());

        Self {
            root: programs_with_children.remove(0).0,
        }
    }

    fn fixed_weight(&self) -> u32 {
        Self::check_balance(&self.root).1.unwrap()
    }

    fn check_balance(program: &Program) -> (u32, Option<u32>) {
        //! Returns the total weight of the program and the fixed weight if it was found
        //! If fixed weight is present the total weight is not calculated anymore and is therefore incorrect

        if let Some(children) = &program.children {
            let mut weights = Vec::new();
            for child in children {
                let (weight, fixed_weight) = Self::check_balance(child);
                if let Some(fixed_weight) = fixed_weight {
                    return (0, Some(fixed_weight));
                }
                weights.push(weight);
            }

            // if program has 2 children or fewer it can't be unbalanced
            // we wouldn't know which child to balance it with
            if weights.len() > 2 && weights.iter().any(|&w| w != weights[0]) {
                // find the unbalanced weight
                let mut appearances = HashMap::new();
                for weight in &weights {
                    *appearances.entry(weight).or_insert(0) += 1;
                }
                assert_eq!(
                    appearances.len(),
                    2,
                    "There should be only 2 different weights"
                );
                let (&&unbalanced_weight, _) =
                    appearances.iter().find(|&(_, &count)| count == 1).unwrap();
                let (&&balanced_weight, _) =
                    appearances.iter().find(|&(_, &count)| count != 1).unwrap();
                let unbalanced_index = weights
                    .iter()
                    .position(|&w| w == unbalanced_weight)
                    .unwrap();
                let offset = balanced_weight as i32 - unbalanced_weight as i32;

                (
                    0,
                    Some((children[unbalanced_index].weight as i32 + offset) as u32),
                )
            } else {
                (program.weight + weights.iter().sum::<u32>(), None)
            }
        } else {
            (program.weight, None)
        }
    }

    fn root(&self) -> &Program<'a> {
        &self.root
    }
}
