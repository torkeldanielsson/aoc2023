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

    let s = 65 + 131 + 131;
    let (
        fill_count_odd,
        fill_count_even,
        fill_count_lower_left,
        fill_count_lower_right,
        fill_count_upper_left,
        fill_count_upper_right,
        fill_count_top,
        fill_count_bottom,
        fill_count_leftmost,
        fill_count_rightmost,
        fill_count_sans_lower_left,
        fill_count_sans_lower_right,
        fill_count_sans_upper_left,
        fill_count_sans_upper_right,
    ) = fill_val_wrapping(&map, map.start_pos(), s);

    let total_steps = 26501365_u64;
    let mut steps = (total_steps - 65) / 131;

    let mut res = fill_count_top as u64
        + fill_count_bottom as u64
        + fill_count_leftmost as u64
        + fill_count_rightmost as u64
        + steps * fill_count_lower_left as u64
        + steps * fill_count_lower_right as u64
        + steps * fill_count_upper_left as u64
        + steps * fill_count_upper_right as u64
        + (steps - 1) * fill_count_sans_lower_left as u64
        + (steps - 1) * fill_count_sans_lower_right as u64
        + (steps - 1) * fill_count_sans_upper_left as u64
        + (steps - 1) * fill_count_sans_upper_right as u64;

    while steps != 0 {
        if steps % 2 == 0 {
            // println!("even, steps {steps}, adding: {}", (steps - 1) * 4);
            res += (steps - 1) * 4 * fill_count_even as u64;
        } else {
            // println!("odd, steps {steps}, adding: {}", (steps - 1) * 4);
            res += (steps - 1) * 4 * fill_count_odd as u64;
        }

        steps -= 1;
    }

    res += fill_count_odd as u64;

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}

#[allow(clippy::type_complexity)]
fn fill_val_wrapping(
    map: &Map,
    start_pos: IVec2,
    turns: u64,
) -> (
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
) {
    let mut p = FxHashSet::default();

    p.insert(start_pos);

    for _ in 0..turns {
        let mut np = FxHashSet::default();

        for p in p {
            for n in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                let n = p + n;
                let wrapped = ivec2(
                    ((n.x % map.w) + map.w) % map.w,
                    ((n.y % map.h) + map.h) % map.h,
                );

                if map.at(&wrapped) != '#' {
                    np.insert(n);
                }
            }
        }
        p = np;
    }

    let fill_count_odd = {
        let mut fill_count = 0;

        for y in 0..131 {
            for x in 0..131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_odd = {fill_count}");
        fill_count
    };

    let fill_count_even = {
        let mut fill_count = 0;

        for y in 131..2 * 131 {
            for x in 0..131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_even = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_lower_right = {
        let mut fill_count = 0;

        for y in -(2 * 131)..-(131) {
            for x in -131..0 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_lower_right = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_lower_left = {
        let mut fill_count = 0;

        for y in -(2 * 131)..-(131) {
            for x in 131..2 * 131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_lower_left = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_upper_left = {
        let mut fill_count = 0;

        for y in 2 * 131..3 * 131 {
            for x in 131..2 * 131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_upper_left = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_upper_right = {
        let mut fill_count = 0;

        for y in 2 * 131..3 * 131 {
            for x in -131..0 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_upper_right = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_top = {
        let mut fill_count = 0;

        for y in -2 * 131..-131 {
            for x in 0..131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_top = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_bottom = {
        let mut fill_count = 0;

        for y in 2 * 131..3 * 131 {
            for x in 0..131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_bottom = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_leftmost = {
        let mut fill_count = 0;

        for y in 0..131 {
            for x in -2 * 131..-131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_leftmost = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_rightmost = {
        let mut fill_count = 0;

        for y in 0..131 {
            for x in 2 * 131..3 * 131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_rightmost = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_sans_lower_right = {
        let mut fill_count = 0;

        for y in 131..2 * 131 {
            for x in 131..2 * 131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_sans_lower_right = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_sans_lower_left = {
        let mut fill_count = 0;

        for y in 131..2 * 131 {
            for x in -131..0 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_sans_lower_left = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_sans_upper_left = {
        let mut fill_count = 0;

        for y in -131..0 {
            for x in -131..0 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_sans_upper_left = {fill_count}");
        println!();

        fill_count
    };

    let fill_count_sans_upper_right = {
        let mut fill_count = 0;

        for y in -131..0 {
            for x in 131..2 * 131 {
                let pos = ivec2(x, y);
                let wrapped = ivec2(((x % map.w) + map.w) % map.w, ((y % map.h) + map.h) % map.h);

                if map.at(&wrapped) == '#' {
                    print!("#")
                } else if p.contains(&pos) {
                    print!("O");
                    fill_count += 1;
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
        println!("fill_count_sans_upper_right = {fill_count}");
        println!();

        fill_count
    };

    println!("p.len(): {}", p.len());

    (
        fill_count_odd,
        fill_count_even,
        fill_count_lower_left,
        fill_count_lower_right,
        fill_count_upper_left,
        fill_count_upper_right,
        fill_count_top,
        fill_count_bottom,
        fill_count_leftmost,
        fill_count_rightmost,
        fill_count_sans_lower_left,
        fill_count_sans_lower_right,
        fill_count_sans_upper_left,
        fill_count_sans_upper_right,
    )
}
