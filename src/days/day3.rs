use crate::{first_answer, read_input, second_answer};

pub fn run() {
    let input = read_input(3);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let mut trees = 0;

    for (index, row) in input.iter().enumerate() {
        if is_tree(index * 3, row) {
            trees += 1
        }
    }

    String::from(format!("{}", trees))
}

fn second_problem(input: &Vec<String>) -> String {
    String::from(format!(
        "1x1 :{}, 3x1 :{}, 5x1 :{}, 7x1 :{}, 1x2 :{}, X: {}",
        slope(input, 1, 1),
        slope(input, 3, 1),
        slope(input, 5, 1),
        slope(input, 7, 1),
        slope(input, 1, 2),
        slope(input, 1, 1)
            * slope(input, 3, 1)
            * slope(input, 5, 1)
            * slope(input, 7, 1)
            * slope(input, 1, 2),
    ))
}

fn is_tree(index: usize, row: &String) -> bool {
    let i = index % row.len();

    row.chars().nth(i).unwrap() == '#'
}

fn slope(input: &Vec<String>, right: usize, down: usize) -> i32 {
    let mut trees = 0;

    for (index, row) in input.iter().enumerate() {
        if index % down == 0 && is_tree(index * right / down, row) {
            trees += 1
        }
    }

    trees
}
