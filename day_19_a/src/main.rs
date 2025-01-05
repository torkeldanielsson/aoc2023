use std::{error::Error, time::Instant};

use rustc_hash::FxHashMap;

struct Rule {
    var: char,
    comp: char,
    val: u64,
    target: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let t = Instant::now();

    let input = include_str!("../input");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut all_rules = FxHashMap::default();

    let mut all_rules_in_order = Vec::new();

    for line in parts[0].lines() {
        let parts: Vec<&str> = line.split("{").collect();
        let id = parts[0];
        let mut rules = Vec::new();
        let rs = parts[1].strip_suffix("}").unwrap();
        for r in rs.split(",") {
            if r.contains(":") {
                let r_parts: Vec<&str> = r.split(":").collect();
                let target = r_parts[1];
                if r_parts[0].contains("<") {
                    let r_parts_2: Vec<&str> = r_parts[0].split("<").collect();
                    rules.push(Rule {
                        var: r_parts_2[0].chars().last().unwrap(),
                        comp: '<',
                        val: r_parts_2[1].parse::<u64>().unwrap(),
                        target: target.to_string(),
                    });
                } else {
                    let r_parts_2: Vec<&str> = r_parts[0].split(">").collect();
                    rules.push(Rule {
                        var: r_parts_2[0].chars().last().unwrap(),
                        comp: '>',
                        val: r_parts_2[1].parse::<u64>().unwrap(),
                        target: target.to_string(),
                    });
                }
            } else {
                rules.push(Rule {
                    var: '-',
                    comp: '=',
                    val: 0,
                    target: r.to_string(),
                });
            }
        }
        all_rules_in_order.push(id);
        all_rules.insert(id, rules);
    }

    // for id in all_rules_in_order {
    //     let rules = &all_rules[id];
    //     print!("{id}{{");
    //     for rule in rules {
    //         if rule.comp == '=' {
    //             print!("{},", rule.target)
    //         } else {
    //             print!("{}{}{}:{},", rule.var, rule.comp, rule.val, rule.target)
    //         }
    //     }
    //     println!("}}")
    // }

    let mut res = 0;

    for line in parts[1].lines() {
        let lp: Vec<&str> = line
            .strip_suffix("}")
            .unwrap()
            .strip_prefix("{")
            .unwrap()
            .split(",")
            .collect();
        let x = lp[0][2..].parse::<u64>().unwrap();
        let m = lp[1][2..].parse::<u64>().unwrap();
        let a = lp[2][2..].parse::<u64>().unwrap();
        let s = lp[3][2..].parse::<u64>().unwrap();

        let mut workflow = "in";

        // println!("{line}");

        'workflow_loop: while workflow != "R" && workflow != "A" {
            // print!("{workflow} -> ");

            let rules = &all_rules[workflow];

            for rule in rules {
                match rule.comp {
                    '=' => workflow = &rule.target,
                    '<' | '>' => {
                        let var = match rule.var {
                            'x' => x,
                            'm' => m,
                            'a' => a,
                            's' => s,
                            _ => panic!(),
                        };
                        if rule.comp == '<' && var < rule.val {
                            workflow = &rule.target;
                            continue 'workflow_loop;
                        }
                        if rule.comp == '>' && var > rule.val {
                            workflow = &rule.target;
                            continue 'workflow_loop;
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        // println!("{workflow}");

        if workflow == "A" {
            res += x + m + a + s;
        }
    }

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}
