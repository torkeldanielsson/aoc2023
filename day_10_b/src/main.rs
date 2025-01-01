use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};
use rustc_hash::FxHashSet;

struct Map<'a> {
    data: &'a [u8],
    w: i32,
    h: i32,
}

impl<'a> Map<'a> {
    fn new(data: &'a str) -> Self {
        let w = data.find('\n').unwrap() as i32;
        let h = data.chars().filter(|&c| c == '\n').count() as i32;

        Map {
            data: data.as_bytes(),
            w,
            h,
        }
    }

    fn at(&self, coord: &IVec2) -> char {
        self.data[(coord.y * (self.w + 1) + coord.x) as usize] as char
    }

    fn start_pos(&self) -> IVec2 {
        for y in 0..self.h {
            for x in 0..self.w {
                let p = ivec2(x, y);
                if self.at(&p) == 'S' {
                    return p;
                }
            }
        }
        panic!()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let map = Map::new(input);

    let mut pos = map.start_pos();
    let mut dir = ivec2(0, 1); // works for the test and for my input...

    let mut trail = FxHashSet::default();
    let mut trail_with_pos = Vec::new();
    trail.insert(pos);
    trail_with_pos.push((pos, dir));

    // left is -1, right +1
    let mut turn_sum = 0;

    loop {
        pos += dir;
        let map_at = map.at(&pos);

        if map_at == 'S' {
            break;
        }

        if map_at == 'L' {
            if dir == ivec2(0, 1) {
                dir = ivec2(1, 0);
                turn_sum -= 1;
            } else if dir == ivec2(-1, 0) {
                dir = ivec2(0, -1);
                turn_sum += 1;
            } else {
                panic!();
            }
        } else if map_at == 'J' {
            if dir == ivec2(0, 1) {
                dir = ivec2(-1, 0);
                turn_sum += 1;
            } else if dir == ivec2(1, 0) {
                dir = ivec2(0, -1);
                turn_sum -= 1;
            } else {
                panic!();
            }
        } else if map_at == '7' {
            if dir == ivec2(0, -1) {
                dir = ivec2(-1, 0);
                turn_sum -= 1;
            } else if dir == ivec2(1, 0) {
                dir = ivec2(0, 1);
                turn_sum += 1;
            } else {
                panic!();
            }
        } else if map_at == 'F' {
            if dir == ivec2(-1, 0) {
                dir = ivec2(0, 1);
                turn_sum -= 1;
            } else if dir == ivec2(0, -1) {
                dir = ivec2(1, 0);
                turn_sum += 1;
            } else {
                panic!();
            }
        } else if !(map_at == '|' || map_at == '-') {
            panic!();
        }

        trail.insert(pos);
        trail_with_pos.push((pos, dir));
    }

    let mut inner_positions = FxHashSet::default();

    for (pos, dir) in trail_with_pos {
        let mut inner_dir = match (dir.x, dir.y) {
            (1, 0) => ivec2(0, 1),
            (-1, 0) => ivec2(0, -1),
            (0, 1) => ivec2(-1, 0),
            (0, -1) => ivec2(1, 0),
            _ => panic!(),
        };
        if turn_sum < 0 {
            inner_dir = -inner_dir;
        }
        let inner_pos = pos + inner_dir;
        if !trail.contains(&inner_pos) {
            fill(inner_pos, &trail, &mut inner_positions);
        }
    }

    println!(
        "res: {}, {} us",
        inner_positions.len(),
        t.elapsed().as_micros()
    );

    Ok(())
}

fn fill(pos: IVec2, trail: &FxHashSet<IVec2>, inner_positions: &mut FxHashSet<IVec2>) {
    inner_positions.insert(pos);

    for n in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
        let np = pos + n;
        if !trail.contains(&np) && !inner_positions.contains(&np) {
            fill(np, trail, inner_positions);
        }
    }
}
