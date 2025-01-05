use std::{error::Error, ops::RangeInclusive, time::Instant};

use range_set::RangeSet;
use rustc_hash::FxHashMap;

struct Rule {
    var: char,
    comp: char,
    val: u32,
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
                        val: r_parts_2[1].parse::<u32>().unwrap(),
                        target: target.to_string(),
                    });
                } else {
                    let r_parts_2: Vec<&str> = r_parts[0].split(">").collect();
                    rules.push(Rule {
                        var: r_parts_2[0].chars().last().unwrap(),
                        comp: '>',
                        val: r_parts_2[1].parse::<u32>().unwrap(),
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

    let mut res = 0;

    recurse_rule(
        "in",
        &all_rules,
        &mut res,
        RangeSet::<[std::ops::RangeInclusive<u32>; 0]>::from(1..=4000),
        RangeSet::<[std::ops::RangeInclusive<u32>; 0]>::from(1..=4000),
        RangeSet::<[std::ops::RangeInclusive<u32>; 0]>::from(1..=4000),
        RangeSet::<[std::ops::RangeInclusive<u32>; 0]>::from(1..=4000),
    );

    println!("res: {res}, {} us", t.elapsed().as_micros());

    Ok(())
}

fn recurse_rule(
    workflow: &str,
    all_rules: &FxHashMap<&str, Vec<Rule>>,
    sum: &mut u64,
    mut x_avail: RangeSet<[RangeInclusive<u32>; 0]>,
    mut m_avail: RangeSet<[RangeInclusive<u32>; 0]>,
    mut a_avail: RangeSet<[RangeInclusive<u32>; 0]>,
    mut s_avail: RangeSet<[RangeInclusive<u32>; 0]>,
) {
    if workflow == "R" {
        return;
    }

    if workflow == "A" {
        *sum += available_numbers(&x_avail) as u64
            * available_numbers(&m_avail) as u64
            * available_numbers(&a_avail) as u64
            * available_numbers(&s_avail) as u64;
        return;
    }

    for rule in &all_rules[workflow] {
        match rule.comp {
            '=' => {
                recurse_rule(
                    &rule.target,
                    all_rules,
                    sum,
                    x_avail.clone(),
                    m_avail.clone(),
                    a_avail.clone(),
                    s_avail.clone(),
                );
                return;
            }
            '<' | '>' => {
                let mut x_avail_clone = x_avail.clone();
                let mut m_avail_clone = m_avail.clone();
                let mut a_avail_clone = a_avail.clone();
                let mut s_avail_clone = s_avail.clone();

                match rule.comp {
                    '<' => {
                        match rule.var {
                            'x' => {
                                x_avail_clone.remove_range(rule.val..=4000);
                                x_avail.remove_range(1..=rule.val - 1);
                            }
                            'm' => {
                                m_avail_clone.remove_range(rule.val..=4000);
                                m_avail.remove_range(1..=rule.val - 1);
                            }
                            'a' => {
                                a_avail_clone.remove_range(rule.val..=4000);
                                a_avail.remove_range(1..=rule.val - 1);
                            }
                            's' => {
                                s_avail_clone.remove_range(rule.val..=4000);
                                s_avail.remove_range(1..=rule.val - 1);
                            }
                            _ => panic!(),
                        };
                    }
                    '>' => {
                        match rule.var {
                            'x' => {
                                x_avail.remove_range(rule.val + 1..=4000);
                                x_avail_clone.remove_range(1..=rule.val);
                            }
                            'm' => {
                                m_avail.remove_range(rule.val + 1..=4000);
                                m_avail_clone.remove_range(1..=rule.val);
                            }
                            'a' => {
                                a_avail.remove_range(rule.val + 1..=4000);
                                a_avail_clone.remove_range(1..=rule.val);
                            }
                            's' => {
                                s_avail.remove_range(rule.val + 1..=4000);
                                s_avail_clone.remove_range(1..=rule.val);
                            }
                            _ => panic!(),
                        };
                    }
                    _ => panic!(),
                }

                recurse_rule(
                    &rule.target,
                    all_rules,
                    sum,
                    x_avail_clone,
                    m_avail_clone,
                    a_avail_clone,
                    s_avail_clone,
                );
            }
            _ => panic!(),
        }
    }
}

fn available_numbers(range_set: &RangeSet<[RangeInclusive<u32>; 0]>) -> u32 {
    range_set
        .as_ref()
        .iter()
        .map(|r| (r.end() - r.start() + 1))
        .sum()
}
