use std::collections::HashSet;
use std::time::Instant;

const PREAMBLE_LEN: usize = 25;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/09")?;
    let numbers = parse_input(&input);
    println!("Part 1: {}", part_one(&numbers));
    println!("Part 2: {}", part_two(&numbers));
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
    // Because the numbers are all positive, we can iterate through with two
    // indices, lower and upper, as follows:
    //  * If the sum of numbers between lower and upper is too small, then
    //     upper is guaranteed to be too low, so increase it.
    //  * If the sum of numbers between lower and upper is too large, then
    //     lower is guaranteed to be too low, so increase it.
    let target = part_one(numbers);
    let mut lower: usize = 0;
    let mut upper: usize = 1;
    let mut sum: u64 = numbers[lower] + numbers[upper];
    while sum!= target {
        if sum < target {
            // upper is too low
            upper += 1;
            sum += numbers[upper];
        }
        else {
            // lower is too low
            sum -= numbers[lower];
            lower += 1;
        }
    }
    // We're done - return the min + max of the interval
    let min = numbers[lower..upper].iter().min().unwrap();
    let max = numbers[lower..upper].iter().max().unwrap();
    min + max
}
