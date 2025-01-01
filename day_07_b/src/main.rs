use std::{cmp::Ordering, error::Error, time::Instant};

#[derive(PartialEq)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Type) -> Option<std::cmp::Ordering> {
        let self_value = match self {
            Type::FiveOfAKind => 6,
            Type::FourOfAKind => 5,
            Type::FullHouse => 4,
            Type::ThreeOfAKind => 3,
            Type::TwoPair => 2,
            Type::OnePair => 1,
            Type::HighCard => 0,
        };

        let other_value = match other {
            Type::FiveOfAKind => 6,
            Type::FourOfAKind => 5,
            Type::FullHouse => 4,
            Type::ThreeOfAKind => 3,
            Type::TwoPair => 2,
            Type::OnePair => 1,
            Type::HighCard => 0,
        };

        Some(self_value.cmp(&other_value))
    }
}

#[derive(PartialEq)]
struct Hand {
    cards: Vec<u8>,
    val: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let self_type = self.get_type();
        let other_type = other.get_type();

        match self_type.partial_cmp(&other_type) {
            Some(Ordering::Equal) => {}
            ord => {
                return ord;
            }
        }

        for i in 0..5 {
            match self.cards[i].partial_cmp(&other.cards[i]) {
                Some(Ordering::Equal) => {}
                ord => {
                    return ord;
                }
            }
        }

        panic!()
    }
}

impl Hand {
    fn new(cards: Vec<u8>, val: i32) -> Hand {
        assert!(cards.len() == 5);
        Hand { cards, val }
    }

    fn get_type(&self) -> Type {
        let mut val_counts = [0; 13];
        let mut joker_count = 0;
        for card in &self.cards {
            if *card != 0 {
                val_counts[*card as usize] += 1;
            } else {
                joker_count += 1;
            }
        }

        let mut max_val_count = *val_counts.iter().max().unwrap();
        let mut pair_count = val_counts.iter().filter(|&&count| count == 2).count();

        match joker_count {
            5 | 4 => max_val_count = 5,
            3 => {
                if pair_count == 1 {
                    pair_count = 0;
                    max_val_count = 5;
                } else {
                    max_val_count = 4;
                }
            }
            2 => match (max_val_count, pair_count) {
                (3, _) => {
                    max_val_count = 5;
                }
                (2, _) => {
                    max_val_count = 4;
                }
                _ => {
                    max_val_count = 3;
                }
            },
            1 => match (max_val_count, pair_count) {
                (4, _) => {
                    max_val_count = 5;
                }
                (3, 0) => {
                    max_val_count = 4;
                }
                (2, 2) => {
                    max_val_count = 3;
                    pair_count = 1;
                }
                (2, 1) => {
                    max_val_count = 3;
                    pair_count = 0;
                }
                _ => {
                    max_val_count = 2;
                    pair_count = 1;
                }
            },
            0 => {}
            _ => panic!(),
        }

        match (max_val_count, pair_count) {
            (5, _) => Type::FiveOfAKind,
            (4, _) => Type::FourOfAKind,
            (3, 1) => Type::FullHouse,
            (3, 0) => Type::ThreeOfAKind,
            (2, 2) => Type::TwoPair,
            (2, 1) => Type::OnePair,
            _ => Type::HighCard,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let cards: Vec<u8> = parts[0]
            .chars()
            .map(|c| match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                'J' => 0,
                _ => panic!(),
            })
            .collect();

        let val = parts[1].parse::<i32>().unwrap();

        let hand = Hand::new(cards, val);

        hands.push(hand);
    }

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut res = 0;

    for (pos, hand) in hands.iter().enumerate() {
        // println!("{} * {}", pos + 1, hand.val);
        res += (pos + 1) as u64 * hand.val as u64;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
