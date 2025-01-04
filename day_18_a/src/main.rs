use std::{error::Error, time::Instant};

use glam::ivec2;
use rustc_hash::FxHashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut pos = ivec2(0, 0);
    let mut map = FxHashSet::default();

    map.insert(pos);

    let mut range_x = ivec2(0, 0);
    let mut range_y = ivec2(0, 0);

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let dist = parts[1].parse::<u32>().unwrap();

        let dir = match parts[0] {
            "U" => ivec2(0, -1),
            "D" => ivec2(0, 1),
            "L" => ivec2(-1, 0),
            "R" => ivec2(1, 0),
            _ => panic!(),
        };

        for _ in 0..dist {
            pos += dir;
            map.insert(pos);

            range_x.x = pos.x.min(range_x.x);
            range_x.y = pos.x.max(range_x.y);
            range_y.x = pos.y.min(range_y.x);
            range_y.y = pos.y.max(range_y.y);
        }
    }

    let mut to_fill = Vec::new();

    to_fill.push(ivec2(1, 1));

    while let Some(p) = to_fill.pop() {
        if !map.contains(&p) {
            map.insert(p);

            to_fill.push(p + ivec2(1, 0));
            to_fill.push(p + ivec2(-1, 0));
            to_fill.push(p + ivec2(0, 1));
            to_fill.push(p + ivec2(0, -1));
        }
    }

    println!("res: {}, {} us", map.len(), t.elapsed().as_micros());

    Ok(())
}
