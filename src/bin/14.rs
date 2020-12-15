use std::collections::HashMap;
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
    println!("Time: {}µs", now.elapsed().as_micros());
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

fn masked2(mut value: u64, masks: &(u64, Vec<u64>)) -> Vec<u64> {
    let one_mask = &masks.0;
    let xs_masks = &masks.1;
    let mut addresses = Vec::new();

    // Apply the one mask first, as this only has to be done once.
    value |= one_mask;

    // Next do the floating bit stuff - for each address in the vector, add
    // one with the X bit switched to 0.
    addresses.push(value);
    for x_mask in xs_masks.iter() {
        let mut new_addresses = Vec::new();
        for addr in addresses.iter() {
            new_addresses.push(addr & x_mask);
        }
        addresses.extend(new_addresses.into_iter());
    }
    addresses
}

fn calculate_masks(mask: &HashMap<usize, MaskVal>) -> (u64, Vec<u64>) {
    // Return
    //  * a u64 with ones where 1 and X are in the mask
    //  * a vector of u64s with all ones except for the X position, for all X's
    let mut one_mask = 0u64;
    let mut xs_masks: Vec<u64> = Vec::new();
    for (k, v) in mask.iter() {
        match v {
            MaskVal::One => one_mask |= 1 << k,
            MaskVal::X => {
                one_mask |= 1 << k;
                xs_masks.push(!(1 << k))
            }
            _ => {}
        }
    }
    (one_mask, xs_masks)
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
    let mut curr_masks: (u64, Vec<u64>) = (0, Vec::new());
    for command in commands.iter() {
        match command {
            Command::Mask(mask) => {
                curr_masks = calculate_masks(mask);
            }
            Command::Mem((addr, val)) => {
                let now = Instant::now();
                for masked_addr in masked2(*addr, &curr_masks).iter() {
                    memory.insert(*masked_addr, *val);
                }
                println!(
                    "Mask time: {}µs\n Masks {:?}\n Addr {}\n Val {}\n",
                    now.elapsed().as_micros(),
                    curr_masks,
                    addr,
                    val
                );
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
