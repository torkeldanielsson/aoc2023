use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut map = Vec::new();

    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        map.push(line);
    }

    // for y in 0..map.len() {
    //     for x in 0..map[0].len() {
    //         print!("{}", map[y][x]);
    //     }
    //     println!();
    // }
    // println!();

    for x in 0..map[0].len() {
        let mut y = 0;
        while map[y][x] != '.' {
            y += 1;
        }

        let mut last_open_pos = y;

        y += 1;

        while y < map.len() {
            match map[y][x] {
                '.' => {}
                '#' => {
                    while y < map.len() && map[y][x] != '.' {
                        y += 1;
                    }
                    last_open_pos = y;
                }
                'O' => {
                    map[y][x] = '.';
                    map[last_open_pos][x] = 'O';
                    y = last_open_pos;
                    while y < map.len() && map[y][x] != '.' {
                        y += 1;
                    }
                    last_open_pos = y;
                }
                _ => panic!(),
            }
            y += 1;
        }
    }

    let mut res = 0;

    for y in 0..map.len() {
        let line_points = map.len() - y;
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                res += line_points;
            }
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
