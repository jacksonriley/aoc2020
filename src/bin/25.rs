use std::collections::HashMap;
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

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    // Modular exponentiation by squaring
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus
    }
    result
}

fn part_one(card_public: u64, door_public: u64) -> u64 {
    // Uses https://en.wikipedia.org/wiki/Baby-step_giant-step to get the
    // number of loops, and then the encrypted key is simply the modular
    // exponentiation of the other public key to the number of loops.
    // Can't say I fully understand the algorithm but it makes the code go brr.
    let g = 7;
    let m = (MOD as f64).sqrt().ceil() as u64;
    let mut table: HashMap<u64, u64> = HashMap::new();
    let mut e = 1u64;
    for i in 0..m {
        table.insert(e, i);
        e = (e * g) % MOD;
    }

    let factor = mod_pow(g, MOD - m - 1, MOD);

    let mut loops = 0;

    e = card_public;
    for j in 0..m {
        if table.contains_key(&e) {
            loops = j * m + table.get(&e).unwrap();
            break;
        }
        e = (e * factor) % MOD;
    }

    mod_pow(door_public, loops, MOD)
}

#[test]
fn test_examples() {
    assert_eq!(part_one(5764801, 17807724), 14897079);
}
