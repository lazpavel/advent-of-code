use std::collections::{HashMap, HashSet};

use crate::utils::read_file_antenna_map;

pub fn run() -> (usize, usize) {
  let map = read_file_antenna_map("./inputs/resonant_collinearity.txt").unwrap();
  run_map(map)
}

fn run_map(map: Vec<Vec<u8>>) -> (usize, usize) {
  let pairs = build_antenna_pairs(&map);

  let mut total = 0;
  let mut unique_antidotes = HashSet::new();
  for pair in pairs.clone() {
    let antidotes = get_map_antidotes(&map, pair, &mut unique_antidotes, false);
    total += antidotes;
  }

  let mut continuous_total = 0;
  let mut unique_antidotes = HashSet::new();
  for pair in pairs.clone() {
    let antidotes = get_map_antidotes(&map, pair, &mut unique_antidotes, true);
    continuous_total += antidotes;
  }

  (total, continuous_total)
}

fn get_map_antidotes(
  map: &Vec<Vec<u8>>,
  pair: (u8, (usize, usize), (usize, usize)),
  unique_antidotes: &mut HashSet<(usize, usize)>,
  is_continuous: bool,
) -> usize {
  let (_, (x1, y1), (x2, y2)) = pair;
  let dx = x2 as isize - x1 as isize;
  let dy = y2 as isize - y1 as isize;

  let mut new_points = Vec::new();

  if is_continuous {
    // Generate points in the negative direction from x1, y1
    let mut nx = x1 as isize - dx;
    let mut ny = y1 as isize - dy;
    while nx >= 0 && ny >= 0 && (nx as usize) < map[0].len() && (ny as usize) < map.len() {
      new_points.push((nx, ny));
      nx -= dx;
      ny -= dy;
    }

    // Generate points in the positive direction from x1, y1
    let mut nx = x1 as isize + dx;
    let mut ny = y1 as isize + dy;
    while nx >= 0 && ny >= 0 && (nx as usize) < map[0].len() && (ny as usize) < map.len() {
      new_points.push((nx, ny));
      nx += dx;
      ny += dy;
    }

    // Generate points in the negative direction from x2, y2
    let mut nx = x2 as isize - dx;
    let mut ny = y2 as isize - dy;
    while nx >= 0 && ny >= 0 && (nx as usize) < map[0].len() && (ny as usize) < map.len() {
      new_points.push((nx, ny));
      nx -= dx;
      ny -= dy;
    }

    // Generate points in the positive direction from x2, y2
    let mut nx = x2 as isize + dx;
    let mut ny = y2 as isize + dy;
    while nx >= 0 && ny >= 0 && (nx as usize) < map[0].len() && (ny as usize) < map.len() {
      new_points.push((nx, ny));
      nx += dx;
      ny += dy;
    }
  } else {
    new_points.push((x1 as isize - dx, y1 as isize - dy));
    new_points.push((x2 as isize + dx, y2 as isize + dy));
  }

  let mut antidotes = 0;
  for &(nx, ny) in &new_points {
    if nx >= 0 && ny >= 0 {
      let (nx, ny) = (nx as usize, ny as usize);
      if nx < map[0].len() && ny < map.len() {
        if unique_antidotes.insert((nx, ny)) {
          antidotes += 1;
        }
      }
    }
  }

  antidotes
}

fn build_antenna_pairs(map: &Vec<Vec<u8>>) -> Vec<(u8, (usize, usize), (usize, usize))> {
  let mut coordinates: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

  for (y, row) in map.iter().enumerate() {
    for (x, &cell) in row.iter().enumerate() {
      if cell != b'.' && cell != b' ' {
        coordinates.entry(cell).or_insert(Vec::new()).push((x, y));
      }
    }
  }

  let mut pairs = Vec::new();
  for (&char, coords) in &coordinates {
    for i in 0..coords.len() {
      for j in i + 1..coords.len() {
        pairs.push((char, (coords[i].0, coords[i].1), (coords[j].0, coords[j].1)));
      }
    }
  }

  pairs
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (396, 1200));
  }

  #[test]
  fn test_pairs() {
    let map = vec![
      b"............".to_vec(),
      b"........0...".to_vec(),
      b".....0......".to_vec(),
      b".......0....".to_vec(),
      b"....0.......".to_vec(),
      b"......A.....".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"........A...".to_vec(),
      b".........A..".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
    ];

    let pairs = build_antenna_pairs(&map);
    assert_eq!(pairs.len(), 9);
  }

  #[test]
  fn test_get_map_antidotes() {
    let map = vec![
      b"..0.0.......".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
      b"............".to_vec(),
    ];

    let pair = (b'0', (0, 2), (0, 4));
    let mut unique_antidotes = HashSet::new();
    let antidotes = get_map_antidotes(&map, pair, &mut unique_antidotes, true);
    assert_eq!(antidotes, 4);
  }

  #[test]
  fn test_run_map() {
    let map = vec![
      b"T.........".to_vec(),
      b"...T......".to_vec(),
      b".T........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
      b"..........".to_vec(),
    ];

    let antidotes = run_map(map);
    assert_eq!(antidotes, (3, 9));
  }
}
