use crate::utils::read_file_guard_map;

pub fn run() -> (usize, usize) {
  let mut map = read_file_guard_map("./inputs/guard_gallivant.txt").unwrap();
  let original_map = map.clone();
  let coordinates = find_guard_position(&map);

  let result = traverse_map(coordinates, &mut map);
  let count = map.iter().flatten().filter(|&&c| c == b'X').count();

  let mut obstructions = vec![];
  for i in 1..result.0.len() {
    if obstructions.contains(&(result.0[i].0, result.0[i].1)) {
      continue;
    }

    let mut modified_map = original_map.clone();
    modified_map[result.0[i].0][result.0[i].1] = b'#';
    
    let modified_path = traverse_map(coordinates, &mut modified_map);
    if modified_path.1 {
      obstructions.push((result.0[i].0, result.0[i].1));
    }
  }

  (count, obstructions.len())
}

fn traverse_map(coordinates: (usize, usize), map: &mut Vec<Vec<u8>>) -> (Vec<(usize, usize, u8)>, bool) {
  let mut path = vec![];
  let mut coordinates = Some(coordinates);
  while coordinates.is_some() {
    let (x, y) = coordinates.unwrap();
    let direction = map[x][y];
    if path.contains(&(x, y, direction)) {
      return (path, true)
    }

    path.push((x, y, direction));
    let next_coordinates = get_next_coordinates(direction, coordinates, &map);

    match next_coordinates {
      Some((i, j)) => {
        if map[i][j] == b'#' {
          map[x][y] = get_next_direction(map[x][y]);
        } else {
          map[i][j] = map[x][y];
          map[x][y] = b'X';
          coordinates = next_coordinates
        }
      }
      _ => {
        map[x][y] = b'X';
        break;
      },
    }
  }

  (path, false)
}

fn get_next_direction(direction: u8) -> u8 {
  match direction {
    b'^' => b'>',
    b'>' => b'v',
    b'v' => b'<',
    b'<' => b'^',
    _ => panic!("Invalid direction")
  }
}

fn get_next_coordinates(
  direction: u8,
  coordinates: Option<(usize, usize)>,
  map: &Vec<Vec<u8>>,
) -> Option<(usize, usize)> {
  if coordinates.is_none() {
    panic!("Invalid coordinates given");
  }
  let (i, j) = coordinates.unwrap();
  if direction == b'^' {
    if i == 0 {
      return None;
    } else {
      return Some((i - 1, j));
    }
  }

  if direction == b'>' {
    if j == map[i].len() - 1 {
      return None;
    } else {
      return Some((i, j + 1));
    }
  }

  if direction == b'v' {
    if i == map.len() - 1 {
      return None;
    } else {
      return Some((i + 1, j));
    }
  }

  if direction == b'<' {
    if j == 0 {
      return None;
    } else {
      return Some((i, j - 1));
    }
  }

  None
}

fn find_guard_position(map: &Vec<Vec<u8>>) -> (usize, usize) {
  for i in 0..map.len() {
    for j in 0..map[i].len() {
      if map[i][j] == b'^' || map[i][j] == b'>' || map[i][j] == b'<' || map[i][j] == b'v' {
        return (i, j);
      }
    }
  }

  panic!("Invalid map, guardian not found");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (4778, 1618));
  }
}
