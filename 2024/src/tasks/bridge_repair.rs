use crate::tasks::utils::read_file_bridge_repair_data;

pub fn run() -> (u64, u64) {
  let inputs = read_file_bridge_repair_data("./inputs/bridge_repair.txt").unwrap();
  let mut sum = 0;
  let mut concat_sum = 0;

  for input in inputs {
    if has_valid_terms(input.0, input.1.clone(), true) {
      concat_sum += input.0;
    }
    if has_valid_terms(input.0, input.1, false) {
      sum += input.0;
    }
  }

  (sum, concat_sum)
}

fn has_valid_terms(result: u64, terms: Vec<u64>, handle_concat: bool) -> bool {
  let operations = generate_operations(terms.len() - 1, handle_concat);

  for operation in operations {
    if let Some(value) = has_valid_terms_operation(&terms, operation, result, handle_concat) {
      return value;
    }
  }

  false
}

fn has_valid_terms_operation(
  terms: &Vec<u64>,
  operation: Vec<String>,
  result: u64,
  handle_concat: bool
) -> Option<bool> {
  let mut total = terms[0];
  for i in 1..terms.len() {
    if operation[i - 1] == "*" {
      total *= terms[i];
    } else if operation[i - 1] == "+" {
      total += terms[i];
    } else if handle_concat && operation[i - 1] == "||" {
      let power = 10u64.pow(terms[i].to_string().len() as u32);
      total = total * power + terms[i];
    }

    if total > result {
      break;
    }
  }

  if total == result {
    return Some(true);
  }
  None
}

fn generate_operations(n: usize, handle_concat: bool) -> Vec<Vec<String>> {
  let mut operations = Vec::new();
  let mut current = vec![String::new(); n];
  generate_operations_recursive(n, 0, &mut current, &mut operations, handle_concat);
  operations
}

fn generate_operations_recursive(
  n: usize,
  index: usize,
  current: &mut Vec<String>,
  operations: &mut Vec<Vec<String>>,
  handle_concat: bool
) {
  if index == n {
    operations.push(current.clone());
    return;
  }

  current[index] = "+".to_owned();
  generate_operations_recursive(n, index + 1, current, operations, handle_concat);

  current[index] = "*".to_owned();
  generate_operations_recursive(n, index + 1, current, operations, handle_concat);

  if handle_concat {
    current[index] = "||".to_owned();
    generate_operations_recursive(n, index + 1, current, operations, handle_concat);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    assert_eq!(run(), (1985268524462, 150077710195188));
  }

  #[test]
  fn test_has_valid_terms() {
    assert_eq!(has_valid_terms(7290, vec![6, 8, 6, 15], true), true);
  }
}
