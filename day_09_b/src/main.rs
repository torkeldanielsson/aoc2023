use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for line in input.lines() {
        let n_0: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|v| v.parse::<i64>().unwrap()).rev()
            .collect();

        let mut nums = vec![n_0];

        let mut all_zeroes = false;
        while !all_zeroes {
            let prev = nums.last().unwrap();

            let mut n_i = Vec::new();

            all_zeroes = true;

            for p in prev.windows(2) {
                let v = p[0] - p[1];
                if v != 0 {
                    all_zeroes = false;
                }
                n_i.push(v);
            }

            nums.push(n_i);
        }

        let mut level = nums.len() - 1;
        loop {
            if level == nums.len() - 1 {
                nums[level].push(0);
            } else {
                let last_below = *nums[level + 1].last().unwrap();
                let last_on_level = *nums[level].last().unwrap();
                nums[level].push(last_on_level - last_below);
            }

            if level == 0 {
                res += nums[level].last().unwrap();
                break;
            }
            level -= 1;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
