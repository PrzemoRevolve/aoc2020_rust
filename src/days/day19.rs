use regex::Regex;

use crate::{first_answer, read_input, second_answer, Input};

#[derive(Debug)]
struct Rule {
    id: usize,
    rule: Exp,
}

#[derive(Debug)]
enum Exp {
    Str(String),
    Or(Vec<Ids>),
    Ids(Ids),
}

type Id = usize;

type Ids = Vec<Id>;

pub fn run(day: u8) {
    let input = read_input(day);

    first_answer(&first_problem(&input));
    second_answer(&second_problem(&input));
}

fn first_problem(input: &Input) -> String {
    let (rules, data) = parse(input);
    let s = rule_to_string(find_rule(&rules, 0), &rules);
    let reg = Regex::new(&format!("^{}$", s)).unwrap();
    let ans = data.iter().filter(|d| reg.is_match(d)).count();

    String::from(format!("{:?}", ans))
}

fn second_problem(input: &Input) -> String {
    let (rules, data) = parse(input);
    // let r42 = rule_to_string(find_rule(&rules, 42), &rules);
    // let r31 = rule_to_string(find_rule(&rules, 31), &rules);
    String::from(format!("{:?} {:?}", "r42", "r31"))
}

fn parse(input: &Input) -> (Vec<Rule>, Input) {
    let rule_exp = Regex::new(r"([0-9]+): (.+)").unwrap();
    let char_exp = Regex::new(r#""(.)""#).unwrap();
    let or_exp = Regex::new(r"(.+)(?: \| (.+))+").unwrap();
    let mut rules = vec![];
    let mut data = vec![];

    for line in input {
        if let Some(c) = rule_exp.captures(&line) {
            let id = c.get(1).unwrap().as_str().parse().unwrap();
            let rest = c.get(2).unwrap().as_str();
            let rule;

            if let Some(d) = char_exp.captures(rest) {
                rule = Exp::Str(d.get(1).unwrap().as_str().to_string());
            } else if let Some(d) = or_exp.captures(rest) {
                // println!("Parsing Or");
                // add

                rule = Exp::Or(parse_rule_or(rest));
            } else {
                // add
                // println!("Parsing Ids");
                rule = Exp::Ids(parse_rule_ids(rest));
            }

            rules.push(Rule { id, rule })
        } else {
            data.push(line.to_owned());
        }
    }

    (rules, data)
}

fn parse_rule_or(s: &str) -> Vec<Ids> {
    let or_exp = Regex::new(r"(.+)(?: \| (.+))+").unwrap();

    or_exp
        .captures(s)
        .unwrap()
        .iter()
        .skip(1)
        .filter(|d| d.is_some())
        .flat_map(|d| {
            // println!("Matching {:?}", d);

            let res = match or_exp.is_match(d.unwrap().as_str()) {
                true => parse_rule_or(d.unwrap().as_str()),
                false => vec![parse_rule_ids(d.unwrap().as_str())],
            };
            res
        })
        .collect()
}

fn parse_rule_ids(s: &str) -> Ids {
    s.split(" ")
        .map(|c| {
            c.parse().unwrap_or_else(|_| {
                panic!(format!(
                    "Parsing error, cannot parse to usize {} whole str: {}",
                    c, s
                ))
            })
        })
        .collect()
}

fn find_rule(rules: &Vec<Rule>, id: Id) -> &Rule {
    rules.iter().find(|r| r.id == id).unwrap()
}

fn rule_to_string(rule: &Rule, rules: &Vec<Rule>) -> String {
    // println!("Matching rule {:?}", rule);
    let res = match &rule.rule {
        Exp::Str(s) => s.to_owned(),
        Exp::Ids(ids) => ids_to_string(&ids, rules),
        Exp::Or(ids_vec) => {
            let mut s = "(".to_string();
            let s1 = ids_vec
                .iter()
                .map(|ids| ids_to_string(ids, rules))
                .collect::<Vec<String>>()
                .join("|");
            s.push_str(&s1);
            s.push(')');
            s
        }
    };
    // println!("rule: {}", res);
    res
}

fn ids_to_string(ids: &Ids, rules: &Vec<Rule>) -> String {
    ids.iter()
        .map(|id| find_rule(rules, *id))
        .map(|r| rule_to_string(r, rules))
        .fold(String::new(), |mut string, s| {
            string.push_str(&s);
            string
        })
}
