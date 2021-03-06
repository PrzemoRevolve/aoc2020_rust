#[macro_use]
extern crate clap;
extern crate lib;

fn main() {
    let day: u32 = clap::App::new("AoC 2020 - Rust")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Attempt at learning practical Rust with 2020th edition of the Advent of Code.")
        .arg_from_usage("<day> 'Runs the appropriate day\'s task'")
        .get_matches()
        .value_of("day")
        .expect("Failed to parse <day> argument!")
        .to_string()
        .parse()
        .expect("Failed to parse <day> argument!");

    match day {
        1 => lib::days::day1::run(),
        2 => lib::days::day2::run(),
        3 => lib::days::day3::run(),
        4 => lib::days::day4::run(),
        5 => lib::days::day5::run(),
        6 => lib::days::day6::run(),
        7 => lib::days::day7::run(),
        8 => lib::days::day8::run(),
        9 => lib::days::day9::run(),
        10 => lib::days::day10::run(),
        11 => lib::days::day11::run(11),
        12 => lib::days::day12::run(12),
        13 => lib::days::day13::run(13),
        14 => lib::days::day14::run(14),
        15 => lib::days::day15::run(15),
        16 => lib::days::day16::run(16),
        17 => lib::days::day17::run(17),
        18 => lib::days::day18::run(18),
        19 => lib::days::day19::run(19),
        20 => lib::days::day20::run(20),
        21 => lib::days::day21::run(21),
        _ => println!("Day {} not found", day),
    }
}
