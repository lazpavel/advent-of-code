use std::{cmp::max, vec};

use crate::{
    data_structures::graph::{Edge, Graph},
    utils::input_utils,
};

pub fn execute(input_path: &std::path::Path) -> usize {
    let raw_game_data = input_utils::read_file_data(input_path);
    let chars = vec!['|', '-', 'L', 'J', '7', 'F'];
    let cols = raw_game_data[0].len() * 2;

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
            Some(cycle) => max_cycle_length = max(max_cycle_length, cycle.len() / 4),
            None => {}
        }
    }

    max_cycle_length
}

fn build_graph(raw_game_data: &[String], start_char: char) -> (usize, Graph) {
    let mut start = 0;
    let rows = raw_game_data.len() * 2;
    let cols = raw_game_data[0].len() * 2;

    let mut graph: Graph = Graph::new(rows * cols);
    let mut row = 0;

    for line in raw_game_data {
        let mut col = 0;
        for mut ch in line.chars() {
            let current = row * cols + col;

            if current >= graph.size {
                break;
            }

            if ch == 'S' {
                start = current;
                ch = start_char;
            }
            connect_graph(ch, current, cols, &mut graph);

            col += 2;
        }

        row += 2;
    }

    (start, graph)
}

fn connect_graph(ch: char, current: usize, cols: usize, graph: &mut Graph) {
    match ch {
        '|' => {
            connect_north_south(current, cols, graph);
        }
        '-' => {
            connect_east_west(current, cols, graph);
        }
        'L' => {
            connect_north_east(current, cols, graph);
        }
        'J' => {
            connect_north_west(current, cols, graph);
        }
        '7' => {
            connect_south_west(current, cols, graph);
        }
        'F' => {
            connect_south_east(current, cols, graph);
        }
        _ => {}
    }
}

fn connect_south_east(current: usize, cols: usize, graph: &mut Graph) {
    // F is a 90-degree bend connecting south and east.
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current + cols, current, 1));
        graph.add_edge(Edge::new(current, current + cols, 1));
    }
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current, current + 1, 1));
        graph.add_edge(Edge::new(current + 1, current, 1));
    }
}

fn connect_south_west(current: usize, cols: usize, graph: &mut Graph) {
    // 7 is a 90-degree bend connecting south and west.
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current, current + cols, 1));
        graph.add_edge(Edge::new(current + cols, current, 1));
    }
    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current - 1, current, 1));
        graph.add_edge(Edge::new(current, current - 1, 1));
    }
}

fn connect_north_west(current: usize, cols: usize, graph: &mut Graph) {
    // J is a 90-degree bend connecting north and west.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current - cols, current, 1));
        graph.add_edge(Edge::new(current, current - cols, 1));
    }
    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current, current - 1, 1));
        graph.add_edge(Edge::new(current - 1, current, 1));
    }
}

fn connect_north_east(current: usize, cols: usize, graph: &mut Graph) {
    // L is a 90-degree bend connecting north and east.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current, current - cols, 1));
        graph.add_edge(Edge::new(current - cols, current, 1));
    }
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current + 1, current, 1));
        graph.add_edge(Edge::new(current, current + 1, 1));
    }
}

fn connect_east_west(current: usize, cols: usize, graph: &mut Graph) {
    // - is a horizontal pipe connecting east and west.
    if current + 1 < graph.size && (current + 1) % cols != 0 {
        graph.add_edge(Edge::new(current + 1, current, 1));
        graph.add_edge(Edge::new(current, current + 1, 1));
    }

    if current as i32 - 1 >= 0 && current - 1 < graph.size && current % cols != 0 {
        graph.add_edge(Edge::new(current, current - 1, 1));
        graph.add_edge(Edge::new(current - 1, current, 1));
    }
}

fn connect_north_south(current: usize, cols: usize, graph: &mut Graph) {
    // | is a vertical pipe connecting north and south.
    if current as i32 - cols as i32 >= 0 && current - cols < graph.size {
        graph.add_edge(Edge::new(current - cols, current, 1));
        graph.add_edge(Edge::new(current, current - cols, 1));
    }
    if current + cols < graph.size {
        graph.add_edge(Edge::new(current, current + cols, 1));
        graph.add_edge(Edge::new(current + cols, current, 1));
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
