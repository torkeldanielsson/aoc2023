use std::{collections::HashMap, error::Error, time::Instant};

#[derive(Hash, Eq, PartialEq)]
struct State {
    pos: usize,
    pattern_pos: usize,
    broken_count: u64,
}

fn pattern_match(
    map: &[char],
    pattern: &[u64],
    pos: usize,
    pattern_pos: usize,
    broken_count: u64,
    cache: &mut HashMap<State, u64>,
) -> u64 {
    let state = State {
        pos,
        pattern_pos,
        broken_count,
    };
    if let Some(&count) = cache.get(&state) {
        return count;
    }

    if pos == map.len() {
        let result = if pattern_pos == pattern.len() && broken_count == 0
            || pattern_pos == pattern.len() - 1 && broken_count == pattern[pattern_pos]
        {
            1
        } else {
            0
        };
        cache.insert(state, result);
        return result;
    }

    if pattern_pos >= pattern.len() && broken_count > 0 {
        cache.insert(state, 0);
        return 0;
    }

    if pattern_pos < pattern.len() && broken_count > pattern[pattern_pos] {
        cache.insert(state, 0);
        return 0;
    }

    let mut count = 0;

    if map[pos] == '.' || map[pos] == '?' {
        if broken_count == 0 {
            count += pattern_match(map, pattern, pos + 1, pattern_pos, 0, cache);
        } else if pattern_pos < pattern.len() && broken_count == pattern[pattern_pos] {
            count += pattern_match(map, pattern, pos + 1, pattern_pos + 1, 0, cache);
        }
    }

    if (map[pos] == '#' || map[pos] == '?')
        && (pattern_pos < pattern.len() && broken_count < pattern[pattern_pos])
    {
        count += pattern_match(map, pattern, pos + 1, pattern_pos, broken_count + 1, cache);
    }

    cache.insert(state, count);
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();
    let input = include_str!("../input");
    let mut res = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let map: Vec<char> = parts[0].chars().collect();
        let pattern: Vec<u64> = parts[1]
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut large_map = Vec::with_capacity(map.len() * 5 + 4);
        let mut large_pattern = Vec::with_capacity(pattern.len() * 5);
        
        for i in 0..5 {
            large_map.extend(&map);
            if i < 4 {
                large_map.push('?');
            }
            large_pattern.extend(&pattern);
        }

        let mut cache = HashMap::new();
        let solution_count = pattern_match(
            &large_map,
            &large_pattern,
            0,
            0,
            0,
            &mut cache,
        );
        res += solution_count;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());
    Ok(())
}