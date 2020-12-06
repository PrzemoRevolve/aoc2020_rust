use crate::{first_answer, read_input_no_skip, second_answer};
use itertools::Itertools;

pub fn run() {
    let input = read_input_no_skip(6);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let groups = get_groups(input);
    let answer: usize = groups.iter().map(|g| get_unique_chars(g).len()).sum();

    String::from(format!("{}", answer))
}

fn second_problem(input: &Vec<String>) -> String {
    let groups = get_groups(input);
    let answer: usize = groups.iter().map(|g| get_intersecting_chars(g).len()).sum();

    String::from(format!("{}", answer))
}

fn get_groups(input: &Vec<String>) -> Vec<Vec<&String>> {
    input.iter().fold(vec![vec![]], |mut acc, line| {
        let last = acc.len() - 1;

        if line.is_empty() {
            acc.push(vec![]);
        } else {
            acc.get_mut(last).unwrap().push(line);
        }

        acc
    })
}

fn get_unique_chars(group: &Vec<&String>) -> Vec<char> {
    group
        .iter()
        .flat_map(|v| v.chars())
        .unique()
        .sorted()
        .collect()
}

fn get_intersecting_chars(group: &Vec<&String>) -> Vec<char> {
    get_unique_chars(group)
        .into_iter()
        .filter(|c| group.iter().all(|s| s.contains(*c)))
        .collect()
}
