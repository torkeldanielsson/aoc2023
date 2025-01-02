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
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let map = Map::new(input);

    let mut visited = FxHashSet::default();
    let pos = ivec2(0, 0);
    let dir = ivec2(1, 0);

    traverse(pos, dir, &map, &mut visited);

    let mut visited_positions = FxHashSet::default();

    for vp in visited {
        visited_positions.insert(vp.0);
    }

    println!(
        "res: {}, {} us",
        visited_positions.len(),
        t.elapsed().as_micros()
    );

    Ok(())
}

fn traverse(pos: IVec2, dir: IVec2, map: &Map, visited: &mut FxHashSet<(IVec2, IVec2)>) {
    if visited.contains(&(pos, dir)) {
        return;
    }

    if pos.x < 0 || pos.y < 0 || pos.x >= map.w || pos.y >= map.h {
        return;
    }

    visited.insert((pos, dir));

    match map.at(&pos) {
        '.' => {
            traverse(pos + dir, dir, map, visited);
        }
        '|' => {
            if dir.x == 0 {
                traverse(pos + dir, dir, map, visited);
            } else {
                traverse(pos + ivec2(0, 1), ivec2(0, 1), map, visited);
                traverse(pos + ivec2(0, -1), ivec2(0, -1), map, visited);
            }
        }
        '-' => {
            if dir.y == 0 {
                traverse(pos + dir, dir, map, visited);
            } else {
                traverse(pos + ivec2(1, 0), ivec2(1, 0), map, visited);
                traverse(pos + ivec2(-1, 0), ivec2(-1, 0), map, visited);
            }
        }
        '/' => match (dir.x, dir.y) {
            (1, 0) => traverse(pos + ivec2(0, -1), ivec2(0, -1), map, visited),
            (-1, 0) => traverse(pos + ivec2(0, 1), ivec2(0, 1), map, visited),
            (0, 1) => traverse(pos + ivec2(-1, 0), ivec2(-1, 0), map, visited),
            (0, -1) => traverse(pos + ivec2(1, 0), ivec2(1, 0), map, visited),
            _ => panic!(),
        },
        '\\' => match (dir.x, dir.y) {
            (1, 0) => traverse(pos + ivec2(0, 1), ivec2(0, 1), map, visited),
            (-1, 0) => traverse(pos + ivec2(0, -1), ivec2(0, -1), map, visited),
            (0, 1) => traverse(pos + ivec2(1, 0), ivec2(1, 0), map, visited),
            (0, -1) => traverse(pos + ivec2(-1, 0), ivec2(-1, 0), map, visited),
            _ => panic!(),
        },
        _ => panic!(),
    }
}
