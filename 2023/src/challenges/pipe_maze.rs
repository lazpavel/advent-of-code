use std::{cmp::max, vec};

use crate::{
    data_structures::graph::{Edge, Graph},
    utils::input_utils,
};

pub fn execute(input_path: &std::path::Path) -> usize {
    let raw_game_data = input_utils::read_file_data(input_path);
    let chars = vec!['|' /*, '-', 'L', 'J', '7', 'F' */];
    let cols = raw_game_data[0].len() * 2;
    let mut max_graph = None;
    let mut max_cycle = None;
    let mut max_cycle_length = 0;
    for ch in chars {
        let (start, graph) = build_graph(&raw_game_data, ch);
        let cycle = find_cycle(
            &graph,
            if ch == '-' {
                start - 1
            } else if ch == '|' {
                start - cols
            } else {
                start
            },
        );

        match cycle {
            Some(cycle) => {
                max_graph = Some(graph);
                max_cycle_length = max(max_cycle_length, cycle.len() / 4);
                max_cycle = Some(cycle);
            }
            None => {}
        }
    }

    let mut matrix = build_maze_matrix(
        &raw_game_data,
        max_cycle.as_ref().unwrap(),
        max_graph.as_ref().unwrap(),
    );
    let result = fill_matrix(&mut matrix, &raw_game_data);
    println!("{:?}", result);

    max_cycle_length
}

fn fill_matrix(matrix: &mut [Vec<i32>], raw_game_data: &[String]) -> Vec<Vec<char>> {
    let mut result = vec![vec![' '; matrix[0].len()]; matrix.len()];
    loop {
        let mut changed = false;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == -2 || matrix[i][j] == -2 {
                    continue;
                }

                if matrix[i][j] == -1 {
                    if i == 0 || i == matrix.len() - 1 || j == 0 || j == matrix[0].len() - 1 {
                        matrix[i][j] = -2;
                        changed = true;
                    } else if matrix[i - 1][j] == -2
                        || matrix[i + 1][j] == -2
                        || matrix[i][j - 1] == -2
                        || matrix[i][j + 1] == -2
                    {
                        matrix[i][j] = -2;
                        changed = true;
                    } 
                }
            }
        }

        if !changed {
            break;
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == -1 {
                result[i][j] = '*';
            } else if matrix[i][j] == -2 {
                result[i][j] = ' ';
            } else {
                result[i][j] = raw_game_data[i].chars().nth(j).unwrap();
            }
        }
    }

    result
}

fn build_maze_matrix(raw_game_data: &[String], cycle: &Vec<usize>, graph: &Graph) -> Vec<Vec<i32>> {
    let rows = raw_game_data.len();
    let cols = raw_game_data[0].len();

    let mut matrix = vec![vec![-1; cols]; rows];

    let mut count = 0;
    for i in 0..cycle.len() {
        let x = graph.get_edge(cycle[i]).unwrap().pos_x;
        let y = graph.get_edge(cycle[i]).unwrap().pos_y;

        if matrix[y][x] == -1 {
            matrix[y][x] = count;
            count += 1;
        }
    }

    matrix
}

fn build_graph(raw_game_data: &[String], start_char: char) -> (usize, Graph) {
    let mut start = 0;
    let rows = raw_game_data.len() * 2;
    let cols = raw_game_data[0].len() * 2;

    let mut graph: Graph = Graph::new(rows * cols);
    let mut row = 0;

    let mut pos_y = 0;
    for line in raw_game_data {
        let mut col = 0;
        let mut pos_x = 0;

        for mut ch in line.chars() {
            let current = row * cols + col;

            if current >= graph.size {
                break;
            }

            if ch == 'S' {
                start = current;
                ch = start_char;
            }
            connect_graph(ch, current, cols, &mut graph, (pos_x, pos_y));

            col += 2;
            pos_x += 1;
        }

        pos_y += 1;
        row += 2;
    }

    (start, graph)
}

fn connect_graph(
    ch: char,
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    match ch {
        '|' => {
            connect_north_south(current, cols, graph, (pos_x, pos_y));
        }
        '-' => {
            connect_east_west(current, cols, graph, (pos_x, pos_y));
        }
        'L' => {
            connect_north_east(current, cols, graph, (pos_x, pos_y));
        }
        'J' => {
            connect_north_west(current, cols, graph, (pos_x, pos_y));
        }
        '7' => {
            connect_south_west(current, cols, graph, (pos_x, pos_y));
        }
        'F' => {
            connect_south_east(current, cols, graph, (pos_x, pos_y));
        }
        _ => {}
    }
}

fn connect_south_east(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // F is a 90-degree bend connecting south and east.
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current + cols, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current + cols, 1, (pos_x, pos_y)));
    }
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current, current + 1, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current + 1, current, 1, (pos_x, pos_y)));
    }
}

fn connect_south_west(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // 7 is a 90-degree bend connecting south and west.
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current, current + cols, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current + cols, current, 1, (pos_x, pos_y)));
    }
    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current - 1, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current - 1, 1, (pos_x, pos_y)));
    }
}

fn connect_north_west(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // J is a 90-degree bend connecting north and west.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current - cols, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current - cols, 1, (pos_x, pos_y)));
    }
    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current, current - 1, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current - 1, current, 1, (pos_x, pos_y)));
    }
}

fn connect_north_east(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // L is a 90-degree bend connecting north and east.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current, current - cols, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current - cols, current, 1, (pos_x, pos_y)));
    }
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current + 1, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current + 1, 1, (pos_x, pos_y)));
    }
}

fn connect_east_west(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // - is a horizontal pipe connecting east and west.
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current + 1, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current + 1, 1, (pos_x, pos_y)));
    }

    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current, current - 1, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current - 1, current, 1, (pos_x, pos_y)));
    }
}

fn connect_north_south(
    current: usize,
    cols: usize,
    graph: &mut Graph,
    (pos_x, pos_y): (usize, usize),
) {
    // | is a vertical pipe connecting north and south.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current - cols, current, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current, current - cols, 1, (pos_x, pos_y)));
    }
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current, current + cols, 1, (pos_x, pos_y)));
        graph.add_edge(Edge::new(current + cols, current, 1, (pos_x, pos_y)));
    }
}

fn is_cyclic_util(
    start: usize,
    visited: &mut Vec<bool>,
    graph: &Graph,
    path: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    let mut stack = vec![(start, None)];

    while let Some((current, parent)) = stack.pop() {
        if visited[current] {
            let cycle_start = path.iter().rposition(|&x| x == current).unwrap();
            return Some(path[cycle_start..].to_vec());
        }

        visited[current] = true;
        path.push(current);

        if let Some(neighbors) = graph.neighbors(current) {
            for neighbor in neighbors {
                if Some(neighbor) != parent {
                    stack.push((neighbor, Some(current)));
                }
            }
        }
    }

    None
}

fn find_cycle(graph: &Graph, start: usize) -> Option<Vec<usize>> {
    let mut visited = vec![false; graph.size];

    let mut path = Vec::new();
    if !visited[start] {
        if let Some(cycle) = is_cyclic_util(start, &mut visited, graph, &mut path) {
            return Some(cycle);
        }
    }

    None
}
