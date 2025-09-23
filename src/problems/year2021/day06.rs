use crate::{Error, Solution};

day!(Day06, 2021, 6, "Lanternfish");

impl Solution for Day06 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let mut lanternfish = Lanternfish::new(input);
        for _ in 0..DAYS_1 {
            lanternfish.simulate_day();
        }
        Ok(lanternfish.fish_count().to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let mut lanternfish = Lanternfish::new(input);
        for _ in 0..DAYS_2 {
            lanternfish.simulate_day();
        }
        Ok(lanternfish.fish_count().to_string())
    }
}

const DAYS_1: u16 = 80;
const DAYS_2: u16 = 256;

struct Lanternfish {
    /// The number of lanternfish in each of the 9 possible states.
    fish_counts: [u64; 9],
}
impl Lanternfish {
    fn new(input: &str) -> Self {
        let mut fish_counts = [0; 9];
        for fish_state in input
            .trim()
            .split(',')
            .map(|num_str| num_str.parse::<usize>().unwrap())
        {
            fish_counts[fish_state] += 1;
        }
        Self { fish_counts }
    }

    fn fish_count(&self) -> u64 {
        self.fish_counts.iter().sum()
    }

    fn simulate_day(&mut self) {
        // by shifting the array to left by one, cycle for each lanternfish is reduced by 1
        // lanternfish that were in state 0 before shift should go to state 6 and produce 1 new lanternfish in state 8
        // by shifting the array to left by one, the lanternfish in state 0 are now in state 8
        // so the state 8 is satisfied, now we just have to add the number of original lanternfish that transition into state 6
        // that is exactly the number currently stored in state 8
        self.fish_counts.rotate_left(1);
        self.fish_counts[6] += self.fish_counts[8];
    }
}
