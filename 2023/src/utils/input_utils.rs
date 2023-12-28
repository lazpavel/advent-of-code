use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn read_file_data(input_file_path: &std::path::Path, skip_empty_lines: bool) -> Vec<String> {
    let file = File::open(input_file_path).unwrap();
    let reader = io::BufReader::new(file);

    // Vec to store the lines
    let mut lines = Vec::new();

    for line in reader.lines() {
        let result = line.unwrap();

        if result.is_empty() && skip_empty_lines {
            continue;
        }

        lines.push(result);
    }
    lines
}