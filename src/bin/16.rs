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
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 1: {}", part_one(&all_info));
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 2: {}", part_two(&all_info));
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

fn part_two(all_info: &AllInfo) -> u64 {
    let mut i_keys: HashMap<usize, String> = HashMap::new();
    let all_keys: HashSet<String> = all_info.keys.keys().cloned().collect();
    let valid_tickets: Vec<Vec<u32>> = all_info
        .nearby_tickets
        .iter()
        .cloned()
        .filter(|t| t.iter().all(|v| all_info.all_valid_values.contains(v)))
        .collect();

    let mut cannot_be: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut to_consider: HashSet<usize> = (0..valid_tickets[0].len()).collect();
    let mut progress = true;

    for i in to_consider.iter() {
        for (key, map) in all_info.keys.iter() {
            if valid_tickets
                .iter()
                .map(|v| v[*i])
                .any(|f| !map.contains(&f))
            {
                cannot_be
                    .entry(*i)
                    .or_insert(HashSet::new())
                    .insert(key.clone());
            }
        }
    }

    while progress {
        progress = false;
        for i in to_consider.iter() {
            if let Some(s) = cannot_be.get(&i) {
                let i_key: HashSet<String> = all_keys.difference(s).cloned().collect();
                if i_key.len() == 1 {
                    let ans = i_key.iter().next().unwrap().to_string();
                    i_keys.insert(*i, ans.clone());
                    for j in to_consider.iter() {
                        cannot_be
                            .entry(*j)
                            .or_insert(HashSet::new())
                            .insert(ans.clone());
                    }
                    progress = true
                }
            }
        }
        for i in i_keys.keys() {
            to_consider.remove(i);
        }
    }

    i_keys
        .iter()
        .filter(|(_i, key)| key.starts_with("departure"))
        .map(|(i, _key)| all_info.our_ticket[*i] as u64)
        .product()
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

    let input = "class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    let all_info = parse_input(&input);
    assert_eq!(part_two(&all_info), 11);
}
