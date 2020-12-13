use crate::{first_answer, read_input, second_answer};
extern crate num;

pub fn run(day: u8) {
    let input = read_input(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let time: u64 = input[0].parse().unwrap();
    let buses: Vec<u64> = input[1]
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse().unwrap())
        .collect();
    let closest: (u64, &u64) = buses
        .iter()
        .map(|b| {
            let rem = time % b;

            (b - rem, b)
        })
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();

    String::from(format!("{:?}", closest.0 * closest.1))
}

fn second_problem(input: &Vec<String>) -> String {
    let buses: Vec<String> = input[1].split(',').map(|s| s.to_string()).collect();
    let tup: (u128, u128) = buses
        .iter()
        .enumerate()
        .filter_map(|s| match &s.1[..] {
            "x" => None,
            n => {
                let num: u128 = n.parse().unwrap();
                Some((
                    num,
                    if s.0 != 0 {
                        num - ((s.0 as u128) % num)
                    } else {
                        0
                    },
                ))
            }
        })
        .fold((0, 1), |mut t, d| {
            // t.0 is the current "timestamp"
            // t.1 is the product of previous factors
            // d.0 is the divisor
            // d.1 is the remainder
            println!("step {:?} | {:?}", t, d);
            loop {
                t.0 += t.1;

                println!("{} % {} = {}, expected {}", t.0, d.0, t.0 % d.0, d.1);

                if t.0 % d.0 == d.1 {
                    return (t.0, t.1 * d.0);
                }
            }
        });

    String::from(format!("{:?}", tup.0))
}
