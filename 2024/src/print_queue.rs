use std::collections::{HashMap, VecDeque};

use crate::utils::read_print_queue_data;

pub fn run() -> (u64, u64) {
  let (rules, queues) = read_print_queue_data("./inputs/print_queue.txt").unwrap();
  let (valid_queues, invalid_queues) = validate_queues(&rules, &queues);

  let mut valid_queues_total = 0;
  for queue in &valid_queues {
    valid_queues_total += queue.get(queue.len() / 2).unwrap();
  }

  let mut fixed_queues = Vec::new();
  for queue in &invalid_queues {
    fixed_queues.push(topological_sort(&rules, queue).unwrap());
  }

  let mut invalid_queues_total = 0;
  for queue in &fixed_queues {
    invalid_queues_total += queue.get(queue.len() / 2).unwrap();
  }

  (valid_queues_total, invalid_queues_total)
}

fn topological_sort(
  rules: &HashMap<u64, Vec<u64>>,
  values: &Vec<u64>,
) -> Result<Vec<u64>, &'static str> {
  let mut in_degree = HashMap::new();
  let mut graph = HashMap::new();

  // Initialize the graph and in-degree count
  for &value in values {
    in_degree.insert(value, 0);
    graph.insert(value, Vec::new());
  }

  // Build the graph and update in-degree count
  for (&key, deps) in rules {
    if !values.contains(&key) {
      continue;
    }
    for &dep in deps {
      if !values.contains(&dep) {
        continue;
      }
      graph.entry(key).or_insert(Vec::new()).push(dep);
      *in_degree.entry(dep).or_insert(0) += 1;
    }
  }

  // Queue for nodes with no incoming edges
  let mut queue = VecDeque::new();
  for (&node, &degree) in &in_degree {
    if degree == 0 {
      queue.push_back(node);
    }
  }

  let mut sorted_order = Vec::new();

  // Perform topological sort
  while let Some(node) = queue.pop_front() {
    sorted_order.push(node);
    if let Some(neighbors) = graph.get(&node) {
      for &neighbor in neighbors {
        if let Some(degree) = in_degree.get_mut(&neighbor) {
          *degree -= 1;
          if *degree == 0 {
            queue.push_back(neighbor);
          }
        }
      }
    }
  }

  // Check if there was a cycle
  if sorted_order.len() != values.len() {
    return Err("There is a cycle in the rules, cannot sort");
  }

  // Ensure the sorted order only contains nodes in the original values
  sorted_order.retain(|&x| values.contains(&x));

  Ok(sorted_order)
}

fn validate_queues(
  rules: &std::collections::HashMap<u64, Vec<u64>>,
  queues: &Vec<Vec<u64>>,
) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
  let mut valid_queues = Vec::new();
  let mut invalid_queues = Vec::new();
  for queue in queues {
    if is_valid_queue(rules, queue) {
      valid_queues.push(queue.clone());
    } else {
      invalid_queues.push(queue.clone());
    }
  }

  (valid_queues, invalid_queues)
}

fn is_valid_queue(rules: &std::collections::HashMap<u64, Vec<u64>>, queue: &Vec<u64>) -> bool {
  for i in 0..queue.len() {
    if !rules.contains_key(queue.get(i).unwrap()) {
      continue;
    }
    if pre_pages_exist(rules.get(queue.get(i).unwrap()).unwrap(), queue, i) {
      return false;
    }
  }

  true
}

fn pre_pages_exist(order: &Vec<u64>, queue: &[u64], i: usize) -> bool {
  for j in 0..i {
    if order.contains(queue.get(j).unwrap()) {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (7307, 4713));
  }
}
