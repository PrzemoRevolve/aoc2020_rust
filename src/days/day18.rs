use regex::Regex;

use crate::{first_answer, read_input_no_skip, second_answer};

enum Token {
    Operation(Operation),
    Number(u128),
}

enum Operation {
    Add,
    Mul,
}

type Expression = Vec<Token>;

pub fn run(day: u8) {
    let input = read_input_no_skip(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let ans: u128 = input
        .into_iter()
        .map(|l| reduce_line(l.to_owned()))
        .map(|s| calculate_expression(parse_expression(&s)))
        .sum();

    String::from(format!("{:?}", ans))
}

fn second_problem(input: &Vec<String>) -> String {
    let ans: u128 = input
        .into_iter()
        .map(|l| reduce_line2(l.to_owned()))
        .map(|s| calculate_expression(parse_expression(&s)))
        .sum();

    String::from(format!("{:?}", ans))
}

fn wrap_addition(l: String) -> String {
    let wrap_regex = Regex::new(r"(\* )([0-9]+ \+ [0-9]+(?: \+ [0-9]+)*)").unwrap();
    let r = wrap_regex.replace_all(&l, r"$1($2)");
    // println!("Wraps {} as {}", l, r.to_string());
    r.to_string()
}

fn reduce_line2(line: String) -> String {
    let mut l = wrap_addition(line);

    loop {
        if let Some(x) = replace_paren(&l) {
            // println!("Replaces {} with {}", l, x);
            l = wrap_addition(x);
        } else {
            return l;
        }
    }
}

fn reduce_line(line: String) -> String {
    let mut l = line;

    loop {
        if let Some(x) = replace_paren(&l) {
            // println!("Replaces {} with {}", l, x);
            l = x;
        } else {
            return l;
        }
    }
}

fn parse_expression(exp: &str) -> Expression {
    let token_regex = Regex::new(r"([0-9]+|\(|\)|\+|\*)").unwrap();

    token_regex
        .captures_iter(exp)
        .map(|c| match c.get(1).unwrap().as_str() {
            "+" => Token::Operation(Operation::Add),
            "*" => Token::Operation(Operation::Mul),
            n => Token::Number(
                n.parse::<u128>()
                    .unwrap_or_else(|_| panic!(format!("Err parse on {}", n))),
            ),
        })
        .collect()
}

fn replace_paren(line: &String) -> Option<String> {
    let paren_regex = Regex::new(r"\(([0-9\s\+\*]+)\)").unwrap();

    let s = paren_regex.captures(line)?.get(1)?.as_str();
    let exp = parse_expression(s);

    let val = calculate_expression(exp);

    let res = paren_regex.replace(line, &val.to_string()[..]);

    Some(res.to_string())
}

fn execute_operation(num1: &u128, op: &Operation, num2: &u128) -> u128 {
    match op {
        Operation::Add => num1 + num2,
        Operation::Mul => num1 * num2,
    }
}

fn calculate_expression(exp: Expression) -> u128 {
    let mut val = 0;
    let mut op = Operation::Add;

    for t in exp {
        match t {
            Token::Number(n) => val = execute_operation(&val, &op, &n),
            Token::Operation(o) => op = o,
        }
    }

    val
}
