use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    for part in &parts[1..] {
        let mut processed_indices = HashSet::new();

        for rule_line in part.lines().skip(1) {
            let parts: Vec<&str> = rule_line.split_whitespace().collect();

            let to = parts[0].parse::<i64>().unwrap();
            let from = parts[1].parse::<i64>().unwrap();
            let range = parts[2].parse::<i64>().unwrap();

            for (n, seed) in seeds.iter_mut().enumerate() {
                if !processed_indices.contains(&n) && *seed >= from && *seed < from + range {
                    let diff = *seed - from;

                    *seed = to + diff;

                    processed_indices.insert(n);
                }
            }
        }
    }

    seeds.sort();

    println!("{:?}", seeds[0]);

    Ok(())
}
