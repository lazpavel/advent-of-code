use crate::tasks::utils::read_string_file;
use regex::Regex;

pub fn run() -> (usize, usize) {
  let data = read_string_file("./inputs/mul_it_over.txt").unwrap();
  let re_mul = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
  let re_instruction = Regex::new(r"(do\(\)|don't\(\))").unwrap();
  let mut total = 0;
  let mut instruction_total = 0;

  let mut instructions = Vec::new();

  for cap in re_instruction.find_iter(&data) {
    let instruction = cap.as_str();
    let pos = cap.start();
    instructions.push((instruction, pos));
  }

  for cap in re_mul.captures_iter(&data) {
    let start_pos = cap.get(0).unwrap().start();
    let mut last_instruction = None;

    for &(instruction, pos) in &instructions {
      if pos < start_pos {
        last_instruction = Some(instruction);
      } else {
        break;
      }
    }

    let num1: usize = cap[1].parse().unwrap();
    let num2: usize = cap[2].parse().unwrap();
   
    if last_instruction != Some("don't()") {
      instruction_total += num1 * num2;
    }

    total += num1 * num2;
  }

  (total, instruction_total)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (183788984, 62098619));
  }
}
