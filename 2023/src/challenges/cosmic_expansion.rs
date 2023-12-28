use std::vec;

use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> u128 {
    let input = input_utils::read_file_data(input_path, true);
    let data = convert_input(&input);
    let expanded_data = expand_data(&data);

    // println!("{:?}", expanded_data);

    let nodes = get_nodes(&expanded_data);
    let distances = get_distances(&expanded_data, &nodes);
    
    distances.iter().map(|&(_, _, distance)| distance).sum()
}

fn get_distances(expanded_data: &[Vec<char>], nodes: &[(u32, usize, usize)]) -> Vec<(u32, u32, u128)> {
    let mut distances: Vec<(u32, u32, u128)> = vec![];

    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            let distance = get_distance(expanded_data, nodes[i], nodes[j]);
            distances.push((nodes[i].0, nodes[j].0, distance));
        }
    }

    distances
}

fn get_distance(expanded_data: &[Vec<char>], i: (u32, usize, usize), j: (u32, usize, usize)) -> u128 {
    let min_row = std::cmp::min(i.1, j.1);
    let max_row = std::cmp::max(i.1, j.1);
    let min_column = std::cmp::min(i.2, j.2);
    let max_column = std::cmp::max(i.2, j.2);
    
    let mut distance = (max_row as i32 - min_row as i32) as u128 + (max_column as i32  - min_column as i32) as u128;
    for row in min_row..max_row {
        if expanded_data[row][min_column] == 'X' {
            distance += 1000000 - 1;
        }
        
    }

    for column in min_column..max_column {
        if expanded_data[max_row][column] == 'X' {
            distance += 1000000 - 1;
        }
    }

    distance
}

fn get_nodes(expanded_data: &[Vec<char>]) -> Vec<(u32, usize, usize)> {
    let mut nodes: Vec<(u32, usize, usize)> = vec![];

    let mut node_idx = 0;
    for row in 0..expanded_data.len() {
        for column in 0..expanded_data[0].len() {
            if expanded_data[row][column] == '#' {
                nodes.push((node_idx, row, column));
                node_idx += 1;
            }
        }
    }

    nodes
}

fn expand_data(data: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut expanded_data: Vec<Vec<char>> = Vec::new();

    for line in data {
        if !line.contains(&'#') {
            expanded_data.push(vec!['X'; line.len()]);
        } else {
            expanded_data.push(line.clone());
        }
    }

    for column in 0..data[0].len() {
        if !column_does_contain(data, column, '#') {
            for row in &mut expanded_data.iter_mut() {
                row[column] = 'X';
            }
        }
    }

    expanded_data
}

fn column_does_contain(data: &[Vec<char>], column: usize, char_to_check: char) -> bool {
    for row in data {
        if row.get(column) == Some(&char_to_check) {
            return true;
        }
    }
    false
}

fn convert_input(input: &[String]) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = Vec::new();

    for line in input {
        data.push(line.chars().into_iter().collect());
    }

    data
}


