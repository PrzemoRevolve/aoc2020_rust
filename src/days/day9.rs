use crate::{first_answer, read_input, second_answer};

pub fn run() {
    let input = read_input(9);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let numbers = parse(&input);
    let ans = find_bug(&numbers);
    String::from(format!("{:?}", ans))
}

fn second_problem(input: &Vec<String>) -> String {
    let numbers = parse(&input);
    let sum = find_bug(&numbers);
    let ans = find_weakness(&numbers, sum);
    String::from(format!("{:?}", ans))
}

fn find_bug(input: &Vec<i64>) -> i64 {
    let mut index = 25;

    loop {
        if index >= input.len() {
            panic!("Didn't find any results");
        }

        let window = &input[index - 25..index];
        let number = input[index];

        let ans = find_sum(window, number);

        if !ans {
            return number;
        } else {
            index += 1;
        }
    }
}

fn parse(input: &Vec<String>) -> Vec<i64> {
    input.iter().map(|l| l.parse().unwrap()).collect()
}

fn find_sum(window: &[i64], number: i64) -> bool {
    for n in window {
        for m in window {
            if n + m == number {
                return true;
            }
        }
    }

    false
}

fn find_weakness(numbers: &Vec<i64>, sum: i64) -> i64 {
    for (pos, start) in numbers.iter().enumerate() {
        let mut list = vec![];
        let mut rolling_sum = 0;

        for num in numbers.iter().skip(pos) {
            list.push(*num);
            rolling_sum += num;

            if rolling_sum == sum {
                return list.iter().min().unwrap() + list.iter().max().unwrap();
            } else if rolling_sum > sum {
                break;
            }
        }

        if start >= &sum {
            panic!("Didn't find any solution");
        }
    }

    panic!("Didn't find any solution");
}
