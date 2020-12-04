use crate::{first_answer, read_input};

pub fn run() {
    let input = read_input(1);
    let answer = input.join(" | ");
    first_answer(&format!("works! {}", answer));
}
