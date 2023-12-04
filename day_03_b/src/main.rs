use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut id_map = HashMap::new();
    let mut id_to_num_map = HashMap::new();
    let mut id = 0;

    for (line_nr, line) in input.lines().enumerate() {
        let mut tmp_num = String::new();

        for (char_nr, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                id_map.insert((line_nr as i32, char_nr as i32), id);
                tmp_num = format!("{tmp_num}{char}");
            } else if !tmp_num.is_empty() {
                let num = tmp_num.parse::<i32>().unwrap();
                tmp_num = String::new();
                id_to_num_map.insert(id, num);
                id += 1;
            }
        }

        if !tmp_num.is_empty() {
            let num = tmp_num.parse::<i32>().unwrap();
            id_to_num_map.insert(id, num);
            id += 1;
        }
    }

    let mut res = 0;

    for (line_nr, line) in input.lines().enumerate() {
        for (char_nr, char) in line.chars().enumerate() {
            if char == '*' {
                let mut neighbor_ids = HashSet::new();

                for n in &[
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ] {
                    if let Some(&neighbor_id) =
                        id_map.get(&(line_nr as i32 + n.0, char_nr as i32 + n.1))
                    {
                        neighbor_ids.insert(neighbor_id);
                    }
                }

                if neighbor_ids.len() == 2 {
                    res += neighbor_ids
                        .iter()
                        .map(|id| id_to_num_map[id] as i64)
                        .product::<i64>();
                }
            }
        }
    }

    println!("{res}");

    Ok(())
}
