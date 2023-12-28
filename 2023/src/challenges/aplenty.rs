use std::collections::HashMap;

use crate::utils::input_utils;

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
    total: u32,
    is_accepted: bool,
}

#[derive(Debug, Clone)]
struct CombinationPart {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

#[derive(Debug)]
struct Condition {
    component: Option<char>,
    value: Option<u32>,
    operation: Option<char>,
    outcome: String,
}

pub fn execute(input_path: &std::path::Path) -> (u32, u128) {
    let input = input_utils::read_file_data(input_path, false);
    let index = input.iter().position(|line| line.is_empty());

    let rules = build_rules(&input[..index.unwrap()]);
    let mut parts = build_parts(&input[index.unwrap() + 1..]);

    process(&rules, &mut parts);
    let combinations = process_combinations(&rules);

    (
        parts
            .iter()
            .filter(|part| part.is_accepted)
            .map(|part| part.total)
            .sum(),
        combinations,
    )
}

fn process_combinations(rules: &HashMap<String, Vec<Condition>>) -> u128 {
    let mut total_combinations: u128 = 0;

    let mut stack = Vec::new();
    let part = CombinationPart {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    stack.push(("in", part.clone()));

    while !stack.is_empty() {
        let (rule_label, part) = stack.pop().unwrap();
        let rule_conditions = rules.get(rule_label).unwrap();
        let mut leftover = part.clone();
        for condition in rule_conditions {
            match condition.component {
                Some(_) => {
                    let (is_match, is_not_match) =
                        process_combination_condition(condition, &leftover);

                    if is_match.is_some() && condition.outcome == "A" {
                        let is_match = is_match.unwrap();
                        total_combinations += (is_match.x.1 - is_match.x.0 + 1) as u128
                            * (is_match.m.1 - is_match.m.0 + 1) as u128
                            * (is_match.a.1 - is_match.a.0 + 1) as u128
                            * (is_match.s.1 - is_match.s.0 + 1) as u128;
                    } else if is_match.is_some() && condition.outcome != "R" {
                        stack.push((&condition.outcome, is_match.unwrap().clone()));
                    } 

                    if is_not_match.is_some() {
                        leftover = is_not_match.unwrap();
                    } else {
                        break;
                    }
                }
                None => {
                    if condition.outcome == "A" {
                        total_combinations += (leftover.x.1 - leftover.x.0 + 1) as u128
                            * (leftover.m.1 - leftover.m.0 + 1) as u128
                            * (leftover.a.1 - leftover.a.0 + 1) as u128
                            * (leftover.s.1 - leftover.s.0 + 1) as u128;
                    } else if condition.outcome != "R" {
                        stack.push((&condition.outcome, leftover.clone()));
                    }
                }
            }
        }
    }

    total_combinations
}

fn process_combination_condition(
    condition: &Condition,
    part: &CombinationPart,
) -> (Option<CombinationPart>, Option<CombinationPart>) {
    let component = condition.component.unwrap();
    let value = condition.value.unwrap();
    let operation = condition.operation.unwrap();
    let mut is_match = part.clone();
    let mut is_not_match = part.clone();
    let mut result;

    match component {
        'x' => {
            if operation == '<' {
                is_match.x.1 = is_match.x.1.min(value - 1);
                is_not_match.x.0 = is_not_match.x.0.max(value);
            } else {
                is_match.x.0 = is_match.x.0.max(value + 1);
                is_not_match.x.1 = is_not_match.x.1.min(value);
            }
        }
        'm' => {
            if operation == '<' {
                is_match.m.1 = is_match.m.1.min(value - 1);
                is_not_match.m.0 = is_not_match.m.0.max(value);
            } else {
                is_match.m.0 = is_match.m.0.max(value + 1);
                is_not_match.m.1 = is_not_match.m.1.min(value);
            }
        }
        'a' => {
            if operation == '<' {
                is_match.a.1 = is_match.a.1.min(value - 1);
                is_not_match.a.0 = is_not_match.a.0.max(value);
            } else {
                is_match.a.0 = is_match.a.0.max(value + 1);
                is_not_match.a.1 = is_not_match.a.1.min(value);
            }
        }
        's' => {
            if operation == '<' {
                is_match.s.1 = is_match.s.1.min(value - 1);
                is_not_match.s.0 = is_not_match.s.0.max(value);
            } else {
                is_match.s.0 = is_match.s.0.max(value + 1);
                is_not_match.s.1 = is_not_match.s.1.min(value);
            }
        }
        _ => {
            is_match.x.0 = is_match.x.1 + 1;
            is_not_match.x.1 = is_not_match.x.0 - 1;
        }
    }

    result = (Some(is_match.clone()), Some(is_not_match.clone()));

    if is_match.x.0 > is_match.x.1
        || is_match.m.0 > is_match.m.1
        || is_match.a.0 > is_match.a.1
        || is_match.s.0 > is_match.s.1
    {
        result.0 = None;
    }

    if is_not_match.x.0 > is_not_match.x.1
        || is_not_match.m.0 > is_not_match.m.1
        || is_not_match.a.0 > is_not_match.a.1
        || is_not_match.s.0 > is_not_match.s.1
    {
        result.1 = None;
    }

    result
}

fn process(rules: &HashMap<String, Vec<Condition>>, parts: &mut [Part]) {
    for part in parts {
        let mut stack = Vec::new();
        stack.push("in");

        while !stack.is_empty() {
            let rule_label = stack.pop().unwrap();
            let rule_conditions = rules.get(rule_label).unwrap();

            for condition in rule_conditions {
                match condition.component {
                    Some(_) => {
                        if process_condition(condition, part) {
                            process_outcome(condition, part, &mut stack);
                            break;
                        }
                    }
                    None => {
                        process_outcome(condition, part, &mut stack);
                        break;
                    }
                }
            }
        }
    }
}

fn process_condition(condition: &Condition, part: &mut Part) -> bool {
    let component = condition.component.unwrap();
    let value = condition.value.unwrap();
    let operation = condition.operation.unwrap();

    match component {
        'x' => {
            if operation == '<' {
                part.x < value
            } else {
                part.x > value
            }
        }
        'm' => {
            if operation == '<' {
                part.m < value
            } else {
                part.m > value
            }
        }
        'a' => {
            if operation == '<' {
                part.a < value
            } else {
                part.a > value
            }
        }
        's' => {
            if operation == '<' {
                part.s < value
            } else {
                part.s > value
            }
        }
        _ => false,
    }
}

fn process_outcome<'a>(condition: &'a Condition, part: &mut Part, stack: &mut Vec<&'a str>) {
    if condition.outcome == "A" {
        part.is_accepted = true;
    } else if condition.outcome == "R" {
        part.is_accepted = false;
    } else {
        stack.push(&condition.outcome);
    }
}

fn build_parts(lines: &[String]) -> Vec<Part> {
    let mut parts = Vec::new();

    for line in lines {
        let tokens: Vec<&str> = line
            .split(|c| {
                c == '{'
                    || c == '}'
                    || c == ','
                    || c == '='
                    || c == 'x'
                    || c == 'm'
                    || c == 'a'
                    || c == 's'
            })
            .filter(|s| !s.is_empty())
            .collect();
        let x = tokens[0].parse().unwrap();
        let m = tokens[1].parse().unwrap();
        let a = tokens[2].parse().unwrap();
        let s = tokens[3].parse().unwrap();
        let total = x + m + a + s;
        let is_accepted = false;

        parts.push(Part {
            x,
            m,
            a,
            s,
            total,
            is_accepted,
        });
    }

    parts
}

fn build_rules(lines: &[String]) -> HashMap<String, Vec<Condition>> {
    let mut rules = HashMap::new();
    for line in lines {
        let tokens: Vec<&str> = line
            .split(|c| c == '{' || c == '}')
            .filter(|s| !s.is_empty())
            .collect();
        rules.insert(tokens[0].to_string(), build_conditions(tokens[1]));
    }
    rules
}

fn build_conditions(data: &str) -> Vec<Condition> {
    let parts = data.split(",").collect::<Vec<&str>>();
    let mut conditions = Vec::new();
    for part in parts {
        if !part.contains(':') {
            conditions.push(Condition {
                component: None,
                value: None,
                operation: None,
                outcome: part.to_string(),
            });
        } else {
            let condition_tokens = part
                .split(|c| c == '<' || c == '>' || c == ':')
                .collect::<Vec<&str>>();
            let component = part.chars().nth(0);
            let component_value: u32 = condition_tokens[1].parse().unwrap();
            let operation = part.chars().nth(1);
            let outcome = condition_tokens[2];

            conditions.push(Condition {
                component,
                value: Some(component_value),
                operation,
                outcome: outcome.to_string(),
            });
        }
    }
    conditions
}
