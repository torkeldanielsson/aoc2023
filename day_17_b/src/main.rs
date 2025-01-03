use glam::{ivec2, IVec2};
use rustc_hash::FxHashMap;
use std::{collections::BinaryHeap, error::Error, time::Instant};

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

    fn contains_pos(&self, coord: &IVec2) -> bool {
        coord.x >= 0 && coord.x < self.w && coord.y >= 0 && coord.y < self.h
    }

    fn at(&self, coord: &IVec2) -> i32 {
        (self.data[(coord.y * (self.w + 1) + coord.x) as usize] - b'0') as i32
    }
}

#[derive(Eq, PartialEq)]
struct State {
    heat_loss: i32,
    pos: IVec2,
    dir: IVec2,
    straight_count: i32,
}

impl State {
    fn new(heat_loss: i32, pos: IVec2, dir: IVec2, straight_count: i32) -> Self {
        State {
            heat_loss,
            pos,
            dir,
            straight_count,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let map = Map::new(input);

    let goal = ivec2(map.w - 1, map.h - 1);

    let mut cost_cache = FxHashMap::default();
    let mut heap = BinaryHeap::new();

    heap.push(State::new(0, ivec2(0, 0), ivec2(1, 0), 0));
    heap.push(State::new(0, ivec2(0, 0), ivec2(0, 1), 0));

    while let Some(State {
        heat_loss,
        pos,
        dir,
        straight_count,
    }) = heap.pop()
    {
        if pos == goal && straight_count >= 4 {
            println!("res: {}, {} us", heat_loss, t.elapsed().as_micros());
            return Ok(());
        }

        let state_key = (pos, dir, straight_count);
        if let Some(&cost) = cost_cache.get(&state_key) {
            if heat_loss >= cost {
                continue;
            }
        }

        cost_cache.insert(state_key, heat_loss);

        let next_dirs = if straight_count < 4 {
            vec![dir]
        } else if straight_count < 10 {
            vec![dir, ivec2(dir.y, -dir.x), ivec2(-dir.y, dir.x)]
        } else {
            vec![ivec2(dir.y, -dir.x), ivec2(-dir.y, dir.x)]
        };

        for next_dir in next_dirs {
            let next_pos = pos + next_dir;
            if !map.contains_pos(&next_pos) {
                continue;
            }

            let next_straight = if next_dir == dir {
                straight_count + 1
            } else {
                1
            };

            let next_heat = heat_loss + map.at(&next_pos).to_string().parse::<i32>().unwrap();
            heap.push(State::new(next_heat, next_pos, next_dir, next_straight));
        }
    }

    panic!()
}
