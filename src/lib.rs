#![doc = include_str!("../README.md")]

use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

pub mod graph;
pub mod math;
pub mod problems;

/// An enum representing the errors that can occur in this crate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// The requested year is unavailable.
    UnavailableYear,
    /// The requested day's challenge is unavailable.
    UnavailableDay,
    /// The requested part is unavailable.
    UnavailablePart,
    /// There is no solution for the challenge with the given input.
    NoSolution,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnavailableYear => write!(f, "The requested year is unavailable."),
            Self::UnavailableDay => write!(f, "The requested day's challenge is unavailable."),
            Self::UnavailablePart => write!(f, "The requested part is unavailable."),
            Self::NoSolution => write!(
                f,
                "There is no solution for the challenge with the given input."
            ),
        }
    }
}
impl StdError for Error {}

/// A trait representing the [*Advent of Code*](https://adventofcode.com/).
pub trait AdventOfCode: Send + Sync {
    /// Get all available years.
    /// # Returns
    /// * An iterator over the years, sorted by their identifier in ascending order.
    fn years<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Year> + 'a>;

    /// Get the year with the specified identifier.
    /// # Arguments
    /// * `year` - The identifier of the year to retrieve.
    /// # Returns
    /// A reference to the specified year or the [Error].
    /// # Errors
    /// * [Error::UnavailableYear] - The requested year is unavailable.
    fn year(&self, year: usize) -> Result<&dyn Year, Error> {
        for iter_year in self.years() {
            match iter_year.id().cmp(&year) {
                Ordering::Less => {}
                Ordering::Equal => return Ok(iter_year),
                Ordering::Greater => break,
            }
        }
        Err(Error::UnavailableYear)
    }

    /// Solve a part of the specified day's challenge from the specified year.
    /// # Arguments
    /// * `year` - The identifier of the year of the challenge to solve.
    /// * `day` - The identifier of the day's challenge to solve.
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailableYear] - The requested year is unavailable.
    /// * [Error::UnavailableDay] - The requested day's challenge is unavailable.
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn solve(&self, year: usize, day: usize, part: usize, input: &str) -> Result<String, Error> {
        self.year(year)?.solve(day, part, input)
    }

    /// Solve a part of the specified day's challenge from the specified year and measure the elapsed time.
    /// # Arguments
    /// * `year` - The identifier of the year of the day's challenge to solve.
    /// * `day` - The identifier of the day's challenge to solve.
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge and the elapsed time or the [Error].
    /// # Errors
    /// * [Error::UnavailableYear] - The requested year is unavailable.
    /// * [Error::UnavailableDay] - The requested day's challenge is unavailable.
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn benchmark(
        &self,
        year: usize,
        day: usize,
        part: usize,
        input: &str,
    ) -> Result<(String, Duration), Error> {
        self.year(year)?.benchmark(day, part, input)
    }
}

/// A trait representing the [*Advent of Code*](https://adventofcode.com/) year.
pub trait Year: Send + Sync {
    /// The identifier of the year.
    /// # Returns
    /// * An integer representing the year.
    fn id(&self) -> usize;

    /// Get all available days' challenges in the year.
    /// # Returns
    /// * An iterator over the days' challenges in the year,
    ///   sorted by their identifier in ascending order.
    fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Day> + 'a>;

    /// Get the day's challenge with the specified identifier.
    /// # Arguments
    /// * `day` - The identifier of the day's challenge to retrieve.
    /// # Returns
    /// * A reference to the specified day's challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailableDay] - The requested day's challenge is unavailable.
    fn day(&self, day: usize) -> Result<&dyn Day, Error> {
        for iter_day in self.days() {
            match iter_day.id().cmp(&day) {
                Ordering::Less => {}
                Ordering::Equal => return Ok(iter_day),
                Ordering::Greater => break,
            }
        }
        Err(Error::UnavailableDay)
    }

    /// Solve a part of the specified day's challenge.
    /// # Arguments
    /// * `day` - The identifier of the day's challenge to solve.
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailableDay] - The requested day's challenge is unavailable.
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn solve(&self, day: usize, part: usize, input: &str) -> Result<String, Error> {
        self.day(day)?.solve(part, input)
    }

    /// Solve a part of the specified day's challenge and measure the elapsed time.
    /// # Arguments
    /// * `day` - The identifier of the day's challenge to solve.
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge and the elapsed time or the [Error].
    /// # Errors
    /// * [Error::UnavailableDay] - The requested day's challenge is unavailable.
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn benchmark(&self, day: usize, part: usize, input: &str) -> Result<(String, Duration), Error> {
        self.day(day)?.benchmark(part, input)
    }
}

