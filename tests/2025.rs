mod runner;
use runner::run_test;

#[test]
fn year2025_day01_part1() {
    run_test(2025, 1, 1);
}

#[test]
fn year2025_day01_part2() {
    run_test(2025, 1, 2);
}
