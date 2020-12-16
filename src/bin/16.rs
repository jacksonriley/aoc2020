use std::collections::{HashMap, HashSet};
use std::time::Instant;
#[macro_use]
extern crate serde_scan;

#[derive(Debug)]
struct AllInfo {
    keys: HashMap<String, HashSet<u32>>,
    our_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
    all_valid_values: HashSet<u32>,
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/16")?;
    let all_info = parse_input(&input);
    println!("Part 1: {}", part_one(&all_info));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> AllInfo {
    let mut keys: HashMap<String, HashSet<u32>> = HashMap::new();
    let our_ticket: Vec<u32>;
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();

    let mut lines = input.lines();
    let mut line = lines.next().unwrap();

    // Parse the keys
    while !line.trim().is_empty() {
        let parse_result: Result<(&str, u32, u32, u32, u32), _> =
            scan!("{}: {}-{} or {}-{}" <- line);
        let vals = parse_result.unwrap();
        let mut set = HashSet::new();
        for v in vals.1..=vals.2 {
            set.insert(v);
        }
        for v in vals.3..=vals.4 {
            set.insert(v);
        }
        keys.insert(vals.0.to_string(), set);
        line = lines.next().unwrap()
    }

    // Parse our ticket
    line = lines.nth(1).unwrap();
    our_ticket = line.split(',').map(|n| n.parse().unwrap()).collect();

    // Skip two lines
    let _ = lines.nth(1);

    // Parse nearby tickets
    for l in lines {
        nearby_tickets.push(l.split(',').map(|n| n.parse().unwrap()).collect());
    }

    let all_valid_values: HashSet<u32> = keys
        .values()
        .fold(HashSet::new(), |acc, s| acc.union(&s).cloned().collect());

    AllInfo {
        keys,
        our_ticket,
        nearby_tickets,
        all_valid_values,
    }
}

fn part_one(all_info: &AllInfo) -> u32 {
    all_info
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|v| !all_info.all_valid_values.contains(v))
        .sum()
}

#[test]
fn test_examples() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let all_info = parse_input(&input);
    assert_eq!(part_one(&all_info), 71);
}
