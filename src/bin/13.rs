use std::time::Instant;

#[derive(Debug)]
struct Bus {
    number: u64,
    offset: u64,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Uses Euclid's algorithm
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    // Lowest common multiple
    a * b / gcd(a, b)
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/13")?;
    let (earliest, buses) = parse_input(&input);
    println!("Part 1: {}", part_one(earliest, &buses));
    println!("Part 2: {}", part_two(&buses));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> (u64, Vec<Bus>) {
    let mut lines = input.lines();
    let earliest: u64 = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|s| s.1 != "x")
        .map(|n| Bus {
            number: n.1.parse::<u64>().unwrap(),
            offset: n.0 as u64,
        })
        .collect();
    (earliest, buses)
}

fn part_one(earliest: u64, buses: &[Bus]) -> u64 {
    let leave_bus = buses
        .iter()
        .map(|b| {
            let div = earliest / b.number + 1;
            (b.number, div * b.number)
        })
        .min_by_key(|e| e.1)
        .unwrap();
    leave_bus.0 * (leave_bus.1 - earliest)
}

fn part_two(buses: &[Bus]) -> u64 {
    // Want to find the first time t such that for each bus b, (t + b.offset) % b.number == 0.
    // Do this pair-wise - once we find a suitable t for two buses, we can
    // search for a suitable t for three buses in increments of LCM(bus1, bus2).
    // This guarantees that when we satisfy the third constraint, the first two
    // will still be satisfied. Repeat.
    let mut t = 1;
    let mut increment = 1;
    for b in buses.iter() {
        while (t + b.offset) % (b.number) != 0 {
            t += increment;
        }
        increment = lcm(increment, b.number);
    }
    t
}

#[test]
fn test_example() {
    let input = "939
7,13,x,x,59,x,31,19";
    let (earliest, buses) = parse_input(&input);
    assert_eq!(part_one(earliest, &buses), 295);
    assert_eq!(part_two(&buses), 1068781);
}

#[test]
fn test_extra_part_two_examples() {
    let test1 = "0
67,7,59,61";
    let (_, buses) = parse_input(&test1);
    assert_eq!(part_two(&buses), 754018);

    let test2 = "0
67,x,7,59,61";
    let (_, buses) = parse_input(&test2);
    assert_eq!(part_two(&buses), 779210);

    let test3 = "0
67,7,x,59,61";
    let (_, buses) = parse_input(&test3);
    assert_eq!(part_two(&buses), 1261476);

    let test4 = "0
1789,37,47,1889";
    let (_, buses) = parse_input(&test4);
    assert_eq!(part_two(&buses), 1202161486);
}