/// A trait representing the [*Advent of Code*](https://adventofcode.com/) day's challenge.
pub trait Day: Send + Sync + Solution {
    /// The identifier of the day within the year.
    /// # Returns
    /// * An integer between `1` and `25` (inclusive).
    fn id(&self) -> usize;

    /// The title of the day's challenge.
    /// # Returns
    /// * The title of the day's challenge.
    fn title(&self) -> &str;
}

/// A trait representing the [*Advent of Code*](https://adventofcode.com/) day's challenge solution.
pub trait Solution: Send + Sync {
    /// Solve a part of the day's challenge.
    /// # Arguments
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn solve(&self, part: usize, input: &str) -> Result<String, Error> {
        match part {
            1 => self.part1(input),
            2 => self.part2(input),
            _ => Err(Error::UnavailablePart),
        }
    }

    /// Solve a part of the day's challenge and measure the elapsed time.
    /// # Arguments
    /// * `part` - The part of the challenge to solve.
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the part of the challenge and the elapsed time or the [Error].
    /// # Errors
    /// * [Error::UnavailablePart] - The requested part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn benchmark(&self, part: usize, input: &str) -> Result<(String, Duration), Error> {
        let result;
        let elapsed;

        #[cfg(not(all(
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        )))]
        {
            let instant = std::time::Instant::now();
            result = self.solve(part, input);
            elapsed = instant.elapsed();
        }
        #[cfg(all(
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        ))]
        {
            use wasm_bindgen::prelude::*;

            let global_obj = js_sys::global();
            let performance = js_sys::Reflect::get(&global_obj, &JsValue::from_str("performance"))
                .unwrap()
                .dyn_into::<web_sys::Performance>()
                .unwrap();

            let instant = performance.now();
            result = self.solve(part, input);
            elapsed = Duration::from_secs_f64((performance.now() - instant) / 1000.0);
        }

        result.map(|val| (val, elapsed))
    }

    /// Solve the first part of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the first part of the challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailablePart] - The solution for the first part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn part1(&self, input: &str) -> Result<String, Error> {
        let _ = input; // suppress unused variable warning
        Err(Error::UnavailablePart)
    }

    /// Solve the second part of the day's challenge.
    /// # Arguments
    /// * `input` - The input to the challenge.
    /// # Returns
    /// * The solution to the second part of the challenge or the [Error].
    /// # Errors
    /// * [Error::UnavailablePart] - The solution to the second part is unavailable.
    /// * [Error::NoSolution] - There is no solution for the challenge with the given input.
    fn part2(&self, input: &str) -> Result<String, Error> {
        let _ = input; // suppress unused variable warning
        Err(Error::UnavailablePart)
    }
}

/// A structure representing the [*Advent of Code*](https://adventofcode.com/).
pub struct AoC {
    years: Vec<Box<dyn Year>>,
}
impl AoC {
    /// Create a new [AoC] instance.
    /// # Returns
    /// * The new [AoC] instance with all available years initialized.
    pub fn new() -> Self {
        let mut new_obj = Self {
            years: vec![
                Box::new(problems::Year2015::new()),
                Box::new(problems::Year2016::new()),
                Box::new(problems::Year2017::new()),
                Box::new(problems::Year2018::new()),
                Box::new(problems::Year2019::new()),
                Box::new(problems::Year2020::new()),
                Box::new(problems::Year2021::new()),
                Box::new(problems::Year2022::new()),
                Box::new(problems::Year2023::new()),
                Box::new(problems::Year2024::new()),
                Box::new(problems::Year2025::new()),
            ],
        };
        new_obj.years.sort_unstable_by_key(|year| year.id());
        new_obj
    }
}
impl Default for AoC {
    fn default() -> Self {
        Self::new()
    }
}
impl AdventOfCode for AoC {
    fn years<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn Year> + 'a> {
        Box::new(self.years.iter().map(|year| year.as_ref()))
    }
}
