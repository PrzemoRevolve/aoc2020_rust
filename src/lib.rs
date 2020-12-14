pub mod days;

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Loads the input from the sources directory. Files have to be in
/// /input/day-12-2.txt for day 12 problem 2 (and the same for others).
fn read_input(day: u8) -> Vec<String> {
    read_optional_skip(day, true)
}

fn read_input_no_skip(day: u8) -> Vec<String> {
    read_optional_skip(day, false)
}

fn read_optional_skip(day: u8, skip_empty: bool) -> Vec<String> {
    let filename = format!("input/day-{}.txt", day);

    let file = File::open(&filename).expect(&format!("Unable to open input file in {}", filename));

    BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect(format!("Unable to read line in {filename}", filename = filename).as_str())
        })
        .filter(|l| if skip_empty { !l.is_empty() } else { true })
        .collect()
}

fn first_answer(answer: &str) {
    print_answer("First", answer);
}

fn second_answer(answer: &str) {
    print_answer("Second", answer);
}

fn print_answer(prefix: &str, answer: &str) {
    println!("{} answer is {}", prefix, answer);
}
