use std::{error::Error, time::Instant};

fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.chars().map(|c| c as u8) {
        v = ((v + c as u32) * 17) % 256;
    }
    v as usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test");

    let mut res = 0_u64;

    for s in input.trim().split(",") {
        if s.ends_with("-") {
            println!("-: {s}");
            let s=s.strip_suffix("-").unwrap();
            
        } else {
            println!("=: {s}")
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
