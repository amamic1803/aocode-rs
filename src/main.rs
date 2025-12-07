use aocode::{AdventOfCode, AoC};
use clap::{Arg, ArgAction, command, value_parser};
use std::io::{Read, stdin};
use std::process::ExitCode;

fn main() -> ExitCode {
    let argv = command!()
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("The year of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(2015..)),
        )
        .arg(
            Arg::new("day")
                .value_name("DAY")
                .help("The day of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(1..=25)),
        )
        .arg(
            Arg::new("part")
                .value_name("PART")
                .help("The part of the Advent of Code challenge")
                .required_unless_present("list")
                .value_parser(value_parser!(u32).range(1..=2)),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List all available Advent of Code challenges")
                .conflicts_with("benchmark")
                .required(false),
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .action(ArgAction::SetTrue)
                .help("Measure the time taken to solve the challenge")
                .conflicts_with("list")
                .required(false),
        )
        .get_matches();

    let year_num = argv.get_one::<u32>("year");
    let day_num = argv.get_one::<u32>("day");
    let part_num = argv.get_one::<u32>("part");
    let list_flag: bool = argv.get_flag("list");
    let benchmark_flag: bool = argv.get_flag("benchmark");

    let advent_of_code = AoC::new();

    if list_flag {
        println!("{}", advent_of_code);
    } else {
        // unwrap the year, day, and part numbers
        // clap will make sure that they are present here
        let year_num = *year_num.unwrap() as usize;
        let day_num = *day_num.unwrap() as usize;
        let part_num = *part_num.unwrap() as usize;

        let mut input = String::new();
        if let Err(err) = stdin().read_to_string(&mut input) {
            eprintln!("Error reading the input: {}", err);
            return ExitCode::FAILURE;
        }

        if !benchmark_flag {
            match advent_of_code.run(year_num, day_num, part_num, &input) {
                Ok(result) => println!("{}", result),
                Err(err) => {
                    eprintln!("Error running the challenge: {}", err);
                    return ExitCode::FAILURE;
                }
            }
        } else {
            match advent_of_code.benchmark(year_num, day_num, part_num, &input) {
                Ok((result, duration)) => {
                    println!("{}    --- {} s", result, duration.as_secs_f64())
                }
                Err(err) => {
                    eprintln!("Error running the challenge: {}", err);
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    ExitCode::SUCCESS
}
