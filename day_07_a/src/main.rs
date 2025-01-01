use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test");

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let score
    }

    Ok(())
}
