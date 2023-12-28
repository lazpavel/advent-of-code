use crate::utils::input_utils;

pub fn execute(input_path: &std::path::Path) -> u128 {
    let input = input_utils::read_file_data(input_path, true);
    let tokens = input[0].split(",").collect::<Vec<&str>>();

    let values = process_tokens(&tokens);

    values.iter().map(|&v| v as u128).sum()
}

fn process_tokens(tokens: &[&str]) -> Vec<u8> {
    let mut values = Vec::new();

    for token in tokens {
        let value = process_token(token);
        values.push(value);
    }

    values
}

fn process_token(token: &str) -> u8 {
    let mut value: u8 = 0;
    let chars = token.chars();
    
    for c in chars {
        let current_char_hash = c as u8;
        value = value.wrapping_add(current_char_hash).wrapping_mul(17);
    }

    value
}