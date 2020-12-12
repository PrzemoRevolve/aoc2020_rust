use crate::{first_answer, read_input, second_answer};

pub fn run(day: u8) {
    let input = read_input(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let ins = parse(input);
    let mut start = (0, 0); // North, West
    let mut curr = 'E';

    for (i, l) in ins {
        match i {
            'N' => start.0 += l,
            'S' => start.0 -= l,
            'W' => start.1 += l,
            'E' => start.1 -= l,
            'L' => {
                for _ in 0..(l / 90) {
                    curr = turn_left(curr)
                }
            }
            'R' => {
                for _ in 0..(l / 90) {
                    curr = turn_right(curr)
                }
            }
            'F' => match curr {
                'N' => start.0 += l,
                'S' => start.0 -= l,
                'W' => start.1 += l,
                'E' => start.1 -= l,
                _ => panic!(format!("Unrecognized instruction {:?}", (i, l))),
            },
            _ => panic!(format!("Unrecognized instruction {:?}", (i, l))),
        }
        // println!(
        //     "Instr: {:?}, curr pos: {:?}, facing: {}",
        //     (i, l),
        //     start,
        //     curr
        // )
    }

    String::from(format!("{:?}", start.0.abs() + start.1.abs()))
}

fn second_problem(input: &Vec<String>) -> String {
    let ins = parse(input);

    let mut way = (1, -10);
    let mut start = (0, 0);

    for (i, l) in ins {
        match i {
            'N' => way.0 += l,
            'S' => way.0 -= l,
            'W' => way.1 += l,
            'E' => way.1 -= l,
            'L' => {
                for _ in 0..(l / 90) {
                    way = rotate_left(way)
                }
            }
            'R' => {
                for _ in 0..(l / 90) {
                    way = rotate_right(way)
                }
            }
            'F' => {
                for _ in 0..l {
                    start.0 += way.0;
                    start.1 += way.1;
                }
            }
            _ => panic!(format!("Unrecognized instruction {:?}", (i, l))),
        }
        println!("Instr: {:?}, curr pos: {:?}, way: {:?}", (i, l), start, way)
    }

    String::from(format!("{:?}", start.0.abs() + start.1.abs()))
}

fn parse(input: &Vec<String>) -> Vec<(char, i32)> {
    input
        .iter()
        .map(|l| {
            let mut c = l.chars();

            (c.next().unwrap(), c.collect::<String>().parse().unwrap())
        })
        .collect()
}

fn turn_left(from: char) -> char {
    match from {
        'N' => 'W',
        'S' => 'E',
        'W' => 'S',
        'E' => 'N',
        _ => panic!(format!("Unrecognized dir {:?}", from)),
    }
}

fn turn_right(from: char) -> char {
    match from {
        'N' => 'E',
        'S' => 'W',
        'W' => 'N',
        'E' => 'S',
        _ => panic!(format!("Unrecognized dir {:?}", from)),
    }
}

fn rotate_left(way: (i32, i32)) -> (i32, i32) {
    (-way.1, way.0)
}
fn rotate_right(way: (i32, i32)) -> (i32, i32) {
    (way.1, -way.0)
}
