use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

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

            max_r = r.max(max_r);
            max_g = g.max(max_g);
            max_b = b.max(max_b);
        }

        res += max_r * max_g * max_b;
    }

    println!("{res}");

    Ok(())
}
