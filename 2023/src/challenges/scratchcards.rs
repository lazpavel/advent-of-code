use crate::utils::input_utils;

struct Scratchcard {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    count: u32,
    value: u32,
}

pub fn calculate_scratchcards(scratchboard_input_path: &std::path::Path) -> (u32, u32) {
    let lines = input_utils::read_file_data(scratchboard_input_path);
    let mut scratchcards = get_scratchcards(&lines);
    let mut sum = 0;

    for scratchcard_index in 0..scratchcards.len() {
        let scratchcard = &mut scratchcards[scratchcard_index];

        let winning_numbers = &scratchcard.winning_numbers;
        let numbers = &scratchcard.numbers;

        let mut count = 0;
        for number in numbers.iter() {
            if winning_numbers.contains(number) {
                count += 1;
            }
        }

        if count > 0 {
            let base: u32 = 2;
            let exponent: u32 = count - 1;
            let current_scratchcard_count = scratchcard.count;

            scratchcard.value = 1 * base.pow(exponent);
            
            sum += scratchcard.value;

            for i in scratchcard_index + 1..scratchcard_index + 1 + (count as usize)  {
                let next_scratchcard = &mut scratchcards[i];
                next_scratchcard.count += current_scratchcard_count;
            }
        }
    }

    let mut total_count = 0;
    for scratchcard in scratchcards {
        total_count += scratchcard.count;
    }
    
    (sum, total_count)
}

fn get_scratchcards<'a>(lines: &'a Vec<String>) -> Vec<Scratchcard> {
    let mut scratchcards = vec![];

    for line in lines.iter() {
        let v: Vec<_> = line.split(':').collect();

        for i in 1..v.len() {
            let mut parts = v[i].split('|');

            let scratchcard = Scratchcard {
                winning_numbers: parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
                numbers: parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
                count: 1,
                value: 0,
            };

            scratchcards.push(scratchcard);
        }
    }

    scratchcards
}
