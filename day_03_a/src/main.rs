use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut symbols = HashSet::new();

    let input = fs::read_to_string("input")?;

    for (line_nr, line) in input.lines().enumerate() {
        for (char_nr, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.insert((line_nr as i32, char_nr as i32));
            }
        }
    }

    let mut res = 0;

    for (line_nr, line) in input.lines().enumerate() {
        let mut tmp_num = String::new();

        let mut is_ok = false;

        for (char_nr, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                tmp_num = format!("{tmp_num}{char}");
                if !is_ok {
                    for n in &[
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ] {
                        if symbols.contains(&(line_nr as i32 + n.0, char_nr as i32 + n.1)) {
                            is_ok = true;
                            break;
                        }
                    }
                }
            } else if !tmp_num.is_empty() {
                let num = tmp_num.parse::<i32>().unwrap();
                if is_ok {
                    res += num;
                }
                tmp_num = String::new();
                is_ok = false;
            }
        }

        if !tmp_num.is_empty() {
            let num = tmp_num.parse::<i32>().unwrap();
            if is_ok {
                res += num;
            }
        }
    }

    println!("{res}");

    Ok(())
}
