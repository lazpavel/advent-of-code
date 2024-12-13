use crate::tasks::utils::read_matrix_file_u8;

pub fn run() -> (usize, usize) {
  let word = "XMAS";
  let data = read_matrix_file_u8("./inputs/ceres_search.txt").unwrap();

  let count = get_word_count(&data, word);
  let x_count = get_word_x_count(&data);

  (count, x_count)
}

fn get_word_x_count(data: &Vec<Vec<u8>>) -> usize {
  let mut count = 0;
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      if (is_word_diagonal_down_right_at(data, "MAS", i, j)
        || is_word_diagonal_down_right_at(data, "SAM", i, j))
        && (is_word_diagonal_down_left_at(data, "MAS", i, j + 2)
          || is_word_diagonal_down_left_at(data, "SAM", i, j + 2))
      {
        count += 1
      }
    }
  }
  count
}

fn get_word_count(data: &Vec<Vec<u8>>, word: &str) -> usize {
  let mut count = 0;
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      count += get_word_count_at(&data, word, i, j);
    }
  }
  count
}

fn get_word_count_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> usize {
  let mut count = 0;

  if is_word_up_at(data, word, i, j) {
    count += 1
  }
  if is_word_down_at(data, word, i, j) {
    count += 1
  }

  if is_word_left_at(data, word, i, j) {
    count += 1
  }

  if is_word_right_at(data, word, i, j) {
    count += 1
  }

  if is_word_diagonal_up_left_at(data, word, i, j) {
    count += 1
  }

  if is_word_diagonal_up_right_at(data, word, i, j) {
    count += 1
  }

  if is_word_diagonal_down_left_at(data, word, i, j) {
    count += 1
  }

  if is_word_diagonal_down_right_at(data, word, i, j) {
    count += 1
  }

  count
}

fn is_word_right_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if j + word.len() > data[i].len() {
    return false;
  }

  for k in 0..word.len() {
    if data[i][j + k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_left_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if j < word.len() - 1 {
    return false;
  }

  for k in 0..word.len() {
    if data[i][j - k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_down_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i + word.len() > data.len() {
    return false;
  }

  for k in 0..word.len() {
    if data[i + k][j] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_up_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i < word.len() - 1 {
    return false;
  }

  for k in 0..word.len() {
    if data[i - k][j] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_diagonal_down_right_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i + word.len() > data.len() || j + word.len() > data[i].len() {
    return false;
  }

  for k in 0..word.len() {
    if data[i + k][j + k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_diagonal_down_left_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i + word.len() > data.len() || j < word.len() - 1 {
    return false;
  }

  for k in 0..word.len() {
    if data[i + k][j - k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_diagonal_up_right_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i < word.len() - 1 || j + word.len() > data[i].len() {
    return false;
  }

  for k in 0..word.len() {
    if data[i - k][j + k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

fn is_word_diagonal_up_left_at(data: &Vec<Vec<u8>>, word: &str, i: usize, j: usize) -> bool {
  if i < word.len() - 1 || j < word.len() - 1 {
    return false;
  }

  for k in 0..word.len() {
    if data[i - k][j - k] != word.as_bytes()[k] {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ceres_search() {
    let (count, x_count) = run();
    assert_eq!(count, 2551);
    assert_eq!(x_count, 1985);
  }
}
