//! Solutions to the *Advent of Code 2020*

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day08;
mod day09;
mod day17;

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
pub use day08::Day08;
#[doc(inline)]
pub use day09::Day09;
#[doc(inline)]
pub use day17::Day17;

year!(
    Year2020, 2020, Day01, Day02, Day03, Day04, Day05, Day06, Day08, Day09, Day17
);
