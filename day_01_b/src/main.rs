use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = fs::read_to_string("input")?;

    input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

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
