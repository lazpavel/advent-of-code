use crate::utils::input_utils;

use std::collections::HashMap;
use std::ops::Add;

pub fn calculate_cube_conundrum(
    input_file_path: &std::path::Path,
    config: HashMap<&str, u32>,
) -> (u32, u128) {
    let lines = input_utils::read_file_data(input_file_path, false);
    let games_data = get_games_data(&lines);

    let mut sum = 0;
    let mut games_minimum_required_product_sum: u128 = 0;

    for entry in games_data.iter() {
        let game_id = entry.0;
        let game_data = entry.1;

        let mut game_successful = true;
        let mut game_minimum_required = HashMap::new();

        for game_round in game_data.iter() {
            for (key, value) in game_round.iter() {
                if !game_minimum_required.contains_key(key) {
                    game_minimum_required.insert(key, *value);
                }

                if game_minimum_required.get(key).unwrap() < value {
                    game_minimum_required.insert(key, *value);
                }

                if !config.contains_key(key) || config.get(key).unwrap() < value {
                    game_successful = false;
                }
            }
        }

        let mut game_minimum_required_product = 1;
        for (_, value) in game_minimum_required.iter() {
            game_minimum_required_product *= value;
        }
        games_minimum_required_product_sum =
            games_minimum_required_product_sum.add(game_minimum_required_product as u128);

        if game_successful {
            sum += game_id;
        }
    }

    (sum, games_minimum_required_product_sum)
}

fn get_games_data<'a>(lines: &'a Vec<String>) -> HashMap<u32, Vec<HashMap<&'a str, u32>>> {
    let mut games_data = HashMap::new();

    for line in lines.iter() {
        let v: Vec<_> = line.split(':').collect();

        let game_id = v[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        for i in 1..v.len() {
            let mut game_data = vec![];

            for game_line_entry in v[i].split(';') {
                let mut game_round = HashMap::new();
                let round_entry: Vec<_> = game_line_entry.split(',').collect();
                for round_entry_it in round_entry {
                    let round_entry_value = round_entry_it.split_whitespace().next().unwrap();
                    let round_entry_key = round_entry_it.split_whitespace().last().unwrap();
                    game_round.insert(round_entry_key, round_entry_value.parse::<u32>().unwrap());
                }

                game_data.push(game_round);
            }

            games_data.insert(game_id, game_data);
        }
    }

    games_data
}
