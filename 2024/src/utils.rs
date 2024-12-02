use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_input(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
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
