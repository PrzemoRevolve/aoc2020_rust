use crate::{first_answer, read_input, second_answer};
use itertools::Itertools;

pub fn run() {
    let input = read_input(10);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Vec<String>) -> String {
    let adapters = get_sorted_adapters(&input);
    let ans = use_all(&adapters);
    String::from(format!("{:?}", ans.0 * ans.2))
}

fn second_problem(input: &Vec<String>) -> String {
    let mut adapters = get_sorted_adapters(&input);
    adapters.insert(0, 0);
    let last = adapters[adapters.len() - 1];
    adapters.push(last + 3);

    let mut groups = vec![];
    let mut group = vec![];

    for (i, adapter) in adapters.iter().enumerate() {
        if i == 0 || i == adapters.len() - 1 {
            continue;
        }

        if adapter - adapters[i - 1] == 1 && adapters[i + 1] - adapter == 1 {
            group.push(adapter);
        } else if !group.is_empty() {
            groups.push(group);
            group = vec![];
        }
    }

    let mut ans: u64 = 1;

    for g in groups {
        match g.len() {
            1 => ans *= 2,
            2 => ans *= 4,
            3 => ans *= 7,
            _ => println!("Unexpected group length! {}", g.len()),
        }
    }

    String::from(format!("{:?}", ans))
}

fn get_sorted_adapters(input: &Vec<String>) -> Vec<u64> {
    input.iter().map(|l| l.parse().unwrap()).sorted().collect()
}

fn use_all(adapters: &Vec<u64>) -> (u64, u64, u64) {
    let mut j1 = 0;
    let mut j2 = 0;
    let mut j3 = 1;
    let mut initial = 0;

    for adapter in adapters {
        if adapter - initial == 1 {
            j1 += 1;
        } else if adapter - initial == 2 {
            j2 += 1;
        } else if adapter - initial == 3 {
            j3 += 1;
        }

        initial = *adapter;
    }

    (j1, j2, j3)
}
