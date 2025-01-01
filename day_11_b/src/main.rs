use std::{error::Error, time::Instant};

use glam::ivec2;

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut galaxies = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(ivec2(x as i32, y as i32));
            }
            max_x = max_x.max(x as i32 + 1);
        }
        max_y = max_y.max(y as i32 + 1);
    }

    let fill_amount = 1000000;

    {
        let mut x = 0;
        'x_loop: loop {
            for g in &galaxies {
                if g.x == x {
                    x += 1;
                    continue 'x_loop;
                }
            }

            max_x += fill_amount - 1;

            for g in galaxies.iter_mut() {
                if g.x > x {
                    g.x += fill_amount - 1;
                }
            }
            x += fill_amount;
            if x >= max_x {
                break;
            }
        }
    }
    {
        let mut y = 0;
        'y_loop: loop {
            for g in &galaxies {
                if g.y == y {
                    y += 1;
                    continue 'y_loop;
                }
            }

            max_y += fill_amount - 1;

            for g in galaxies.iter_mut() {
                if g.y > y {
                    g.y += fill_amount - 1;
                }
            }
            y += fill_amount;
            if y >= max_y {
                break;
            }
        }
    }

    let mut res = 0;

    for i in 0..galaxies.len() {
        let pi = galaxies[i];
        for pj in galaxies.iter().skip(i + 1) {
            let diff = pi - pj;
            res += (diff.x.abs() + diff.y.abs()) as u64;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
