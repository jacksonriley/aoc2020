use std::collections::HashSet;
use std::time::Instant;

const PREAMBLE_LEN: usize = 25;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/09")?;
    let numbers = parse_input(&input);
    println!("Part 1: {}", part_one(&numbers));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_one(numbers: &[u64]) -> u64 {
    let mut possible_summants: HashSet<&u64> = numbers.iter().take(PREAMBLE_LEN).collect();
    for (i, n) in numbers.iter().skip(PREAMBLE_LEN).enumerate() {
        match possible_summants
            .iter()
            .find(|&x| possible_summants.contains(&(n - *x)))
        {
            None => return *n,
            Some(_) => {
                // Modify the possible summants set in preparation for the next
                // iteration.
                possible_summants.remove(&numbers[i]);
                possible_summants.insert(&numbers[i + PREAMBLE_LEN]);
            }
        }
    }
    unreachable!();
}

fn part_two(numbers: &[u64]) -> u64 {
    let target = part_one(numbers);
    // Should do something based on indexes I guess
    todo!()
}
