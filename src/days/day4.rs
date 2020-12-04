use crate::{first_answer, second_answer};

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    first_answer(&first_problem());
    second_answer(&second_problem());
}

fn first_problem() -> String {
    let passports = get_passports();

    let mut valid_passports = 0;

    for passport in passports.iter() {
        let values: Vec<&str> = passport
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| &v[0..3])
            .collect();

        if values.len() > 7 || values.len() == 7 && values.iter().all(|v| v != &"cid") {
            valid_passports += 1
        }
    }

    String::from(format!("{}", valid_passports))
}

fn second_problem() -> String {
    let passports = get_passports();
    let mut valid_passports = 0;

    for passport in passports.iter() {
        let values: Vec<(&str, &str)> = passport
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| (&v[0..3], &v[4..]))
            .collect();

        if values.len() > 7 || values.len() == 7 && values.iter().all(|v| v.0 != "cid") {
            let mut valid = true;

            for value in values.into_iter() {
                match value.0 {
                    "byr" => {
                        let year: i32 = value.1.parse().unwrap();

                        if year > 2002 || year < 1920 {
                            print!("invalid byr {}", year);
                            valid = false
                        }
                    }
                    "iyr" => {
                        let year: i32 = value.1.parse().unwrap();

                        if year > 2020 || year < 2010 {
                            print!("invalid iyr {}", year);
                            valid = false
                        }
                    }
                    "eyr" => {
                        let year: i32 = value.1.parse().unwrap();

                        if year > 2030 || year < 2020 {
                            print!("invalid eyr {}", year);
                            valid = false
                        }
                    }
                    "hgt" => {
                        if value.1.len() < 3 {
                            print!("invalid hgt {}", value.1);
                            valid = false;
                            break;
                        }

                        let unit = &value.1[value.1.len() - 2..];
                        let height: i32 =
                            value.1[..value.1.len() - 2].parse().unwrap_or_else(|e| {
                                println!("ERRORED {:?}", value);
                                panic!("er");
                            });

                        match unit {
                            "cm" => {
                                if height > 193 || height < 150 {
                                    print!("invalid hgt {}", value.1);
                                    valid = false
                                }
                            }
                            "in" => {
                                if height > 76 || height < 59 {
                                    print!("invalid hgt {}", value.1);
                                    valid = false
                                }
                            }
                            _ => {
                                print!("invalid hgt {}", value.1);
                                valid = false
                            }
                        }
                    }
                    "hcl" => {
                        if value.1.len() != 7 || value.1.chars().nth(0).unwrap() != '#' {
                            print!("invalid hcl {}", value.1);
                            valid = false;
                        }

                        if !value.1[1..]
                            .chars()
                            .all(|c| (c >= 'a' && c <= 'f') || (c >= '0' && c <= '9'))
                        {
                            print!("invalid hcl {}", value.1);
                            valid = false
                        }
                    }
                    "ecl" => {
                        let allowed = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

                        if !allowed.iter().any(|&v| v == value.1) {
                            print!("invalid ecl {}", value.1);
                            valid = false
                        }
                    }
                    "pid" => {
                        if value.1.len() != 9 {
                            print!("invalid pid {}", value.1);
                            valid = false;
                        }

                        if !value.1.chars().all(|c| c >= '0' && c <= '9') {
                            print!("invalid pid {}", value.1);
                            valid = false
                        }
                    }
                    "cid" => (),
                    _ => valid = false,
                }
            }

            if valid {
                println!(" valid{:?}", passport);
                valid_passports += 1
            } else {
                println!("invalid{:?}", passport);
            }
        }
    }

    String::from(format!("{}", valid_passports))
}

fn get_passports() -> Vec<String> {
    let filename = format!("input/day-{}.txt", 4);
    let file = File::open(&filename).expect(&format!("Unable to open input file in {}", filename));
    let mut passports: Vec<String> = vec![];
    let mut new_passport = String::from("");

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if !new_passport.is_empty() {
                passports.push(new_passport);
            }

            new_passport = String::from("");
            continue;
        }

        new_passport.push(' ');
        new_passport.push_str(&line);
    }

    passports.push(new_passport);

    passports
}
