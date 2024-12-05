use std::collections::HashMap;

use crate::utils::read_matrix_file_i32;

fn calculate_distance(matrix: &Vec<Vec<i32>>) -> i32 {
  let mut v1: Vec<i32> = matrix.iter().map(|row| row[0]).collect();
  let mut v2: Vec<i32> = matrix.iter().map(|row| row[1]).collect();

  v1.sort();
  v2.sort();

  let mut distance = 0;
  for i in 0..v1.len() {
    distance += (v1[i] - v2[i]).abs();
  }
  distance
}

fn calculate_similarity(matrix: &Vec<Vec<i32>>) -> i32 {
  let v1: Vec<i32> = matrix.iter().map(|row| row[0]).collect();
  let v2: Vec<i32> = matrix.iter().map(|row| row[1]).collect();

  let mut counter = HashMap::new();
  for i in 0..v2.len() {
    let count = counter.entry(v2[i]).or_insert(0);
    *count += 1;
  }

  let mut similarity = 0;
  for i in 0..v1.len() {
    similarity += v1[i] * counter.get(&v1[i]).or(Some(&0)).unwrap()
  }

  similarity
}

pub fn run() -> (i32, i32) {
  let matrix = read_matrix_file_i32("./inputs/historian_hysteria.txt").unwrap();

  let distance = calculate_distance(&matrix);
  let similarity = calculate_similarity(&matrix);
  (distance, similarity)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_historian_hysteria() {
    let (distance, similarity) = run();
    assert_eq!(distance, 1722302);
    assert_eq!(similarity, 20373490);
  }
}
