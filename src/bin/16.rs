use std::collections::HashMap;
use std::time::Instant;
#[macro_use]
extern crate serde_scan;

#[derive(Debug)]
struct AllInfo {
    keys: HashMap<String, ((u32, u32), (u32, u32))>,
    our_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/16")?;
    let all_info = parse_input(&input);
    println!("All info: {:?}", all_info);
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn parse_input(input: &str) -> AllInfo {
    let mut keys: HashMap<String, ((u32, u32), (u32, u32))> = HashMap::new();
    let our_ticket: Vec<u32>;
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();

    let mut lines = input.lines();
    let mut line = lines.next().unwrap();

    // Parse the keys
    while !line.trim().is_empty() {
        println!("{}", line);
        let parse_result: Result<(&str, u32, u32, u32, u32), _> =
            scan!("{}: {}-{} or {}-{}" <- line);
        let vals = parse_result.unwrap();
        keys.insert(vals.0.to_string(), ((vals.1, vals.2), (vals.3, vals.4)));
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

    AllInfo {
        keys,
        our_ticket,
        nearby_tickets,
    }
}
