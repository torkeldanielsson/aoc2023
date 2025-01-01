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
        for card in &self.cards {
            val_counts[*card as usize] += 1;
        }

        let max_val_count = *val_counts.iter().max().unwrap();
        let pair_count = val_counts.iter().filter(|&&count| count == 2).count();

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
                'J' => 9,
                'T' => 8,
                '9' => 7,
                '8' => 6,
                '7' => 5,
                '6' => 4,
                '5' => 3,
                '4' => 2,
                '3' => 1,
                '2' => 0,
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
