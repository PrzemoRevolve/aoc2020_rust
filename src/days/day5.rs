use crate::{first_answer, read_input, second_answer};

pub fn run() {
    let input = read_input(5);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let answer = parse_seats(input).iter().map(|t| t.2).max().unwrap();
    String::from(format!("{}", answer))
}

fn second_problem(input: &Vec<String>) -> String {
    let mut seat_ids: Vec<u32> = parse_seats(input).iter().map(|s| s.2).collect();
    seat_ids.sort_unstable();

    let mut prev = seat_ids[seat_ids.len() - 1];
    let mut ans = 0;

    for id in seat_ids {
        if prev == id - 2 {
            println!("{}", id);
            ans = id - 1;
            break;
        }

        prev = id;
    }

    String::from(format!("{}", ans))
}

fn parse_seats(input: &Vec<String>) -> Vec<(u32, u32, u32)> {
    input
        .iter()
        .map(|line| {
            let row: String = line[..line.len() - 3]
                .chars()
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    _ => ' ',
                })
                .collect();
            let row = u32::from_str_radix(&row, 2).unwrap();

            let col: String = line[line.len() - 3..]
                .chars()
                .map(|c| match c {
                    'L' => '0',
                    'R' => '1',
                    _ => ' ',
                })
                .collect();
            let col = u32::from_str_radix(&col, 2).unwrap();

            (row, col, row * 8 + col)
        })
        .collect()
}
