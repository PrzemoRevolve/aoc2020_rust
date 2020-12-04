use crate::{first_answer, read_input, second_answer};

pub fn run() {
    first_answer(&first_problem());
    second_answer(&second_problem());
}

fn first_problem() -> String {
    let input = read_input(2);
    let mut valid_count = 0;

    for line in input.iter() {
        let mut split = line.split(' ');
        let range = split.next().unwrap();
        let rule = split.next().unwrap().chars().nth(0).unwrap();
        let text = split.next().unwrap();

        let mut minmax = range.split('-');
        let min: i32 = minmax.next().unwrap().parse().unwrap();
        let max: i32 = minmax.next().unwrap().parse().unwrap();

        let mut count = 0;

        for c in text.chars().into_iter() {
            if c == rule {
                count += 1;
            }
        }

        if count <= max && count >= min {
            valid_count += 1
        }
    }

    String::from(format!("{}", valid_count))
}

fn second_problem() -> String {
    let input = read_input(2);
    let mut valid_count = 0;

    for line in input.iter() {
        let mut split = line.split(' ');
        let range = split.next().unwrap();
        let rule = split.next().unwrap().chars().nth(0).unwrap();
        let text = split.next().unwrap();

        let mut positions = range.split('-');
        let pos1: usize = positions.next().unwrap().parse().unwrap();
        let pos2: usize = positions.next().unwrap().parse().unwrap();

        let mut iter = text.chars().into_iter();
        let first = iter.nth(pos1 - 1);
        let second = iter.nth(pos2 - pos1 - 1);

        let mut first_matches = false;
        let mut second_matches = false;

        if let Some(v) = first {
            first_matches = v == rule;
        }

        if let Some(v) = second {
            second_matches = v == rule;
        }

        if (first_matches && !second_matches) || (!first_matches && second_matches) {
            valid_count += 1
        }
    }

    String::from(format!("{}", valid_count))
}
