//! Solutions to the *Advent of Code 2018*

mod day01;
mod day02;
mod day03;
mod day05;
mod day06;
mod day08;
mod day12;

#[doc(inline)]
pub use day01::Day01;
#[doc(inline)]
pub use day02::Day02;
#[doc(inline)]
pub use day03::Day03;
#[doc(inline)]
pub use day05::Day05;
#[doc(inline)]
pub use day06::Day06;
#[doc(inline)]
pub use day08::Day08;
#[doc(inline)]
pub use day12::Day12;

year!(
    Year2018, 2018, Day01, Day02, Day03, Day05, Day06, Day08, Day12
);
