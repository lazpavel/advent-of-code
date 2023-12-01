use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn calculate_trebuchet(input: &std::path::Path) -> u32 {
    println!("Calculating trebuchet...");

    let file = File::open(input).unwrap();
    let reader = io::BufReader::new(file);

    // Vec to store the lines
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line);
    }

    let mut sum = 0;
    for line in lines.iter() {
        if let Ok(line) = line {
            let first_digit = get_first_digit(line);
            let last_digit = get_last_digit(line);

            let result = first_digit * 10 + last_digit;
            sum += result;
        }
    }

    sum
}

fn get_last_digit(line: &str) -> u32 {
    let digit_map = get_digit_map();

    let mut index = 0;
    let mut result = 0;
    for digit in digit_map.keys() {
        let position = line.rfind(&digit.to_string());
        if let Some(position) = position {
            if position >= index {
                index = position;
                result = *digit_map.get(digit).unwrap();
            }
        }
    }

    result
}

fn get_first_digit(line: &str) -> u32 {
    let digit_map = get_digit_map();

    let mut index = line.len();
    let mut result = 0;
    for digit in digit_map.keys() {
        if let Some(position) = line.find(&digit.to_string()[..]) {
            if position <= index {
                index = position;
                result = *digit_map.get(digit).unwrap();
            }
        }
    }

    result
}

fn get_digit_map() -> HashMap<&'static str, u32> {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .cloned()
    .collect()
}
