//! Solutions to the *Advent of Code 2019*

mod day01;
mod day02;
mod day03;
mod day04;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day02::Day02;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day04::Day04;

year!(Year2019, 2019, Day01, Day02, Day03, Day04);
