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
                if self.at(&ivec2(x, y)) == 'S' {
                    return ivec2(x, y);
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

    let mut p = FxHashSet::default();

    p.insert(map.start_pos());

    for _ in 0..64 {
        let mut np = FxHashSet::default();

        for p in p {
            for n in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                let n = p + n;
                if map.at(&n) != '#' {
                    np.insert(n);
                }
            }
        }
        p = np;
    }

    println!("res: {}, {} us", p.len(), t.elapsed().as_micros());

    Ok(())
}
