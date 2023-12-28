use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> u128 {
    let input = input_utils::read_file_data(input_path, true);
    let mut matrix: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // comment out for part 1
    for row in matrix.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == '<' || *cell == '>' || *cell == 'v' || *cell == '^' {
                *cell = '.';
            }
        }
    }

    let start = matrix[0].iter().position(|&c| c == '.').unwrap();
    let end = matrix[matrix.len() - 1].iter().position(|&c| c == '.').unwrap();

    longest_path(&matrix, (0, start), (matrix.len() - 1, end))
}

fn longest_path(matrix: &[Vec<char>], start: (i32, usize), end: (usize, usize)) -> u128 {
    let mut stack = Vec::new();
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    visited[start.0 as usize][start.1] = true;

    let mut longest_path: u128 = 0;
    stack.push((start.0, start.1, 0, visited.clone()));
    while !stack.is_empty() {
        let (row, col, path, mut visited) = stack.pop().unwrap();
        if row == end.0 as i32 && col == end.1 {
            longest_path = longest_path.max(path);
            continue;
        }

        if row > 0 && matrix[row as usize - 1][col] == '.' && !visited[row as usize - 1][col] {
            visited[row as usize - 1][col] = true;
            stack.push((row - 1, col, path + 1, visited.clone()));
        }

        if row < matrix.len() as i32 - 1 && matrix[row as usize + 1][col] == '.'
            && !visited[row as usize + 1][col]
        {
            visited[row as usize + 1][col] = true;
            stack.push((row + 1, col, path + 1, visited.clone()));
        }

        if col > 0 && matrix[row as usize][col - 1] == '.' && !visited[row as usize][col - 1] {
            visited[row as usize][col - 1] = true;
            stack.push((row, col - 1, path + 1, visited.clone()));
        }

        if col < matrix[0].len() - 1 && matrix[row as usize][col + 1] == '.'
            && !visited[row as usize][col + 1]
        {
            visited[row as usize][col + 1] = true;
            stack.push((row, col + 1, path + 1, visited.clone()));
        }

        /* Part 1 
        if col < matrix[0].len() - 2 && matrix[row as usize][col + 1] == '>' && matrix[row as usize][col + 2] == '.'
            && !visited[row as usize][col + 2]
        {
            visited[row as usize][col + 2] = true;
            stack.push((row, col + 2, path + 2, visited.clone()));
        }

        if col > 1 && matrix[row as usize][col - 1] == '<' && matrix[row as usize][col - 2] == '.'
            && !visited[row as usize][col - 2]
        {
            visited[row as usize][col - 2] = true;
            stack.push((row, col - 2, path + 2, visited.clone()));
        }

        if row < matrix.len() as i32 - 2 && matrix[row as usize + 1][col] == 'v' && matrix[row as usize + 2][col] == '.'
            && !visited[row as usize + 2][col]
        {
            visited[row as usize + 2][col] = true;
            stack.push((row + 2, col, path + 2, visited.clone()));
        }

        if row > 1 && matrix[row as usize - 1][col] == '^' && matrix[row as usize - 2][col] == '.'
            && !visited[row as usize - 2][col]
        {
            visited[row as usize - 2][col] = true;
            stack.push((row - 2, col, path + 2, visited.clone()));
        }

        */
    }

    longest_path
}