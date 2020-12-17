use std::collections::{HashMap, HashSet};
use std::time::Instant;
#[macro_use]
extern crate serde_scan;

type Ticket = Vec<u32>;
type FieldName = String;
#[derive(Debug)]
struct AllInfo {
    fields: HashMap<FieldName, HashSet<u32>>,
    our_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
    all_valid_values: HashSet<u32>,
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/16")?;
    let all_info = parse_input(&input);
    println!("Part 1: {}", part_one(&all_info));
    println!("Part 2: {}", part_two(&all_info));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> AllInfo {
    let mut fields: HashMap<FieldName, HashSet<u32>> = HashMap::new();
    let our_ticket: Ticket;
    let mut nearby_tickets: Vec<Ticket> = Vec::new();

    let mut lines = input.lines();
    let mut line = lines.next().unwrap();

    // Parse the fields
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
        fields.insert(vals.0.to_string(), set);
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

    let all_valid_values: HashSet<u32> = fields
        .values()
        .fold(HashSet::new(), |acc, s| acc.union(&s).cloned().collect());
    AllInfo {
        fields,
        our_ticket,
        nearby_tickets,
        all_valid_values,
    }
}

fn part_one(all_info: &AllInfo) -> u32 {
    // Iterate through all of the ticket values and filter out any that are not
    // in the set of valid values. Their sum is the ticket scanning error rate.
    all_info
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|v| !all_info.all_valid_values.contains(v))
        .sum()
}

fn part_two(all_info: &AllInfo) -> u64 {
    let mut col_to_field: HashMap<usize, FieldName> = HashMap::new();
    let valid_tickets: Vec<Vec<u32>> = all_info
        .nearby_tickets
        .iter()
        .cloned()
        .filter(|t| t.iter().all(|v| all_info.all_valid_values.contains(v)))
        .collect();
    let mut to_consider: HashSet<usize> = (0..valid_tickets[0].len()).collect();
    let mut possible_mappings: HashMap<usize, HashSet<FieldName>> = to_consider
        .iter()
        .map(|i| (*i, all_info.fields.keys().cloned().collect()))
        .collect();

    // For each column of ticket values, if any are not in the set of
    // possible values for a given field, this column cannot map to the
    // given field, so rule that field out for that column.
    for i in to_consider.iter() {
        for (key, map) in all_info.fields.iter() {
            if valid_tickets
                .iter()
                .map(|v| v[*i])
                .any(|f| !map.contains(&f))
            {
                possible_mappings.get_mut(i).unwrap().remove(key);
            }
        }
    }

    // While there are columns that we haven't solved, check if any have only
    // one possible field. If they have, then
    //  * rule out this field for all other columns
    //  * add the new correct mapping to col_to_field
    //  * remove the column from consideration
    while !to_consider.is_empty() {
        for i in to_consider.iter() {
            let s = possible_mappings.get(i).unwrap();
            if s.len() == 1 {
                let ans = s.iter().next().unwrap().to_string();
                for j in to_consider.iter() {
                    possible_mappings.get_mut(j).unwrap().remove(&ans);
                }
                col_to_field.insert(*i, ans);
            }
        }
        for i in col_to_field.keys() {
            to_consider.remove(i);
        }
    }

    col_to_field
        .iter()
        .filter(|(_col, field)| field.starts_with("departure"))
        .map(|(col, _field)| all_info.our_ticket[*col] as u64)
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
