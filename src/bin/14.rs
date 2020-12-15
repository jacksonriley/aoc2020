use std::collections::{HashMap, HashSet};
use std::time::Instant;
#[macro_use]
extern crate serde_scan;

#[derive(Debug, Clone)]
enum MaskVal {
    X,
    One,
    Zero,
}

#[derive(Debug, Clone)]
enum Command {
    Mask(HashMap<usize, MaskVal>),
    Mem((u64, u64)),
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/14")?;
    let commands = parse_input(&input);
    println!("Part 1: {}", part_one(&commands));
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 2: {}", part_two(&commands));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("mask") {
                let map = l
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .bytes()
                    .rev()
                    .enumerate()
                    .map(|(i, b)| match b {
                        b'1' => (i, MaskVal::One),
                        b'0' => (i, MaskVal::Zero),
                        b'X' => (i, MaskVal::X),
                        _ => panic!("Got unexpected byte {} from line {}", b, l),
                    })
                    .collect::<HashMap<usize, MaskVal>>();
                Command::Mask(map)
            } else {
                let parse_result: Result<(u64, u64), _> = scan!("mem[{}] = {}" <- l);
                Command::Mem(parse_result.unwrap())
            }
        })
        .collect()
}

fn masked1(mut value: u64, mask: &HashMap<usize, MaskVal>) -> u64 {
    for (k, v) in mask.iter() {
        match v {
            MaskVal::One => {
                // Set that bit to 1
                value |= 1 << k
            }
            MaskVal::Zero => {
                // Set that bit to 0
                value &= !(1 << k)
            }
            MaskVal::X => {}
        }
    }
    value
}

fn masked2(mut value: u64, mask: &HashMap<usize, MaskVal>) -> Vec<u64> {
    // First, do all of the MaskVal::One bit sets, as these can all be done on one value.
    // Also set all of the MashVal::X bits to 1.
    let mut addresses = Vec::new();
    for (k, v) in mask.iter() {
        match v {
            MaskVal::One | MaskVal::X => {
                // Set that bit to 1
                value |= 1 << k
            }
            _ => {}
        }
    }
    // Next do the floating bit stuff - for each address in the vector, add
    // one with the X bit switched to 0.
    addresses.push(value);
    for (k, v) in mask.iter() {
        let mut new_addresses = Vec::new();
        if let MaskVal::X = v {
            for addr in addresses.iter() {
                new_addresses.push(addr & !(1 << k));
            }
        }
        addresses.extend(new_addresses.into_iter());
    }
    addresses
}

fn part_one(commands: &[Command]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut curr_mask = HashMap::new();
    for command in commands.iter() {
        match command {
            Command::Mask(mask) => curr_mask = mask.clone(),
            Command::Mem((addr, val)) => {
                memory.insert(*addr, masked1(*val, &curr_mask));
            }
        }
    }
    memory.values().sum()
}

fn part_two(commands: &[Command]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut curr_mask = HashMap::new();
    for command in commands.iter() {
        match command {
            Command::Mask(mask) => curr_mask = mask.clone(),
            Command::Mem((addr, val)) => {
                for masked_addr in masked2(*addr, &curr_mask).iter() {
                    memory.insert(*masked_addr, *val);
                }
            }
        }
    }
    memory.values().sum()
}

#[test]
fn test_example_one() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let commands = parse_input(&input);
    assert_eq!(part_one(&commands), 165);
}

#[test]
fn test_example_two() {
    let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    let commands = parse_input(&input);
    assert_eq!(part_two(&commands), 208);
}
