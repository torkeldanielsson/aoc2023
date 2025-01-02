use std::{error::Error, time::Instant};

fn find_symmetry(d: &[u32]) -> u32 {
    'outer: for s in 1..d.len() {
        let p1 = d[..s].iter().rev();
        let p2 = d[s..].iter();
        let mut ones_count = 0;
        for (p1_v, p2_v) in p1.zip(p2) {
            ones_count += (p1_v ^ p2_v).count_ones();
            if ones_count > 1 {
                continue 'outer;
            }
        }
        if ones_count == 1 {
            return s as u32;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let mut res = 0;

    for part in input.split("\n\n") {
        let mut rows = Vec::new();

        for line in part.lines() {
            let mut v = 0_u32;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    v |= 1 << i;
                }
            }
            rows.push(v);
        }

        let line_len = part.find("\n").unwrap();

        let mut cols = Vec::new();

        for i in 0..line_len {
            let mut v = 0_u32;
            for (j, row_v) in rows.iter().enumerate() {
                if (row_v & 1 << i) != 0 {
                    v |= 1 << j;
                }
            }
            cols.push(v);
        }

        let row_symmetry = find_symmetry(&rows);
        let col_symmetry = find_symmetry(&cols);
        assert!(row_symmetry == 0 || col_symmetry == 0);
        if row_symmetry == 0 {
            res += find_symmetry(&cols);
        } else {
            res += 100 * row_symmetry;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
