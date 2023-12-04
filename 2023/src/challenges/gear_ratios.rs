use std::ops::Add;

use crate::utils::input_utils;

pub fn calculate_gear_ratios(file_input_path: &std::path::Path) -> u32 {
    let lines = input_utils::read_file_data(file_input_path);

    let mut sum = 0;
    for line_index in 0..lines.len() {
        let line = lines.get(line_index).unwrap();
        let mut char_index = 0;
        while char_index < line.len() {
            let c = line.chars().nth(char_index).unwrap();
            if c.is_digit(10) && is_symbol_adjacent(char_index, line_index, &lines) {
                let gear_ratio = extract_number(line, char_index).1;
                sum += gear_ratio;

                while char_index < line.len() && line.chars().nth(char_index).unwrap().is_digit(10) {
                    char_index += 1;
                }
            }

            char_index += 1;
        }
    }

    sum
}

pub fn calculate_advanced_gear_ratios(file_input_path: &std::path::Path) -> u128 {
    let lines = input_utils::read_file_data(file_input_path);

    let mut advanced_sum: u128 = 0;
    for line_index in 0..lines.len() {
        let line = lines.get(line_index).unwrap();
        let mut char_index = 0;
        while char_index < line.len() {
            let c = line.chars().nth(char_index).unwrap();

            if c == '*' {
                let adjacent_numbers = get_adjacent_numbers(char_index, line_index, &lines);

                if adjacent_numbers.len() == 2 {
                    advanced_sum = advanced_sum.add((adjacent_numbers[0] as u128) * (adjacent_numbers[1] as u128));
                }
            }

            char_index += 1;
        }
    }

    advanced_sum
}

fn get_adjacent_numbers(char_index: usize, line_index: usize, lines: &Vec<String>) -> Vec<u32> {
    let mut numbers = Vec::new();

    let top_left = if line_index > 0 && char_index > 0 {
        get_char_safe(lines, line_index - 1, char_index - 1)
    } else {
        None
    };
    let top = if line_index > 0 {
        get_char_safe(lines, line_index - 1, char_index)
    } else {
        None
    };
    let top_right = if line_index > 0 {
        get_char_safe(lines, line_index - 1, char_index + 1)
    } else {
        None
    };
    let left = if char_index > 0 {
        get_char_safe(lines, line_index, char_index - 1)
    } else {
        None
    };
    let right = get_char_safe(lines, line_index, char_index + 1);
    let bottom_left = if char_index > 0 {
        get_char_safe(lines, line_index + 1, char_index - 1)
    } else {
        None as Option<char>
    };
    let bottom = get_char_safe(lines, line_index + 1, char_index);
    let bottom_right = get_char_safe(lines, line_index + 1, char_index + 1);

    let mut top_indexes: Vec<usize> = vec![];
    let mut bottom_indexes: Vec<usize> = vec![];

    let mut top_numbers = Vec::new();
    let mut same_line_numbers = Vec::new();
    let mut bottom_numbers = Vec::new();

    if let Some(top_left) = top_left{
        if top_left.is_digit(10) {
            let result = extract_number(&lines[line_index - 1], char_index - 1);
            if !top_indexes.contains(&result.0) {
                top_indexes.push(result.0);  
                top_numbers.push(result.1);  
            }
        }
    }

    if let Some(top) = top {
        if top.is_digit(10) {
            let result = extract_number(&lines[line_index - 1], char_index);
            if !top_indexes.contains(&result.0) {
                top_indexes.push(result.0);  
                top_numbers.push(result.1);  
            }
        }
    }

    if let Some(top_right) = top_right {
        if top_right.is_digit(10) {
            let result = extract_number(&lines[line_index - 1], char_index + 1);
            if !top_indexes.contains(&result.0) {
                top_indexes.push(result.0);  
                top_numbers.push(result.1);  
            }
        }
    }

    if let Some(left) = left {
        if left.is_digit(10) {
            let result = extract_number(&lines[line_index], char_index - 1);
            same_line_numbers.push(result.1);  
        }
    }

    if let Some(right) = right {
        if right.is_digit(10) {
            let result = extract_number(&lines[line_index], char_index + 1);
            same_line_numbers.push(result.1);  
        }
    }

    if let Some(bottom_left) = bottom_left {
        if bottom_left.is_digit(10) {
            let result = extract_number(&lines[line_index + 1], char_index - 1);

            if !bottom_indexes.contains(&result.0) {
                bottom_indexes.push(result.0);  
                bottom_numbers.push(result.1);  
            }
        }
    }

    if let Some(bottom) = bottom {
        if bottom.is_digit(10) {
            let result = extract_number(&lines[line_index + 1], char_index);

            if !bottom_indexes.contains(&result.0) {
                bottom_indexes.push(result.0);  
                bottom_numbers.push(result.1);  
            }
        }
    }

    if let Some(bottom_right) = bottom_right {
        if bottom_right.is_digit(10) {
            let result = extract_number(&lines[line_index + 1], char_index + 1);
            if !bottom_indexes.contains(&result.0) {
                bottom_indexes.push(result.0);  
                bottom_numbers.push(result.1);  
            }
        }
    }

    numbers.extend(top_numbers.iter());
    numbers.extend(bottom_numbers.iter());
    numbers.extend(same_line_numbers.iter());

    numbers
}

