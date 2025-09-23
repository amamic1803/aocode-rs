use aocode::{AdventOfCode, AoC};
use std::fs;

/// A function that runs a test for a given year, day, part and expected output.
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

    // test library
    let output_lib = AoC::new().solve(year, day, part, &input).unwrap();

    assert_eq!(output_lib.trim(), output.trim());
}
