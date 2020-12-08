use std::collections::HashSet;

use crate::{first_answer, read_input, second_answer};
use regex::Regex;

pub fn run() {
    let input = read_input(8);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let ans = run_instructions(&parse(input));
    String::from(format!("{:?}", ans))
}

fn second_problem(input: &Vec<String>) -> String {
    let instructions = parse(input);
    let ans = try_fix_bug(&instructions);

    String::from(format!("{:?}", ans))
}

fn parse(input: &Vec<String>) -> Vec<(String, i32)> {
    let reg = Regex::new(r"^([a-z]{3}) \+?(-?[0-9]+)$").unwrap();

    input
        .iter()
        .map(|l| {
            let res = reg.captures(l).unwrap();

            (
                String::from(res.get(1).unwrap().as_str()),
                res.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn run_instructions(instructions: &Vec<(String, i32)>) -> (i32, i32) {
    let mut acc = 0;
    let mut i: i32 = 0;
    let mut set = HashSet::new();

    loop {
        if i >= instructions.len() as i32 {
            return (i, acc);
        }

        let instruction = &instructions[i as usize];

        if set.contains(&i) {
            break;
        }

        set.insert(i);

        match &instruction.0[..] {
            "nop" => i += 1,
            "jmp" => i += instruction.1,
            "acc" => {
                acc += instruction.1;
                i += 1;
            }
            _ => panic!(format!(
                "Unrecognized instruction {}",
                &instructions[i as usize].0
            )),
        }
    }

    (i, acc)
}

fn try_fix_bug(instructions: &Vec<(String, i32)>) -> (i32, i32) {
    try_replace(instructions, "jmp", "nop")
        .unwrap_or_else(|| try_replace(instructions, "nop", "jmp").unwrap())
}

fn try_replace(
    instructions: &Vec<(String, i32)>,
    what: &str,
    with_what: &str,
) -> Option<(i32, i32)> {
    let mut ans;

    for (position, _) in instructions.iter().enumerate().filter(|l| l.1 .0 == what) {
        let mut cloned = instructions.clone();
        cloned[position].0 = with_what.to_string();

        ans = run_instructions(&cloned);

        if ans.0 >= instructions.len() as i32 {
            return Some(ans);
        }
    }

    None
}
