use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn read_matrix_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
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