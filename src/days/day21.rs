use std::collections::{HashMap, HashSet};

use crate::{first_answer, read_input_no_skip, second_answer, Input};

pub fn run(day: u8) {
    let input = read_input_no_skip(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Input) -> String {
    let foods = parse(input);

    let mut ingredients = HashSet::new();
    let mut alergens = HashSet::new();
    // let mut alergen_to_ingredient = HashMap::new();

    for food in &foods {
        for ingredient in &food.ingredients {
            ingredients.insert(ingredient);
        }

        if let Some(a) = &food.alergens {
            for alergen in a {
                alergens.insert(alergen);
            }
        }
    }

    let start = &foods[0];

    println!("{:?}", ingredients);
    println!("{:?}", alergens);
    println!("{:?}", start);

    String::from(format!("{:?}", "a"))
}

fn second_problem(input: &Input) -> String {
    String::from(format!("{:?}", "r42, r31"))
}

fn parse(input: &Input) -> Vec<Food> {
    let mut res: Vec<Food> = input
        .iter()
        .map(|line| {
            let mut split = line.split(" (contains ");
            let ingredients = split
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
            let mut alergens = None;

            if let Some(x) = split.next() {
                alergens = Some(
                    x.trim()
                        .trim_end_matches(")")
                        .split(" ")
                        .map(|s| s.to_string())
                        .collect(),
                );
            }

            Food {
                ingredients,
                alergens,
            }
        })
        .collect();

    res.sort_by(|a, b| a.ingredients.len().cmp(&b.ingredients.len()));

    res
}

// fn choose(foods: &Vec<Food>, map: _) -> Option<_> {}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    alergens: Option<Vec<String>>,
}
