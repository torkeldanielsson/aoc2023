use std::{error::Error, time::Instant};

use glam::i64vec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut pos = i64vec2(0, 0);

    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let dist = &parts[2][2..7];
        let dir = &parts[2][7..8];

        let dist = u32::from_str_radix(dist, 16).unwrap();

        let dir = match dir {
            "3" => i64vec2(0, -1),
            "1" => i64vec2(0, 1),
            "2" => i64vec2(-1, 0),
            "0" => i64vec2(1, 0),
            _ => panic!(),
        };

        let new_pos = pos + dist as i64 * dir;

        res += pos.x * new_pos.y - pos.y * new_pos.x;

        res += (pos.x - new_pos.x).abs() + (pos.y - new_pos.y).abs();

        pos = new_pos;
    }

    res /= 2;
    res += 1;

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
