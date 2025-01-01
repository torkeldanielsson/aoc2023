use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

fn encode_as_u32(label: &str) -> u32 {
    let bytes = label.as_bytes();
    (bytes[0] as u32) | ((bytes[1] as u32) << 8) | ((bytes[2] as u32) << 16)
}

fn _decode_from_u32(encoded: u32) -> String {
    let b1 = (encoded & 0xFF) as u8;
    let b2 = ((encoded >> 8) & 0xFF) as u8;
    let b3 = ((encoded >> 16) & 0xFF) as u8;
    String::from_utf8(vec![b1, b2, b3]).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let parts: Vec<&str> = include_str!("../input").split("\n\n").collect();

    let mut map = FxHashMap::default();

    for line in parts[1].lines() {
        let a = encode_as_u32(&line[0..3]);
        let b = encode_as_u32(&line[7..10]);
        let c = encode_as_u32(&line[12..15]);

        map.insert(a, (b, c));
    }

    let mut p = encode_as_u32("AAA");
    let goal = encode_as_u32("ZZZ");
    let mut steps = 0;

    let direction: Vec<char> = parts[0].chars().collect();

    while p != goal {
        match direction[steps % direction.len()] {
            'R' => {
                p = map[&p].1;
            }
            'L' => {
                p = map[&p].0;
            }
            _ => panic!(),
        }
        steps += 1;
    }

    println!("res: {steps}, {} us", t.elapsed().as_micros());

    Ok(())
}
