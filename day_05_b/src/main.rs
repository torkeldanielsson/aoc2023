use std::{error::Error, fs};

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    span: i64,
}

impl Range {
    fn end(&self) -> i64 {
        self.start + self.span
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let end = self.end().min(other.end());
        
        if start < end {
            Some(Range {
                start,
                span: end - start,
            })
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<i64> = parts[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let mut ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|x| Range {
            start: x[0],
            span: x[1],
        })
        .collect();

    for part in &parts[1..] {
        let mut new_ranges = Vec::new();
        let mut unprocessed = ranges.clone();

        for line in part.lines().skip(1) {
            let nums: Vec<i64> = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            
            let (dest_start, src_start, span) = (nums[0], nums[1], nums[2]);
            let src_range = Range { start: src_start, span };
            let offset = dest_start - src_start;

            let mut remaining = Vec::new();
            
            while let Some(range) = unprocessed.pop() {
                if let Some(intersection) = range.intersect(&src_range) {
                    new_ranges.push(Range {
                        start: intersection.start + offset,
                        span: intersection.span,
                    });

                    if range.start < intersection.start {
                        remaining.push(Range {
                            start: range.start,
                            span: intersection.start - range.start,
                        });
                    }
                    if range.end() > intersection.end() {
                        remaining.push(Range {
                            start: intersection.end(),
                            span: range.end() - intersection.end(),
                        });
                    }
                } else {
                    remaining.push(range);
                }
            }
            unprocessed = remaining;
        }

        new_ranges.extend(unprocessed);
        ranges = new_ranges;
    }

    let result = ranges.iter().map(|r| r.start).min().unwrap();
    println!("Lowest location: {}", result);

    Ok(())
}