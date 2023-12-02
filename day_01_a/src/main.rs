use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut res = 0;

    for line in input.lines() {
        let mut digits = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                digits.push(c.to_digit(10).unwrap());
            }
        }

        if !digits.is_empty() {
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            res += first * 10 + last;
        }
    }

    println!("{}", res);

    Ok(())
}
