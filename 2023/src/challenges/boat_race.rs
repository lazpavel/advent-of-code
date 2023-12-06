use crate::utils::input_utils;

pub fn calculate(input_path: &std::path::Path) -> u128 {
    let raw_game_data = input_utils::read_file_data(input_path);
    let game_data = parse_boat_race_data(raw_game_data);
    
    calculate_wins(game_data.0, game_data.1)
}

fn calculate_wins(times: Vec<u128>, distances: Vec<u128>) -> u128 {
    let mut win_score = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let mut wins = 0;
        for j in 0..time {
            if (time - j) * j > distance {
                wins += 1;
            }
        }

        win_score *= wins;
    }

    win_score
}

fn parse_boat_race_data(raw_game_data: Vec<String>) -> (Vec<u128>, Vec<u128>) {
    let times = raw_game_data[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let distances = raw_game_data[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    (times, distances)
}
