use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> u32 {
    let input = input_utils::read_file_data(input_path, true);
    let arrangements = get_arrangements(&input);

    arrangements.iter().sum()
}

fn get_arrangements(input: &[String]) -> Vec<u32> {
    let mut arrangements = Vec::new();

    for line in input {
        let count = get_line_arrangements(line);
        arrangements.push(count);

        // println!("{} :: {}", line, count);
    }

    arrangements
}

fn get_line_arrangements(line: &str) -> u32 {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let mut records = parts[0]
        .split('.')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut numbers: Vec<u32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

    let mut delete_records: Vec<usize> = Vec::new();
    let mut delete_numbers: Vec<usize> = Vec::new();
    for i in 0..records.len() {
        if i < numbers.len() && records[i].len() == numbers[i] as usize {
            delete_records.push(i);
            delete_numbers.push(i);
        } else {
            break;
        }
    }
    records = records
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !delete_records.contains(i))
        .map(|(_, item)| item)
        .collect();

    numbers = numbers
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !delete_numbers.contains(i))
        .map(|(_, item)| item)
        .collect();

    delete_records.clear();
    delete_numbers.clear();

    if numbers.len() > 0 && records.len() > 0 {
        let mut numbers_idx = (numbers.len() - 1) as i32;
        for i in (0..records.len()).rev() {
            if numbers_idx >= 0 && records[i].len() == numbers[numbers_idx as usize] as usize {
                delete_records.push(i);
                delete_numbers.push(numbers_idx as usize);
                numbers_idx -= 1;
            } else {
                break;
            }
        }
    }

    records = records
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !delete_records.contains(i))
        .map(|(_, item)| item)
        .collect();

    numbers = numbers
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !delete_numbers.contains(i))
        .map(|(_, item)| item)
        .collect();

    delete_records.clear();
    delete_numbers.clear();

    if numbers.len() == records.len() {
        let mut matches = true;
        for i in 0..numbers.len() {
            if numbers[i] != records[i].len() as u32 {
                matches = false;
                break;
            }
        }

        if matches {
            return 1;
        }
    }

    calculate_arrangements(&records, &numbers)
}

fn calculate_arrangements(records: &[&str], numbers: &[u32]) -> u32 {
    let mut used_index = 0;

    for i in 0..records.len() {
        let max_numbers = numbers.len() - (records.len() - i - 1);
        for j in used_index..max_numbers + 1 {
            let slice = &numbers[used_index..j];
            let combinations = generate_combinations(&records[i]);

            for combination in combinations {
                let _matches = get_combination_matches(&combination, slice);
            }
        }

        used_index += 1;
    }

    0
}

fn get_combination_matches(_combination: &str, _slice: &[u32]) -> (bool, u32) {
    (false, 0)
}

fn generate_combinations(record: &str) -> Vec<String> {
    if let Some(index) = record.find('?') {
        let mut s1 = record.to_string();
        s1.replace_range(index..index+1, ".");
        let mut s2 = record.to_string();
        s2.replace_range(index..index+1, "#");
        let mut result = generate_combinations(&s1);
        result.extend(generate_combinations(&s2));
        result
    } else {
        vec![record.to_string()]
    }
}
