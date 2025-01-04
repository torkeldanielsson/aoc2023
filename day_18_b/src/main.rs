use std::{error::Error, time::Instant};

use glam::{i64vec2, I64Vec2};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut pos = i64vec2(0, 0);
    let mut map_lines_horizontal = Vec::new();
    let mut map_lines_vertical = Vec::new();

    let mut top_pos = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        let dist = &parts[2][2..7];
        let dir = &parts[2][7..8];

        let dist = u32::from_str_radix(dist, 16).unwrap();

        let dir = match dir {
            "3" => i64vec2(0, -1),
            "1" => i64vec2(0, 1),
            "2" => i64vec2(-1, 0),
            "0" => i64vec2(1, 0),
            _ => panic!(),
        };

        let new_pos = pos + dist as i64 * dir;

        if dir.x == 0 {
            map_lines_vertical.push((pos, new_pos));
        } else {
            map_lines_horizontal.push((pos, new_pos));
        }

        pos = new_pos;

        top_pos = pos.y.min(top_pos);
    }

    top_pos -= 1;

    let mut active_horizontal: Vec<(I64Vec2, I64Vec2)> = Vec::new();

    let mut res = 0_u64;

    loop {
        let mut next_y = i64::MAX;
        for l in &map_lines_horizontal {
            if l.0.y > top_pos && l.0.y < next_y {
                next_y = l.0.y;
            }
        }

        if next_y == i64::MAX {
            break;
        }

        let y_dist = next_y - top_pos;

        for bar in &active_horizontal {
            res += ((y_dist) * (1 + (bar.1.x - bar.0.x).abs())) as u64;
        }

        top_pos = next_y;

        'bar_loop: for bar in &map_lines_horizontal {
            let bar_min = bar.0.x.min(bar.1.x);
            let bar_max = bar.0.x.max(bar.1.x);

            if bar.0.y == top_pos {
                let mut i = 0;
                while i < active_horizontal.len() {
                    let ah_min = active_horizontal[i].0.x.min(active_horizontal[i].1.x);
                    let ah_max = active_horizontal[i].0.x.max(active_horizontal[i].1.x);

                    // removal
                    if bar_min == ah_min && bar_max == ah_max {
                        res += (ah_max - ah_min + 1) as u64;
                        active_horizontal.remove(i);
                        continue 'bar_loop;
                    }

                    // bridge
                    let mut j = 0;
                    while j < active_horizontal.len() {
                        if i == j {
                            j += 1;
                            continue;
                        }

                        let ah2_min = active_horizontal[j].0.x.min(active_horizontal[j].1.x);
                        let ah2_max = active_horizontal[j].0.x.max(active_horizontal[j].1.x);

                        if ah_max == bar_min && bar_max == ah2_min {
                            active_horizontal
                                .push((i64vec2(ah_min, top_pos), i64vec2(ah2_max, top_pos)));

                            active_horizontal.remove(i.max(j));
                            active_horizontal.remove(i.min(j));

                            continue 'bar_loop;
                        }

                        j += 1;
                    }

                    // extension
                    if ah_min == bar_min {
                        res += (bar_max - bar_min) as u64;
                        active_horizontal[i].0.x = bar_max;
                        active_horizontal[i].1.x = ah_max;
                        continue 'bar_loop;
                    }
                    if ah_min == bar_max {
                        active_horizontal[i].0.x = bar_min;
                        active_horizontal[i].1.x = ah_max;
                        continue 'bar_loop;
                    }
                    if ah_max == bar_min {
                        active_horizontal[i].0.x = bar_max;
                        active_horizontal[i].1.x = ah_min;
                        continue 'bar_loop;
                    }
                    if ah_max == bar_max {
                        res += (bar_max - bar_min) as u64;
                        active_horizontal[i].0.x = ah_min;
                        active_horizontal[i].1.x = bar_min;
                        continue 'bar_loop;
                    }

                    // split
                    if ah_min < bar_min && bar_max < ah_max {
                        res += (bar_max - bar_min - 1) as u64;

                        active_horizontal
                            .push((i64vec2(ah_min, top_pos), i64vec2(bar_min, top_pos)));
                        active_horizontal
                            .push((i64vec2(bar_max, top_pos), i64vec2(ah_max, top_pos)));

                        active_horizontal.remove(i);

                        continue 'bar_loop;
                    }

                    if ah_min < bar_min && ah_max > bar_min {
                        panic!("unexpected 1 {bar_min} {bar_max}, {ah_min} {ah_max}")
                    }
                    if ah_min < bar_max && ah_max > bar_max {
                        println!("{active_horizontal:?}");

                        panic!("unexpected 2 {bar_min} {bar_max}, {ah_min} {ah_max}, {top_pos}")
                    }

                    i += 1;
                }
                active_horizontal.push(*bar);
            }
        }
    }

    println!("{active_horizontal:?}");

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
