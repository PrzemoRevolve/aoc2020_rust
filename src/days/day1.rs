use crate::{first_answer, read_input, second_answer};

pub fn run() {
    first_answer(&first_problem());
    second_answer(&second_problem());
}

fn first_problem() -> String {
    let input = read_input(1);
    let numbers: Vec<u32> = input.iter().map(|value| value.parse().unwrap()).collect();

    for number in numbers.iter() {
        for number2 in numbers.iter() {
            if number + number2 == 2020 {
                return String::from(format!("{} * {} = {}", number, number2, number * number2));
            }
        }
    }

    String::from("ERROR")
}

fn second_problem() -> String {
    let input = read_input(1);
    let numbers: Vec<u32> = input.iter().map(|value| value.parse().unwrap()).collect();

    for number in numbers.iter() {
        for number2 in numbers.iter() {
            for number3 in numbers.iter() {
                if number + number2 + number3 == 2020 {
                    return String::from(format!(
                        "{} * {} * {} = {}",
                        number,
                        number2,
                        number3,
                        number * number2 * number3
                    ));
                }
            }
        }
    }

    String::from("ERROR")
}
