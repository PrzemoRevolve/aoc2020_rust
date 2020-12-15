use std::collections::HashMap;

use crate::{first_answer, read_input, second_answer};
extern crate num;

pub fn run(day: u8) {
    let input = read_input(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let input = parse(input);

    String::from(format!("{:?}", run_for(input, 2020)))
}

fn second_problem(input: &Vec<String>) -> String {
    let input = parse(input);

    String::from(format!("{:?}", run_for(input, 30_000_000)))
}

fn parse(input: &Vec<String>) -> Vec<usize> {
    input
        .iter()
        .flat_map(|l| l.split(',').map(|s| s.parse().unwrap()))
        .rev()
        .collect()
}

fn run_for(nums: Vec<usize>, limit: usize) -> usize {
    let mut last = nums[0];
    // <the_number, which turn>
    let mut map: HashMap<usize, usize> = nums
        .into_iter()
        .skip(1)
        .rev()
        .enumerate()
        .map(|(a, b)| (b, a + 1))
        .collect();

    for t in (map.len() + 2)..(limit + 1) {
        let prev_turn = map.get(&last);

        let new = match prev_turn {
            Some(n) => t - 1 - n,
            None => 0,
        };

        if t % 100_000 == 0 {
            println!("Turn {}", t);
        }

        map.insert(last, t - 1);
        last = new;
    }

    last
}
