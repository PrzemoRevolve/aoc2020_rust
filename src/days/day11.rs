use crate::{first_answer, read_input, second_answer};

pub fn run(day: u8) {
    let input = read_input(day);
    let matrix: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();

    first_answer(&first_problem(&matrix));
    second_answer(&second_problem(&matrix));
}

fn first_problem(matrix: &Vec<Vec<char>>) -> String {
    let mut m = matrix.clone();

    loop {
        let (n, changed) = run_day1(&m);
        m = n;

        if !changed {
            break;
        }
    }

    let ans: usize = m
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|c| *c == '#')
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    String::from(format!("{:?}", ans))
}

fn second_problem(matrix: &Vec<Vec<char>>) -> String {
    let mut m = matrix.clone();

    loop {
        let (n, changed) = run_day2(&m);
        m = n;

        if !changed {
            break;
        }
    }

    let ans: usize = m
        .iter()
        .map(|row| {
            row.into_iter()
                .filter(|c| **c == '#')
                .collect::<Vec<_>>()
                .len()
        })
        .sum();

    String::from(format!("{:?}", ans))
}

fn run_day2(matrix: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut changed = false;
    let mut new_matrix = vec![];

    for (j, row) in matrix.iter().enumerate() {
        new_matrix.push(vec![]);

        for (k, cell) in row.iter().enumerate() {
            let seen = get_seen_seats(matrix, (j as i32, k as i32));
            let state = get_new_state(*cell, seen, 5);

            if state != *cell {
                changed = true;
            }

            new_matrix[j].push(state);
        }
    }

    (new_matrix, changed)
}

fn run_day1(matrix: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut changed = false;
    let mut new_matrix = vec![];

    for (j, row) in matrix.iter().enumerate() {
        new_matrix.push(vec![]);

        for (k, cell) in row.iter().enumerate() {
            let adjacent = get_adjacent(matrix, (j, k));
            let state = get_new_state(*cell, adjacent, 4);

            if state != *cell {
                changed = true;
            }

            new_matrix[j].push(state);
        }
    }

    (new_matrix, changed)
}

fn get_new_state(seat: char, adjacent: Vec<char>, limit: usize) -> char {
    if seat == '.' {
        return seat;
    }

    let taken = adjacent
        .into_iter()
        .filter(|c| *c == '#')
        .collect::<Vec<_>>()
        .len();

    if seat == 'L' && taken == 0 {
        return '#';
    } else if seat == '#' && taken >= limit {
        return 'L';
    } else {
        return seat;
    }
}

fn get_adjacent(matrix: &Vec<Vec<char>>, p: (usize, usize)) -> Vec<char> {
    let mut v = vec![];
    let max0 = matrix.len() - 1;
    let max1 = matrix[0].len() - 1;

    if p.0 > 0 {
        v.push(matrix[p.0 - 1][p.1]);

        if p.1 > 0 {
            v.push(matrix[p.0 - 1][p.1 - 1]);
        }

        if p.1 < max1 {
            v.push(matrix[p.0 - 1][p.1 + 1]);
        }
    }

    if p.1 > 0 {
        v.push(matrix[p.0][p.1 - 1]);
    }

    if p.1 < max1 {
        v.push(matrix[p.0][p.1 + 1]);
    }

    if p.0 < max0 {
        v.push(matrix[p.0 + 1][p.1]);

        if p.1 > 0 {
            v.push(matrix[p.0 + 1][p.1 - 1]);
        }

        if p.1 < max1 {
            v.push(matrix[p.0 + 1][p.1 + 1]);
        }
    }

    v
}

fn get_seen_seats(matrix: &Vec<Vec<char>>, p: (i32, i32)) -> Vec<char> {
    let mut v = vec![];

    for x in -1..2 {
        for y in -1..2 {
            v.push(get_next_seat(p, x, y, matrix))
        }
    }

    v
}

fn get_next_seat(seat: (i32, i32), x: i32, y: i32, matrix: &Vec<Vec<char>>) -> char {
    let mut cy: i32 = seat.0;
    let mut cx: i32 = seat.1;

    if x == 0 && y == 0 {
        return '.';
    }

    loop {
        cx += x;
        cy += y;
        // println!("{},{}", cy, cx);
        if cx < 0 || cx as usize >= matrix[0].len() || cy < 0 || cy as usize >= matrix.len() {
            // println!("return . {},{}", cy, cx);
            return '.';
        }

        if matrix[cy as usize][cx as usize] != '.' {
            // println!("return {}: {},{}", matrix[cy as usize][cx as usize], cy, cx);

            return matrix[cy as usize][cx as usize];
        }
    }
}
