use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0_u64;

    for s in input.trim().split(",") {
        let mut v = 0;
        for c in s.chars().map(|c| c as u8) {
            v = ((v + c as u32) * 17) % 256;
        }

        res += v as u64;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
