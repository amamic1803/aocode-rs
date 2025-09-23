//! [*Advent of Code*](https://adventofcode.com/) problems with solutions.

macro_rules! year {
    ($struct_name:ident, $id:literal, $($day:ident),* ) => {
        #[doc = concat!("[*Advent of Code ", $id, "*](https://adventofcode.com/", $id, ")")]
        pub struct $struct_name {
            id: usize,
            days: Vec<Box<dyn crate::Day>>,
        }
        impl $struct_name {
            #[doc = concat!("Create a new [", stringify!($struct_name), "] instance.")]
            pub fn new() -> Self {
                let mut new_self = Self {
                    id: $id,
                    days: vec![
                        $(Box::new($day::new())),*
                    ],
                };
                new_self.days.sort_unstable_by_key(|day| day.id());
                new_self
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl crate::Year for $struct_name {
            fn id(&self) -> usize {
                self.id
            }
            fn days<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn crate::Day> + 'a> {
                Box::new(self.days.iter().map(|day| day.as_ref()))
            }
        }
    };
}

macro_rules! day {
    ($struct_name:ident, $year_id:literal, $day_id:literal, $title:literal) => {
        #[doc = concat!("[*", $title, "*](https://adventofcode.com/", $year_id, "/day/", $day_id, ")")]
        #[derive(Copy, Clone)]
        pub struct $struct_name {
            id: usize,
            title: &'static str,
        }
        impl $struct_name {
            #[doc = concat!("Create a new [", stringify!($struct_name), "] instance.")]
            pub fn new() -> Self {
                Self {
                    id: $day_id,
                    title: $title,
                }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl crate::Day for $struct_name {
            fn id(&self) -> usize {
                self.id
            }
            fn title(&self) -> &str {
                self.title
            }
        }
    };
}

pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;
pub mod year2024;

#[doc(inline)]
pub use year2015::Year2015;
#[doc(inline)]
pub use year2016::Year2016;
#[doc(inline)]
pub use year2017::Year2017;
#[doc(inline)]
pub use year2018::Year2018;
#[doc(inline)]
pub use year2019::Year2019;
#[doc(inline)]
pub use year2020::Year2020;
#[doc(inline)]
pub use year2021::Year2021;
#[doc(inline)]
pub use year2022::Year2022;
#[doc(inline)]
pub use year2023::Year2023;
#[doc(inline)]
pub use year2024::Year2024;
