use std::{error::Error, time::Instant};

fn pattern_match(map: &[char], pattern: &[i32], solution_count: &mut i32, broken_count: i32) {
    if map.is_empty() {
        if pattern.is_empty() && broken_count == 0
            || pattern.len() == 1 && broken_count == pattern[0]
        {
            *solution_count += 1;
        }
        return;
    }

    if pattern.is_empty() && broken_count > 0 {
        return;
    }

    if !pattern.is_empty() && broken_count > pattern[0] {
        return;
    }

    if map[0] == '.' || map[0] == '?' {
        if broken_count == 0 {
            pattern_match(&map[1..], pattern, solution_count, 0);
        } else if !pattern.is_empty() && broken_count == pattern[0] {
            pattern_match(&map[1..], &pattern[1..], solution_count, 0);
        }
    }

    if (map[0] == '#' || map[0] == '?') && (pattern.is_empty() || broken_count < pattern[0]) {
        pattern_match(&map[1..], pattern, solution_count, broken_count + 1);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let map: Vec<char> = parts[0].chars().collect();

        let pattern: Vec<i32> = parts[1]
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut solution_count = 0;

        pattern_match(&map, &pattern, &mut solution_count, 0);

        res += solution_count as u64;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
