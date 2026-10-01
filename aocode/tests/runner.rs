use aocode::{AdventOfCode, AoC};
use std::fs;

/// A function that runs the test for the given year, day and part.
/// # Arguments
/// * `year` - The year of the Advent of Code challenge.
/// * `day` - The day of the Advent of Code challenge.
/// * `part` - The part of the Advent of Code challenge.
/// # Panics
/// * If the input or output files cannot be read.
/// * If the test fails (i.e., the output from the library does not match the expected output).
pub fn run_test(year: usize, day: usize, part: usize) {
    // load input
    let input = fs::read_to_string(format!(
        "./tests/test-data/input/year{year:04}/day{day:02}.txt"
    ))
    .expect("Failed to read the input file!")
    .replace("\r\n", "\n");

    // load output
    let output = fs::read_to_string(format!(
        "./tests/test-data/output/year{year:04}/day{day:02}/part{part:01}.txt"
    ))
    .expect("Failed to read the output file!")
    .replace("\r\n", "\n");

    // test solution in the library
    let output_lib = AoC::new().solve(year, day, part, &input).unwrap();
    assert_eq!(output_lib.trim(), output.trim());
}
