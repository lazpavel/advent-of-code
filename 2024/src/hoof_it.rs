use std::collections::HashSet;

use crate::utils::read_file_matrix;

pub fn run() -> (usize, usize) {
  let map = read_file_matrix("./inputs/hoof_it.txt").unwrap();
  let starts = get_start_positions(&map);

  let mut score_by_ends = 0;
  let mut score_by_paths = 0;

  for (x, y) in starts {
    let (ends, paths) = calculate_paths((x, y), &map, 0);
    score_by_ends += ends.len();
    score_by_paths += paths;
  }
  

  (score_by_ends, score_by_paths)
}

fn calculate_paths(
  coordinates: (usize, usize),
  map: &[Vec<Option<u32>>],
  current: u32,
) -> (HashSet<(usize, usize)>, usize) {
  if current == 9 {
    let mut end = HashSet::new();
    end.insert(coordinates);
    return (end, 1);
  }

  let mut ends = HashSet::new();
  let mut total_paths = 0;
  let options = get_next_steps(coordinates, map, current + 1);
  for option in options {
    let (option_ends, paths)  = calculate_paths(option, map, current + 1);
    ends.extend(option_ends);
    total_paths += paths;
  }

  (ends, total_paths)
}

fn get_next_steps(
  coordinates: (usize, usize),
  map: &[Vec<Option<u32>>],
  next: u32,
) -> Vec<(usize, usize)> {
  let mut options = vec![];
  let (x, y) = coordinates;
  if x > 0 && map[x - 1][y].is_some() && map[x - 1][y].unwrap() == next {
    options.push((x - 1, y))
  }
  if y > 0 && map[x][y - 1].is_some() && map[x][y - 1].unwrap() == next {
    options.push((x, y - 1))
  }
  if x < map.len() - 1 && map[x + 1][y].is_some() && map[x + 1][y].unwrap() == next {
    options.push((x + 1, y))
  }
  if y < map[0].len() - 1 && map[x][y + 1].is_some() && map[x][y + 1].unwrap() == next {
    options.push((x, y + 1))
  }

  options
}

fn get_start_positions(map: &[Vec<Option<u32>>]) -> Vec<(usize, usize)> {
  map
    .iter()
    .enumerate()
    .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &val)| (i, j, val)))
    .filter(|&(_, _, val)| val == Some(0))
    .map(|(i, j, _)| (i, j))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (841, 1875));
  }
}