fn extract_number(line: &str, i: usize) -> (usize, u32) {
    // get first digit
    let mut first_index = i;
    for j in (0..i).rev() {
        if line.chars().nth(j).unwrap().is_digit(10) {
           first_index = j;
        } else {
            break;
        }
    }

    let mut number = 0;
    for j in first_index..line.len() {
        if line.chars().nth(j).unwrap().is_digit(10) {
            number = number * 10 + line.chars().nth(j).unwrap().to_digit(10).unwrap();
        } else {
            break;
        }
    }

    (first_index, number)
}

fn is_symbol_adjacent(char_index: usize, line_index: usize, lines: &Vec<String>) -> bool {
    let top_left = if line_index > 0 && char_index > 0 {
        get_char_safe(lines, line_index - 1, char_index - 1)
    } else {
        None
    };
    let top = if line_index > 0 {
        get_char_safe(lines, line_index - 1, char_index)
    } else {
        None
    };
    let top_right = if line_index > 0 {
        get_char_safe(lines, line_index - 1, char_index + 1)
    } else {
        None
    };
    let left = if char_index > 0 {
        get_char_safe(lines, line_index, char_index - 1)
    } else {
        None
    };
    let right = get_char_safe(lines, line_index, char_index + 1);
    let bottom_left = if char_index > 0 {
        get_char_safe(lines, line_index + 1, char_index - 1)
    } else {
        None as Option<char>
    };
    let bottom = get_char_safe(lines, line_index + 1, char_index);
    let bottom_right = get_char_safe(lines, line_index + 1, char_index + 1);

    let mut symbol_adjacent = false;
    if let Some(top_left) = top_left {
        symbol_adjacent = symbol_adjacent || !top_left.is_digit(10) && top_left != '.';
    }

    if let Some(top) = top {
        symbol_adjacent = symbol_adjacent || !top.is_digit(10) && top != '.';
    }

    if let Some(top_right) = top_right {
        symbol_adjacent = symbol_adjacent || !top_right.is_digit(10) && top_right != '.';
    }

    if let Some(left) = left {
        symbol_adjacent = symbol_adjacent || !left.is_digit(10) && left != '.';
    }

    if let Some(right) = right {
        symbol_adjacent = symbol_adjacent || !right.is_digit(10) && right != '.';
    }

    if let Some(bottom_left) = bottom_left {
        symbol_adjacent = symbol_adjacent || !bottom_left.is_digit(10) && bottom_left != '.';
    }

    if let Some(bottom) = bottom {
        symbol_adjacent = symbol_adjacent || !bottom.is_digit(10) && bottom != '.';
    }

    if let Some(bottom_right) = bottom_right {
        symbol_adjacent = symbol_adjacent || !bottom_right.is_digit(10) && bottom_right != '.';
    }

    symbol_adjacent
}

fn get_char_safe(lines: &Vec<String>, line_index: usize, char_index: usize) -> Option<char> {
    if line_index < lines.len() {
        let line = &lines[line_index];
        if char_index < line.len() {
            return Some(line.chars().nth(char_index)?);
        }
    }

    None
}
