use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i32> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let distances: Vec<i32> = lines[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut res = 1;

    for i in 0..times.len() {
        let time = times[i];
        let best_distance = distances[i];

        let mut ways_to_win = 0;

        for speed in 1..time {
            let t = time - speed;
            let distance = t * speed;
            if distance > best_distance {
                ways_to_win += 1;
            }
        }
        res *= ways_to_win;
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
