use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u32 {
        match *self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5, 
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::Pair => 2,
            HandType::HighCard => 1,
        }
    }
}

#[derive(Debug)]
struct Hand {
    value: String,
    hand_type: HandType,
    bid: u32,
}

fn get_card_value(card: char) -> u32 {
    match card {
        // for part 1, set J to 11
        'J' => 1,
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

pub fn execute(input_path: &std::path::Path) -> u32 {
    let raw_game_data = crate::utils::input_utils::read_file_data(input_path, true);
    let mut hands = get_hands(&raw_game_data);

    hands.sort_by(|a, b| {
        match a.hand_type.value().cmp(&b.hand_type.value()) {
            Ordering::Equal => a.value.chars().into_iter().zip(b.value.chars().into_iter())
                .map(|(card_a, card_b)| get_card_value(card_a).cmp(&get_card_value(card_b)))
                .find(|&order| order != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            ordering => ordering,
        }
    });

    let mut total_score = 0;
    for (index, hand) in hands.iter().enumerate() {
        total_score += (index + 1) as u32 * hand.bid;
    }

    total_score
}

fn get_hands(raw_game_data: &[String]) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in raw_game_data {
        let mut data = line.split_whitespace();
        let value = data.next().unwrap();
        let bid = data.next().unwrap().parse::<u32>().unwrap();
        let hand = Hand {
            value: value.to_string(),
            bid,
            hand_type: get_hand_type(value),
        };

        hands.push(hand);
    }

    hands
}

fn get_hand_type(cards: &str) -> HandType {
    let mut cards_map = HashMap::new();

    for card in cards.chars() {
        if cards_map.contains_key(&card) {
            *cards_map.get_mut(&card).unwrap() += 1;
        } else {
            cards_map.insert(card, 1);
        }
    }

    // comment this block for part 1
    let j_count = cards.matches('J').count();
    if j_count > 0 && cards_map.len() > 1 {
        cards_map.remove(&'J');
        if let Some((card, _)) = cards_map.iter().max_by_key(|&(_, count)| count) {
            let card = *card;
            if let Some(count) = cards_map.get_mut(&card) {
                *count += j_count;
            }
        }
    }
    
    if cards_map.len() == 1 {
        HandType::FiveOfAKind
    } else if cards_map.len() == 2 {
        if cards_map.values().any(|&x| x == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else if cards_map.len() == 3 {
        if cards_map.values().any(|&x| x == 3) {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if cards_map.len() == 4 {
        HandType::Pair
    } else {
        HandType::HighCard
    }
}