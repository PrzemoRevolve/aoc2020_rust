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
        _ => println!("Day {} not found", day),
    }
}
