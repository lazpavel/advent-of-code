use std::collections::HashMap;
use itertools::Itertools;

use crate::{utils::input_utils, data_structures::graph::Graph};

pub fn execute(input_path: &std::path::Path) -> u128 {
    let input = input_utils::read_file_data(input_path, true);
    let graph = build_graph(&input);

    // println!("{:?}", graph);

    let edges = graph.get_undirected_edges();
    let combinations = edges.iter().combinations(3);

    // let clusters = find_clusters(&graph);
    // println!("{:?}", clusters);

    for combination in combinations {
        let mut graph_clone = graph.clone();

        graph_clone.remove_undirected_edge(combination[0].0, combination[0].1);
        graph_clone.remove_undirected_edge(combination[1].0, combination[1].1);
        graph_clone.remove_undirected_edge(combination[2].0, combination[2].1);

        let clusters = find_clusters(&graph_clone);
        if clusters.len() == 2 {
            return clusters[0].len() as u128 * clusters[1].len() as u128;
        }
    }

    0
}

fn build_graph(input: &[String]) -> Graph<u32, u32, String> {
    let mut graph = Graph::<u32, u32, String>::new();
    let mut hash_map = HashMap::<String, u32>::new();

    let mut node_id = 0;
    for line in input {
        let parts = line.split(":").collect::<Vec<&str>>();
        let from_label = parts[0].trim();
        let from_neighbors = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
        
        let from_id = if let Some(id) = hash_map.get(from_label) {
            *id
        } else {
            hash_map.insert(from_label.to_string(), node_id);
            node_id += 1;
            node_id - 1
        };

        for neighbor in from_neighbors {
            let neighbor_id = if let Some(id) = hash_map.get(neighbor) {
                *id
            } else {
                hash_map.insert(neighbor.to_string(), node_id);
                node_id += 1;
                node_id - 1
            };

            graph.add_node(from_id, from_label.to_string());
            graph.add_node(neighbor_id, neighbor.to_string());
            
            graph.add_undirected_edge(from_id, neighbor_id, 1)
        }
    }
    graph
}

fn dfs(node: u32, visited: &mut Vec<bool>, graph: &Graph<u32, u32, String>, cluster: &mut Vec<u32>) {
    visited[node as usize] = true;
    cluster.push(node);

    for neighbor in graph.neighbors(node).unwrap() {
        if !visited[neighbor.0 as usize] {
            dfs(neighbor.0, visited, graph, cluster);
        }
    }
}

fn find_clusters(graph: &Graph<u32, u32, String>) -> Vec<Vec<u32>> {
    let mut visited = vec![false; graph.size()];
    let mut clusters = Vec::new();

    for node in graph.nodes.keys() {
        if !visited[*node as usize] {
            let mut cluster = Vec::new();
            dfs(*node as u32, &mut visited, graph, &mut cluster);
            clusters.push(cluster);
        }
    }

    clusters
}