use std::collections::HashMap;

use crate::utils::input_utils;

#[derive(Debug)]
struct Range {
    source: u128,
    destination: u128,
    range: u128,
}

#[derive(Debug)]
struct AlmanacMap {
    seeds: Vec<(u128, u128)>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

pub fn calculate_almanac_map(input_path: &std::path::Path) -> u128 {
    let raw_game_data = input_utils::read_file_data(input_path);
    let almanac_map = process_almanac_map(raw_game_data);
    process_game_data(almanac_map)
}

fn process_game_data(almanac_map: AlmanacMap) -> u128 {
    let mut min_location = u128::MAX;

    for seed_tuple in almanac_map.seeds {
        println!("Processing seed tuple: {:?}", seed_tuple);
        for seed in seed_tuple.0..(seed_tuple.0 + seed_tuple.1) {
            let mut soil = seed;
            
            for seed_to_soil in &almanac_map.seed_to_soil {
                if seed >= seed_to_soil.source && seed < seed_to_soil.source + seed_to_soil.range {
                    soil = seed_to_soil.destination + seed - seed_to_soil.source;
                }
            }

            let mut fertilizer = soil;
            for soil_to_fertilizer in &almanac_map.soil_to_fertilizer {
                if soil >= soil_to_fertilizer.source
                    && soil < soil_to_fertilizer.source + soil_to_fertilizer.range
                {
                    fertilizer = soil_to_fertilizer.destination + soil - soil_to_fertilizer.source;
                }
            }

            let mut water = fertilizer;
            for fertilizer_to_water in &almanac_map.fertilizer_to_water {
                if fertilizer >= fertilizer_to_water.source
                    && fertilizer < fertilizer_to_water.source + fertilizer_to_water.range
                {
                    water =
                        fertilizer_to_water.destination + fertilizer - fertilizer_to_water.source;
                }
            }

            let mut light: u128 = water;
            for water_to_light in &almanac_map.water_to_light {
                if water >= water_to_light.source
                    && water < water_to_light.source + water_to_light.range
                {
                    light = water_to_light.destination + water - water_to_light.source;
                }
            }

            let mut temperature: u128 = light;
            for light_to_temperature in &almanac_map.light_to_temperature {
                if light >= light_to_temperature.source
                    && light < light_to_temperature.source + light_to_temperature.range
                {
                    temperature =
                        light_to_temperature.destination + light - light_to_temperature.source;
                }
            }

            let mut humidity = temperature;
            for temperature_to_humidity in &almanac_map.temperature_to_humidity {
                if temperature >= temperature_to_humidity.source
                    && temperature < temperature_to_humidity.source + temperature_to_humidity.range
                {
                    humidity = temperature_to_humidity.destination + temperature
                        - temperature_to_humidity.source;
                }
            }

            for humidity_to_location in &almanac_map.humidity_to_location {
                if humidity >= humidity_to_location.source
                    && humidity < humidity_to_location.source + humidity_to_location.range
                {
                    if humidity_to_location.destination + humidity - humidity_to_location.source
                        < min_location
                    {
                        min_location = humidity_to_location.destination + humidity
                            - humidity_to_location.source;
                    }
                }
            }
        }

        println!("Min location: {}", min_location);
    }

    min_location
}

fn process_almanac_map(raw_game_data: Vec<String>) -> AlmanacMap {
    let mut categories: HashMap<String, Vec<Vec<u128>>> = HashMap::new();
    let mut current_category = String::new();

    for line in raw_game_data {
        if line.contains("seeds") {
            categories.insert(
                "seeds".to_string(),
                line.split(':')
                    .skip(1)
                    .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
                    .collect(),
            );
        } else if line.contains("map:") {
            current_category = line.split_whitespace().next().unwrap().to_string();
        } else if !line.trim().is_empty() {
            let numbers: Vec<u128> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            categories
                .entry(current_category.clone())
                .or_default()
                .push(numbers);
        }
    }

    let mut seeds: Vec<(u128, u128)> = Vec::new();
    let raw_seeds = categories.get("seeds").unwrap()[0].clone();

    for i in (0..raw_seeds.len()).step_by(2) {
        seeds.push((raw_seeds[i], raw_seeds[i + 1]));
    }

    AlmanacMap {
        seeds,
        seed_to_soil: categories
            .get("seed-to-soil")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        soil_to_fertilizer: categories
            .get("soil-to-fertilizer")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        fertilizer_to_water: categories
            .get("fertilizer-to-water")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        water_to_light: categories
            .get("water-to-light")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        light_to_temperature: categories
            .get("light-to-temperature")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        temperature_to_humidity: categories
            .get("temperature-to-humidity")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
        humidity_to_location: categories
            .get("humidity-to-location")
            .unwrap()
            .iter()
            .map(|v| Range {
                source: v[1],
                destination: v[0],
                range: v[2],
            })
            .collect(),
    }
}
