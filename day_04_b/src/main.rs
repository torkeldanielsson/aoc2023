use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut card_counts = HashMap::new();

    for (n, _line) in input.lines().enumerate() {
        card_counts.insert(n, 1_i64);
    }

    for (line_nr, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split(':').collect();
        let line = parts[1];

        let parts: Vec<&str> = line.split('|').collect();

        let mut winning_numbers = HashSet::new();

        for winning_number in parts[0].split(' ') {
            let winning_number = winning_number.trim();
            if !winning_number.is_empty() {
                winning_numbers.insert(winning_number.parse::<i32>().unwrap());
            }
        }

        let mut matches = 0;

        for n in parts[1].split(' ') {
            let n = n.trim();
            if !n.is_empty() {
                let n = n.parse::<i32>().unwrap();
                if winning_numbers.contains(&n) {
                    matches += 1;
                }
            }
        }

        if matches > 0 {
            let c = card_counts[&line_nr];
            for n in (line_nr + 1)..=(line_nr + matches as usize) {
                if let Some(tmp) = card_counts.get_mut(&n) {
                    *tmp += c;
                }
            }
        }
    }

    let mut res = 0;

    for c in card_counts {
        res += c.1;
    }

    println!("{res}");

    Ok(())
}
