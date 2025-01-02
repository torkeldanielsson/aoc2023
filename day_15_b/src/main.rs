use std::{collections::HashMap, error::Error, time::Instant};

fn hash(s: &str) -> usize {
    let mut v = 0;
    for c in s.chars().map(|c| c as u8) {
        v = ((v + c as u32) * 17) % 256;
    }
    v as usize
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut boxes = vec![(Vec::new(), HashMap::new()); 256];

    for s in input.trim().split(",") {
        if s.ends_with("-") {
            let st = s.strip_suffix("-").unwrap();
            let ib = hash(st);
            let b = &mut boxes[ib];
            b.0.retain(|&x| x != st);
        } else {
            let parts: Vec<&str> = s.split("=").collect();
            let lens_num = parts[1].parse::<u8>().unwrap();
            let st = parts[0];
            let ib = hash(st);
            let b = &mut boxes[ib];
            if !b.0.contains(&st) {
                b.0.push(st);
            }
            b.1.insert(st, lens_num);
        }

        /*
        println!("After \"{s}\":");
        for i in 0..256 {
            if !boxes[i].0.is_empty() {
                print!("Box {i}: ");
                for st in &boxes[i].0 {
                    print!("[{st} {}]", boxes[i].1[st]);
                }
                println!();
            }
        }
        println!();
        */
    }

    let mut res = 0_u64;

    for (box_i, b) in boxes.iter().enumerate() {
        for (slot, lens) in b.0.iter().enumerate() {
            let focal_length = b.1[lens];
            res += (box_i as u64 + 1) * (slot as u64 + 1) * focal_length as u64;
        }
    }
    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
