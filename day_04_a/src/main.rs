use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut res = 0;

    for line in input.lines() {
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

        let mut line_value = 0;

        for n in parts[1].split(' ') {
            let n = n.trim();
            if !n.is_empty() {
                let n = n.parse::<i32>().unwrap();
                if winning_numbers.contains(&n) {
                    if line_value == 0 {
                        line_value = 1;
                    } else {
                        line_value *= 2;
                    }
                }
            }
        }

        res += line_value;
    }

    println!("{res}");

    Ok(())
}
