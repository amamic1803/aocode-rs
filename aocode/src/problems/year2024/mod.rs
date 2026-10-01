//! Solutions to the *Advent of Code 2024*

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day09;
mod day14;
mod day18;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day02::Day02;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day04::Day04;
#[doc(inline)]
pub use day05::Day05;
#[doc(inline)]
pub use day06::Day06;
#[doc(inline)]
pub use day07::Day07;
#[doc(inline)]
pub use day09::Day09;
#[doc(inline)]
pub use day14::Day14;
#[doc(inline)]
pub use day18::Day18;

year!(
    Year2024, 2024, Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day09, Day14, Day18
);
