use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::{first_answer, read_input_no_skip, second_answer};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rule {
    name: String,
    range1: (u64, u64),
    range2: (u64, u64),
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<u64>,
    tickets: Vec<Vec<u64>>,
}

pub fn run(day: u8) {
    let input = read_input_no_skip(day);

    // first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let parsed = parse(input);
    let wrong = parsed
        .tickets
        .iter()
        .filter(|t| {
            !t.iter()
                .all(|n| parsed.rules.iter().any(|r| matches_rule(n, r)))
        })
        .flat_map(|t| t)
        .filter(|n| !parsed.rules.iter().any(|r| matches_rule(n, r)))
        .collect::<Vec<&u64>>();

    String::from(format!("{:?}", wrong.into_iter().sum::<u64>()))
}

fn second_problem(input: &Vec<String>) -> String {
    let parsed = parse(input);
    let valid: Vec<&Vec<u64>> = parsed
        .tickets
        .iter()
        .filter(|t| {
            t.iter()
                .all(|n| parsed.rules.iter().any(|r| matches_rule(n, r)))
        })
        .collect();
    let mut possible_rules: Vec<Vec<&Rule>> = parsed
        .my_ticket
        .iter()
        .map(|_| parsed.rules.iter().collect())
        .collect();

    for ticket in valid.into_iter() {
        // println!("\n\nmatching ticket {:?}", ticket);
        for (i, value) in ticket.iter().enumerate() {
            // println!("\nmatching value {:?}", value);
            possible_rules[i] = possible_rules[i]
                .iter()
                .filter(|r| matches_rule(value, r))
                .map(|r| *r)
                .collect()
        }
    }

    let mut recognized: Vec<(Vec<&String>, &u64)> = possible_rules
        .iter()
        .map(|v| v.into_iter().map(|r| &r.name).collect::<Vec<&String>>())
        .zip(parsed.my_ticket.iter())
        .collect();

    recognized.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    let mut used: HashSet<&String> = HashSet::new();
    let recognized: Vec<(&String, &u64)> = recognized
        .into_iter()
        .map(|(v, i)| {
            let curr: Vec<&String> = v.into_iter().filter(|r| !used.contains(*r)).collect();
            used.insert(curr[0]);
            (curr[0], i)
        })
        .filter(|(r, _)| r.contains("departure"))
        .collect();
    let ans = recognized.iter().fold(1, |res, next| res * next.1);

    String::from(format!("{:?}", ans))
}

fn parse(input: &Vec<String>) -> Input {
    let rule_reg = Regex::new(r"(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
    let rules: Vec<Rule> = input
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            if let Some(r) = rule_reg.captures(l) {
                Rule {
                    name: r.get(1).unwrap().as_str().to_string(),
                    range1: (
                        r.get(2).unwrap().as_str().parse().unwrap(),
                        r.get(3).unwrap().as_str().parse().unwrap(),
                    ),
                    range2: (
                        r.get(4).unwrap().as_str().parse().unwrap(),
                        r.get(5).unwrap().as_str().parse().unwrap(),
                    ),
                }
            } else {
                panic!(format!("Unreachable case, error parsing regex {}", l))
            }
        })
        .collect();

    let my_ticket: Vec<u64> = input
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(2)
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets: Vec<Vec<u64>> = input
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(5)
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    Input {
        rules,
        my_ticket,
        tickets,
    }
}

fn matches_rule(n: &u64, r: &Rule) -> bool {
    let res = (r.range1.0..=r.range1.1).contains(n) || (r.range2.0..=r.range2.1).contains(n);

    // if !res {
    //     println!("{} does not match {:?}", n, r)
    // } else {
    //     println!("{} matches {:?}", n, r)
    // }

    res
}
