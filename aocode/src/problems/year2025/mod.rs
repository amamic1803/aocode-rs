//! Solutions to the *Advent of Code 2025*

mod day01;
mod day03;
mod day04;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day04::Day04;

year!(Year2025, 2025, Day01, Day03, Day04);
