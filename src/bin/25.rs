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

fn part_one(card_public: u64, door_public: u64) -> u64 {
    // Only loop the minimum required number of times by using an implicit
    // loop size - as soon as one of the public keys reach the observed card or
    // door public key, the correpsonding encryption key will be correct.
    let mut public_keys = [1, 1];
    let mut encryption_key = [1, 1];
    let mut which = 0;
    loop {
        public_keys[0] = (public_keys[0] * 7) % MOD;
        public_keys[1] = (public_keys[1] * 7) % MOD;
        encryption_key[0] = (encryption_key[0] * card_public) % MOD;
        encryption_key[1] = (encryption_key[1] * door_public) % MOD;
        if public_keys[0] == door_public {
            break
        }
        if public_keys[1] == card_public {
            which = 1;
            break
        }
    }
    encryption_key[which]
}

#[test]
fn test_examples() {
    assert_eq!(part_one(5764801, 17807724), 14897079);
}
