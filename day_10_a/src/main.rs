use std::{error::Error, time::Instant};

use glam::{ivec2, IVec2};

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

    let mut trail = Vec::new();

    loop {
        pos += dir;
        let map_at = map.at(&pos);

        if map_at == 'S' {
            break;
        }

        if map_at == 'L' {
            if dir == ivec2(0, 1) {
                dir = ivec2(1, 0);
            } else if dir == ivec2(-1, 0) {
                dir = ivec2(0, -1);
            } else {
                panic!();
            }
        } else if map_at == 'J' {
            if dir == ivec2(0, 1) {
                dir = ivec2(-1, 0);
            } else if dir == ivec2(1, 0) {
                dir = ivec2(0, -1);
            } else {
                panic!();
            }
        } else if map_at == '7' {
            if dir == ivec2(0, -1) {
                dir = ivec2(-1, 0);
            } else if dir == ivec2(1, 0) {
                dir = ivec2(0, 1);
            } else {
                panic!();
            }
        } else if map_at == 'F' {
            if dir == ivec2(-1, 0) {
                dir = ivec2(0, 1);
            } else if dir == ivec2(0, -1) {
                dir = ivec2(1, 0);
            } else {
                panic!();
            }
        } else if !(map_at == '|' || map_at == '-') {
            panic!();
        }

        trail.push(pos);
    }

    println!(
        "res: {}, {} us",
        trail.len() / 2 + 1,
        t.elapsed().as_micros()
    );

    Ok(())
}
