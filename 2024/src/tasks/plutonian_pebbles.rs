use std::collections::HashMap;

use tracing::info;
use crate::tasks::utils::read_file_u64_vec;

pub fn run() -> usize {
  info!("Plutonian Pebbles");

  let input = read_file_u64_vec("./inputs/plutonian_pebbles.txt").unwrap();
  let mut cache: HashMap<String, usize> = HashMap::new();

  let mut total = 0;
  for v in input {
    total += execute_blink(v, 75, &mut cache);
    info!("{} - {}", v, total);
  }

  total
}

fn execute_blink(
  v: u64,
  n: usize,
  cache: &mut HashMap<String, usize>
) -> usize {
  if n == 0 {
    return 1;
  }

  let mut result = 0;
  let arrangements = calculate_arrangements(v);

  for val in arrangements {
    let key = val.to_string() + "_" + &(n - 1).to_string();
    match cache.get(&key) {
      Some(count) => {
        result += count
      },
      None => {
        let next_count = execute_blink(val, n - 1, cache);
        cache.insert(key, next_count);
        result += next_count;
      }
    }
  }

  result
}

fn calculate_arrangements(v: u64) -> Vec<u64> {
  let mut arrangements = Vec::with_capacity(2);
  if v == 0 {
    arrangements.push(1);
  } else if event_digits(v) {
    arrangements.push(left_half(v));
    arrangements.push(right_half(v));
  } else {
    arrangements.push(v * 2024);
  }
  arrangements
}

fn event_digits(v: u64) -> bool {
  let mut digits = 0;
  let mut num = v;
  while num > 0 {
    digits += 1;
    num /= 10;
  }
  digits % 2 == 0
}

fn left_half(v: u64) -> u64 {
  let len = (v as f64).log10() as u32 + 1;
  let half_len = len / 2;
  v / 10u64.pow(half_len)
}

fn right_half(v: u64) -> u64 {
  let len = (v as f64).log10() as u32 + 1;
  let half_len = len / 2;
  v % 10u64.pow(half_len)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), 194557);
  }
}
