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
        .scan(0, |state, s| {
            *state += 1;
            Some((s, *state - 1))
        })
        .filter(|s| s.0 != "x")
        .map(|n| Bus {
            number: n.0.parse::<u64>().unwrap(),
            offset: n.1,
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
    // Want to find the first time t such that for each bus b, t + b.offset is
    // a multiple of b.number
    // For example, find 7x = T, 13y = (T+1)
    // Check multiples of 7 and check if T+1 is a multiple of 13
    // Find t'
    // Next, need 59z = (T+4)
    // Have 7x = t', 13y = t' + 1, but still need 7x = T, 13y = (T+1)
    // So T is t' + W
    // 7x = t' + W, 13y = t' + 1 + W
    // Once we have t', how can we find more?
    // W just has to be a multiple of 7 and 13! Then our first two conditions still hold!
    // We can therefore try adding multiples of LCM(7, 13) to find 59x = t' + mW + 4
    // Once found, t' + mW becomes our new t', and we then search for the next one using LCM(7, 13, 59).
    let mut t = 1;
    let mut jump = 1;
    for b in buses.iter() {
        while (t + b.offset) % (b.number) != 0 {
            t += jump;
        }
        jump = lcm(jump, b.number);
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
