use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> i32 {
    let raw_game_data = input_utils::read_file_data(input_path, true);
    let data = process_input_data(&raw_game_data);
    
    calculate(data)
}

fn calculate(data: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for line in data {
        result += calculate_line(line);
    }

    result
}

fn calculate_line(line: Vec<i32>) -> i32 {
    let mut result = Vec::new();
    result.push(line.clone());
    
    loop {
        let mut result_line = Vec::new();
        let mut stop = true;

        let working_line = result.get(result.len() - 1).unwrap();
        for i in 1..working_line.len() {
            let diff = working_line[i] - working_line[i - 1];
            if diff != 0 {
                stop = false;
            }
            result_line.push(diff);
        }

        result.push(result_line);

        if stop {
            break;
        }
    }

    let mut tracker = 0;
    for line in result.iter_mut().rev().skip(1) {
        // line.push(tracker + line.last().unwrap());
        // tracker = line.last().unwrap().clone();

        line.insert(0, line.first().unwrap() - tracker);
        tracker = line.first().unwrap().clone();
    }

    // result.first().unwrap().last().unwrap().clone()
    result.first().unwrap().first().unwrap().clone()
}

fn process_input_data(raw_game_data: &[String]) -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    for line in raw_game_data {
        let tokens = line.split(" ");
        let mut numbers = Vec::new();
        for token in tokens {
            numbers.push(token.parse::<i32>().unwrap());
        }

        data.push(numbers);
    }

    data
}