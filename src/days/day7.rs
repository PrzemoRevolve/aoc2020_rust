use std::collections::HashMap;

use crate::{first_answer, read_input, second_answer};

pub fn run() {
    let input = read_input(7);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let query = "shinygold";
    let rules = get_rules(input);
    let ans: Vec<_> = rules
        .keys()
        .filter(|a| can_hold(&rules, a, query))
        .collect();

    String::from(format!("{:?}", ans.len()))
}

fn second_problem(input: &Vec<String>) -> String {
    let query = "shinygold";
    let rules = get_rules(input);
    let ans = sum_children_recursive(&rules, query);
    String::from(format!("{}", ans))
}

fn get_rules(input: &Vec<String>) -> HashMap<String, Vec<(i32, String)>> {
    input
        .iter()
        .map(|l| {
            let no_punctuation = l.replace(",", "").replace(".", "");
            let mut iter = no_punctuation.split_whitespace();
            let key = iter.by_ref().take(2).collect::<Vec<&str>>().join("");

            let _ = iter.next(); // "bags"

            let mut bags = vec![];

            loop {
                let _ = iter.next(); // "contain / bag/bags"
                let next = iter.nth(0);

                if let None = next {
                    break;
                }

                if next.unwrap() == "no" {
                    break; // "no other bags" case
                }

                let count: i32 = next.unwrap().parse().unwrap();
                let name = iter.by_ref().take(2).collect::<Vec<&str>>().join("");

                bags.push((count, name));
            }

            (key, bags)
        })
        .collect()
}

fn can_hold(map: &HashMap<String, Vec<(i32, String)>>, current: &String, query: &str) -> bool {
    let bags = map.get(current);

    if let None = bags {
        false
    } else if bags.unwrap().len() == 0 {
        false
    } else if bags.unwrap().iter().any(|b| &b.1 == query) {
        true
    } else {
        bags.unwrap()
            .into_iter()
            .any(|bag| can_hold(map, &bag.1, query))
    }
}

fn sum_children_recursive(map: &HashMap<String, Vec<(i32, String)>>, query: &str) -> i32 {
    let bags = map.get(query);

    if let None = bags {
        0
    } else if bags.unwrap().len() == 0 {
        0
    } else {
        bags.unwrap()
            .into_iter()
            .map(|bag| bag.0 + bag.0 * sum_children_recursive(map, &bag.1))
            .sum()
    }
}
