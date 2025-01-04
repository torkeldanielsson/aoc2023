use std::{error::Error, time::Instant};

use glam::ivec2;
use rustc_hash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test");

    let mut pos = ivec2(0, 0);
    let mut map = FxHashSet::default();

    map.insert(pos);

    let mut range_x = ivec2(0, 0);
    let mut range_y = ivec2(0, 0);

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let dist = &parts[2][2..7];
        let dir = &parts[2][7..8];

        let dist = u32::from_str_radix(dist, 16).unwrap();

        let dir = match dir {
            "3" => ivec2(0, -1),
            "1" => ivec2(0, 1),
            "2" => ivec2(-1, 0),
            "0" => ivec2(1, 0),
            _ => panic!(),
        };
    }

    println!("{range_x} {range_y}");

    Ok(())
}
