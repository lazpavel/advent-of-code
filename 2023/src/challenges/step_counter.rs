use std::collections::HashSet;

use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> usize {
    let raw_game_data = input_utils::read_file_data(input_path, true);
    let mut matrix: Vec<Vec<char>> = raw_game_data
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let start: (usize, usize) = matrix
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &cell)| if cell == 'S' { Some((i, j)) } else { None })
        })
        .unwrap();

    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == 'S' {
                *cell = '.';
            }
        }
    }

    let result = get_covered_cells(&mut matrix, start, 100);

    result
}

fn get_covered_cells(matrix: &[Vec<char>], start: (usize, usize), steps: u32) -> usize {
    let mut current_cells = HashSet::new();
    let mut next_cells = HashSet::new();

    current_cells.insert((start.0 as i32, start.1 as i32));

    for _ in 0..steps {
        for &(row, col) in &current_cells {
            let row_idx = calculate_idx(row, matrix.len());
            let col_idx = calculate_idx(col, matrix[0].len());
            let above_row = calculate_less_index(row_idx, matrix.len());
            let below_row = calculate_more_index(row_idx, matrix.len());
            let left_col = calculate_less_index(col_idx, matrix[0].len());
            let right_col = calculate_more_index(col_idx, matrix[0].len());

            if matrix[above_row][col_idx] == '.' {
                next_cells.insert((row - 1, col));
            }

            if matrix[below_row][col_idx] == '.' {
                next_cells.insert((row + 1, col));
            }

            if matrix[row_idx][left_col] == '.' {
                next_cells.insert((row, col - 1));
            }

            if matrix[row_idx][right_col] == '.' {
                next_cells.insert((row, col + 1));
            }
        }

        current_cells = next_cells.clone();
        next_cells.clear();
    }

    current_cells.len()
}

fn calculate_less_index(idx: usize, len: usize) -> usize {
    let result = if idx == 0 {
        len - 1
    } else {
        idx - 1
    };
    result
}

fn calculate_more_index(idx: usize, len: usize) -> usize {
    let result = if idx == len - 1 {
        0
    } else {
        idx + 1
    };
    result
}

fn calculate_idx(raw_index: i32, len: usize) -> usize {
    let mut idx = raw_index;
    while idx < 0 {
        idx += len as i32;
    }
    return idx as usize % len;
}
