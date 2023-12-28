use regex::Regex;
use std::collections::HashMap;
use num_integer::lcm;

use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> u128 {
    let raw_game_data = input_utils::read_file_data(input_path, true);
    let (steps, graph, start_points) = extract_map_graph(&raw_game_data);
    let mut steps_generator = steps.iter().cycle();
    
    let mut counters = vec![];

    for start_point in start_points {
        let mut current = start_point;
        let mut counter: u32 = 0;
        loop {
            if current.ends_with("Z") {
                counters.push(counter);
                break;
            }
    
            let step = steps_generator.next().unwrap();
            counter += 1;
    
            if *step == 0 {
                current = graph.get(&current).unwrap().0.to_string();
            } else {
                current = graph.get(&current).unwrap().1.to_string();
            }
        }
    }

    println!("{:?}", counters);
    calculate_lcm(&counters)
}

fn calculate_lcm(numbers: &[u32]) -> u128 {
    numbers.iter().fold(1, |acc, &n| lcm(acc, n.into()))
}

fn extract_map_graph(
    raw_game_data: &[String],
) -> (
    Vec<u32>,
    HashMap<String, (String, String)>,
    Vec<String>,
) {
    let steps = raw_game_data[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    let re = Regex::new(r"\(|\)|,").unwrap();
    let mut start_points = Vec::new();
    let mut nodes = HashMap::new();

    for line in &raw_game_data[1..] {
        let mut tokens = line.split("=");
        let vertex = tokens.next().unwrap().trim();
        let children: Vec<&str> = re
            .split(tokens.next().unwrap())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if vertex.ends_with("A") {
            start_points.push(vertex.to_string());
        }

        nodes.insert(
            vertex.to_string(),
            (children[0].to_string(), children[1].to_string()),
        );
    }

    (steps, nodes, start_points)
}
