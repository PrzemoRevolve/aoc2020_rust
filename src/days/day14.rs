use std::collections::HashMap;

use regex::Regex;

use crate::{first_answer, read_input, second_answer};
extern crate num;

enum Instruction {
    SetMask(String),
    SetMem((String, String)),
}

pub fn run(day: u8) {
    let input = read_input(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let mut values: HashMap<String, u64> = HashMap::new();
    let instructions = parse_input(input);
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for ins in instructions {
        match ins {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::SetMem((i, v)) => {
                let v2 = apply_mask_to_value(v, &mask);
                let val = bin_to_int(v2);
                values.insert(i, val);
            }
        }
    }

    String::from(format!("{:?}", values.values().sum::<u64>()))
}

fn second_problem(input: &Vec<String>) -> String {
    let mut values: HashMap<String, u64> = HashMap::new();
    let instructions = parse_input(input);
    let mut mask = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

    for ins in instructions {
        match ins {
            Instruction::SetMask(new_mask) => mask = new_mask,
            Instruction::SetMem((i, v)) => {
                let addrs = apply_mask_to_address(i, &mask);
                let val = bin_to_int(v);

                for addr in addrs {
                    values.insert(addr, val);
                }
            }
        }
    }

    String::from(format!("{:?}", values.values().sum::<u64>()))
}

fn apply_mask_to_value(val: String, mask: &String) -> String {
    val.chars()
        .zip(mask.chars())
        .map(|(c, mc)| match mc {
            '0' => '0',
            '1' => '1',
            _ => c,
        })
        .collect()
}

fn apply_mask_to_address(address: String, mask: &String) -> Vec<String> {
    let new_address: String = address
        .chars()
        .zip(mask.chars())
        .map(|(c, mc)| match mc {
            '0' => c,
            '1' => '1',
            _ => 'X',
        })
        .collect();
    let mut addresses = vec![new_address];

    loop {
        if addresses.iter().any(|a| a.contains('X')) {
            addresses = replace_x(addresses);
        } else {
            return addresses;
        }
    }
}

fn replace_x(arr: Vec<String>) -> Vec<String> {
    arr.into_iter()
        .flat_map(|s| {
            if s.contains('X') {
                vec![s.replacen("X", "0", 1), s.replacen("X", "1", 1)]
            } else {
                vec![s]
            }
        })
        .collect()
}

fn bin_to_int(val: String) -> u64 {
    u64::from_str_radix(&val[..], 2).unwrap()
}

fn int_to_bin(val: u64) -> String {
    format!("{:0>36b}", val)
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {
    let set_mem = Regex::new(r"^mem\[(.+)\] = (.+)$").unwrap();
    let set_mask = Regex::new(r"^mask = (.+)$").unwrap();

    input
        .iter()
        .map(|l| {
            if let Some(mem) = set_mem.captures(l) {
                Instruction::SetMem((
                    int_to_bin(mem.get(1).unwrap().as_str().parse::<u64>().unwrap()),
                    int_to_bin(mem.get(2).unwrap().as_str().parse::<u64>().unwrap()),
                ))
            } else if let Some(mask) = set_mask.captures(l) {
                Instruction::SetMask(mask.get(1).unwrap().as_str().to_owned())
            } else {
                panic!("Unreachable case!")
            }
        })
        .collect()
}
