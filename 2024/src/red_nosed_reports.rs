use crate::utils::parse_input;

pub fn run() -> (usize, usize) {
  let matrix = parse_input("./inputs/red_nosed_reports.txt").unwrap();
  let safe_count = matrix.iter().filter(|row| is_safe(row, 0)).count();
  let limit_safe_count = matrix.iter().filter(|row| is_safe(row, 1)).count();

  (safe_count, limit_safe_count)
}

fn is_safe(row: &[i32], limit: i32) -> bool {
  let min_distance = 1;
  let max_distance = 3;

  let is_increasing = row[0] < row[1];
  let mut unsafe_spots = 0;
  for i in 1..row.len() {
    if (is_increasing && row[i - 1] > row[i]) || (!is_increasing && row[i - 1] < row[i]) || (row[i - 1] - row[i]).abs() < min_distance || (row[i - 1] - row[i]).abs() > max_distance {
      unsafe_spots += 1;
    }
  }

  unsafe_spots <= limit
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_red_nosed_reports() {
    let (safe_count, limit_safe_count) = run();
    assert_eq!(safe_count, 371);
    assert_eq!(limit_safe_count, 426);
  }
}