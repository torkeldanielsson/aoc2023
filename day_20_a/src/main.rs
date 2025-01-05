use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

#[derive(Debug)]
enum TargetType {
    Conjunction,
    FlipFlop,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../test1");

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

        if parts[0] == "broadcaster" {
            continue;
        }

        let key = &parts[0][1..];

        for destination in &destinations {
            if let Some(conjunction) = conjunctions.get_mut(destination) {
                conjunction.insert(key, false);
                target_type.insert(*destination, TargetType::Conjunction);
            }
            if flip_flops.get_mut(destination).is_some() {
                target_type.insert(*destination, TargetType::FlipFlop);
            }
        }

        targets.insert(key, destinations);
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        low_pulses += 1; // push button

        let mut active: Vec<(&str, &str, bool)> = broadcaster_targets
            .iter()
            .map(|v| ("broadcaster", *v, false))
            .collect();

        while !active.is_empty() {
            for a in &active {
                if a.2 {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
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
                            new_active.push((a.1, *target, all_high));
                        }
                    }
                    TargetType::FlipFlop => {
                        if a.2 {
                            let state = !flip_flops[a.1];
                            *flip_flops.get_mut(a.1).unwrap() = state;
                            for target in &targets[a.1] {
                                new_active.push((a.1, *target, state));
                            }
                        }
                    }
                }
            }
            active = new_active;
        }
    }

    println!("low_pulses: {low_pulses}, high_pulses: {high_pulses}");

    Ok(())
}
