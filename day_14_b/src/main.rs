use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut map = Vec::new();

    for line in input.lines() {
        let line: Vec<char> = line.chars().collect();
        map.push(line);
    }

    let max_y = map.len();
    let max_x = map[0].len();

    let mut history = Vec::new();

    let mut i = 0;

    while i < 1000000000 {
        i += 1;

        history.push(map.clone());

        for x in 0..max_x {
            let mut y = 0;
            while map[y][x] != '.' {
                y += 1;
            }

            let mut last_open_pos = y;

            y += 1;

            while y < max_y {
                match map[y][x] {
                    '.' => {}
                    '#' => {
                        while y < max_y && map[y][x] != '.' {
                            y += 1;
                        }
                        last_open_pos = y;
                    }
                    'O' => {
                        map[y][x] = '.';
                        map[last_open_pos][x] = 'O';
                        y = last_open_pos;
                        while y < max_y && map[y][x] != '.' {
                            y += 1;
                        }
                        last_open_pos = y;
                    }
                    _ => panic!(),
                }
                y += 1;
            }
        }

        for y in 0..max_y {
            let mut x = 0;
            while map[y][x] != '.' {
                x += 1;
            }

            let mut last_open_pos = x;

            x += 1;

            while x < max_x {
                match map[y][x] {
                    '.' => {}
                    '#' => {
                        while x < max_x && map[y][x] != '.' {
                            x += 1;
                        }
                        last_open_pos = x;
                    }
                    'O' => {
                        map[y][x] = '.';
                        map[y][last_open_pos] = 'O';
                        x = last_open_pos;
                        while x < max_x && map[y][x] != '.' {
                            x += 1;
                        }
                        last_open_pos = x;
                    }
                    _ => panic!(),
                }
                x += 1;
            }
        }

        for x in 0..max_x {
            let mut y = 0;
            while map[max_y - y - 1][x] != '.' {
                y += 1;
            }

            let mut last_open_pos = y;

            y += 1;

            while y < max_y {
                match map[max_y - y - 1][x] {
                    '.' => {}
                    '#' => {
                        while y < max_y && map[max_y - y - 1][x] != '.' {
                            y += 1;
                        }
                        last_open_pos = y;
                    }
                    'O' => {
                        map[max_y - y - 1][x] = '.';
                        map[max_y - last_open_pos - 1][x] = 'O';
                        y = last_open_pos;
                        while y < max_y && map[max_y - y - 1][x] != '.' {
                            y += 1;
                        }
                        last_open_pos = y;
                    }
                    _ => panic!(),
                }
                y += 1;
            }
        }

        for y in 0..max_y {
            let mut x = 0;
            while map[y][max_x - x - 1] != '.' {
                x += 1;
            }

            let mut last_open_pos = x;

            x += 1;

            while x < max_x {
                match map[y][max_x - x - 1] {
                    '.' => {}
                    '#' => {
                        while x < max_x && map[y][max_x - x - 1] != '.' {
                            x += 1;
                        }
                        last_open_pos = x;
                    }
                    'O' => {
                        map[y][max_x - x - 1] = '.';
                        map[y][max_x - last_open_pos - 1] = 'O';
                        x = last_open_pos;
                        while x < max_x && map[y][max_x - x - 1] != '.' {
                            x += 1;
                        }
                        last_open_pos = x;
                    }
                    _ => panic!(),
                }
                x += 1;
            }
        }

        if i > 50 && i < 1000000 {
            for j in 1..=50 {
                if map == history[history.len() - j] {
                    // we found the repeat pattern (18 for input)
                    let left_to_do = 1000000000 - i;
                    let rem = left_to_do % j;
                    i = 1000000000 - rem;
                }
            }
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
