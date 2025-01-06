use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

#[derive(Debug)]
enum TargetType {
    Conjunction,
    FlipFlop,
    Unknown,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut broadcaster_targets = Vec::new();
    let mut conjunctions = FxHashMap::default();
    let mut flip_flops = FxHashMap::default();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let destinations: Vec<&str> = parts[1].split(", ").collect();

        if parts[0] == "broadcaster" {
            broadcaster_targets = destinations;
            continue;
        }

        if parts[0].starts_with("%") {
            flip_flops.insert(&parts[0][1..], false);
        } else {
            conjunctions.insert(&parts[0][1..], FxHashMap::default());
        }
    }

    let mut target_type = FxHashMap::default();
    let mut targets = FxHashMap::default();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let destinations: Vec<&str> = parts[1].split(", ").collect();

        let key = &parts[0][1..];

        for destination in &destinations {
            if let Some(conjunction) = conjunctions.get_mut(destination) {
                conjunction.insert(key, false);
                target_type.insert(*destination, TargetType::Conjunction);
            } else if flip_flops.get_mut(destination).is_some() {
                target_type.insert(*destination, TargetType::FlipFlop);
            } else {
                target_type.insert(*destination, TargetType::Unknown);
            }
        }

        targets.insert(key, destinations);
    }

    let mut dl = Vec::new();
    let mut pm = Vec::new();
    let mut ks = Vec::new();
    let mut vk = Vec::new();

    for i in 0..10000000 {
        let mut active: Vec<(&str, &str, bool)> = broadcaster_targets
            .iter()
            .map(|v| ("broadcaster", *v, false))
            .collect();

        while !active.is_empty() {
            for a in &active {
                if a.1 == "rx" && !a.2 {
                    println!("{i}")
                }
            }

            let mut new_active = Vec::new();

            for a in &active {
                match target_type[a.1] {
                    TargetType::Conjunction => {
                        conjunctions.entry(a.1).and_modify(|e| {
                            e.entry(a.0).and_modify(|v| *v = a.2);
                        });
                        let mut all_high = true;
                        for v in &conjunctions[a.1] {
                            if !v.1 {
                                all_high = false;
                                break;
                            }
                        }
                        for target in &targets[a.1] {
                            new_active.push((a.1, *target, !all_high));
                        }
                    }
                    TargetType::FlipFlop => {
                        if !a.2 {
                            let state = !flip_flops[a.1];
                            *flip_flops.get_mut(a.1).unwrap() = state;
                            for target in &targets[a.1] {
                                new_active.push((a.1, *target, state));
                            }
                        }
                    }
                    TargetType::Unknown => {}
                }

                if a.0 == "dl" && a.2 {
                    dl.push(i);
                }
                if a.0 == "pm" && a.2 {
                    pm.push(i);
                }
                if a.0 == "ks" && a.2 {
                    ks.push(i);
                }
                if a.0 == "vk" && a.2 {
                    vk.push(i);
                }
            }
            active = new_active;
        }

        // for x in ["vn", "xg", "gk", "jr", "cd", "mt", "rr", "pb"] {
        //     if flip_flops[x] {
        //         print!("1 ");
        //     } else {
        //         print!("0 ");
        //     }
        // }
        // println!();

        if dl.len() >= 2 && pm.len() >= 2 && ks.len() >= 2 && vk.len() >= 2 {
            break;
        }
    }

    let sequence = [dl[1] - dl[0], pm[1] - pm[0], ks[1] - ks[0], vk[1] - vk[0]];

    let alignment = find_alignment(&sequence);

    println!("res: {alignment}, {} us", t.elapsed().as_micros());

    Ok(())
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

fn find_alignment(sequences: &[u64]) -> u64 {
    if sequences.is_empty() {
        return 0;
    }

    let mut result = sequences[0];
    for &period in sequences.iter().skip(1) {
        result = lcm(result, period);
    }
    result
}
