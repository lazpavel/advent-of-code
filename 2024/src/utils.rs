use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn read_file_guard_map(file_path: &str) -> io::Result<Vec<Vec<u8>>> {
  let mut map = Vec::new();

  let path = Path::new(file_path);
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let row: Vec<u8> = line.into_bytes();
    map.push(row);
  }

  Ok(map)
}

pub fn read_print_queue_data(
  file_path: &str,
) -> io::Result<(HashMap<u64, Vec<u64>>, Vec<Vec<u64>>)> {
  let mut queues = Vec::new();
  let mut rules = HashMap::new();

  let path = Path::new(file_path);
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  let mut in_rules_section = true;

  for line in reader.lines() {
    let line = line?;

    if line.is_empty() {
      in_rules_section = false;
      continue;
    }

    if in_rules_section {
      let row: Vec<u64> = line.split('|').map(|s| s.parse::<u64>().unwrap()).collect();

      if rules.contains_key(&row[0]) {
        let queue: &mut Vec<u64> = rules.get_mut(&row[0]).unwrap();
        queue.push(row[1]);
      } else {
        rules.insert(row[0], vec![row[1]]);
      }
    } else {
      let row: Vec<u64> = line.split(',').map(|s| s.parse::<u64>().unwrap()).collect();
      queues.push(row);
    }
  }

  Ok((rules, queues))
}

pub fn read_matrix_file_u8(file_path: &str) -> io::Result<Vec<Vec<u8>>> {
  let mut matrix = Vec::new();

  let path = Path::new(file_path);
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let row: Vec<u8> = line.into_bytes();
    matrix.push(row);
  }

  Ok(matrix)
}

pub fn read_matrix_file_i32(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
  let mut matrix = Vec::new();

  let path = Path::new(file_path);
  let file = File::open(&path)?;
  let reader = io::BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let row: Vec<i32> = line
      .split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect();
    matrix.push(row);
  }

  Ok(matrix)
}

pub fn read_string_file(file_path: &str) -> io::Result<String> {
  let path = Path::new(file_path);
  let mut file = File::open(&path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}
