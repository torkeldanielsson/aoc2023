use std::{error::Error, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    // let x = 71530_i64;
    // let prod = 940200_i64;

    let x = 40929790_i64;
    let prod = 215106415051100_i64;

    let mut first_valid = 0;

    for y in 1..x {
        let test_prod = (x - y) * y;
        if test_prod > prod {
            first_valid = y;
            break;
        }
    }

    println!("first_valid: {first_valid}");

    let res = x - 2 * first_valid + 1;

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
