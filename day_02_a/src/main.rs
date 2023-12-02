use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let mut res = 0;

    'outer: for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let game_nr = parts[0].split(' ').last().unwrap().parse::<i32>().unwrap();

        for turn in parts[1].split(';') {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            for info in turn.split(',') {
                let parts: Vec<&str> = info.trim().split(' ').collect();

                let count = parts[0].parse::<i32>().unwrap();
                match parts[1] {
                    "red" => r += count,
                    "green" => g += count,
                    "blue" => b += count,
                    _ => panic!(""),
                }
            }

            if r > max_r || g > max_g || b > max_b {
                continue 'outer;
            }
        }

        res += game_nr;
    }

    println!("{res}");

    Ok(())
}
