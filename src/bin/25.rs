use std::time::Instant;

const MOD: u64 = 20201227;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/25")?;
    let (card, door) = parse_input(&input);
    println!("Part 1: {}", part_one(card, door));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let card_public = lines.next().unwrap().trim().parse().unwrap();
    let door_public = lines.next().unwrap().trim().parse().unwrap();
    (card_public, door_public)
}

fn part_one(card_public: u64, card_door: u64) -> u64 {
    let mut card_loop_size = 0;
    let mut current = 1;
    loop {
        card_loop_size += 1;
        current *= 7;
        current %= MOD;
        if current == card_public {
            break;
        }
    }

    let mut encryption_key = 1;
    for _ in 0..card_loop_size {
        encryption_key *= card_door;
        encryption_key %= MOD;
    }
    encryption_key
}

#[test]
fn test_examples() {
    assert_eq!(part_one(5764801, 17807724), 14897079);
}
