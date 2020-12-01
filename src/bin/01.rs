use std::collections::HashSet;
use std::time::Instant;

const TARGET: u32 = 2020;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/01")?;
    let numbers = parse_input(&input);
    println!(
        "Part 1: {}",
        part_one(&numbers, TARGET).expect("Couldn't find a solution for part 1")
    );
    println!(
        "Part 2: {}",
        part_two(&numbers).expect("Couldn't find a solution for part 2")
    );
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> HashSet<u32> {
    input
        .lines()
        .map(|s| s.parse().expect("Couldn't parse into u32"))
        .collect()
}

fn part_one(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    // O(N) - loop over the numbers and check if the number that would sum to
    // the target is in the set.
    // Return None if no such pair of numbers is found.
    for x in numbers {
        if let Some(y) = target.checked_sub(*x) {
            if numbers.contains(&y) {
                return Some(x * y);
            }
        }
    }
    None
}

fn part_two(numbers: &HashSet<u32>) -> Option<u32> {
    // O(N^2) - for each element x in the set we want to find two other numbers
    // which add to (TARGET - x), so we can simply reuse part_one.
    // Return None if no triple of numbers is found.
    for x in numbers {
        if let Some(pair_product) = part_one(&numbers, TARGET - x) {
            return Some(pair_product * x);
        }
    }
    None
}

#[test]
fn test_examples() {
    let input = "1721
979
366
299
675
1456";
    let numbers = parse_input(&input);

    // Check None is returned if there is no solution
    assert_eq!(part_one(&HashSet::new(), 0), None);
    assert_eq!(part_one(&numbers, 0), None);

    // Check the correct answers are obtained from the example input.
    assert_eq!(part_one(&numbers, TARGET), Some(1721 * 299));
    assert_eq!(part_two(&numbers), Some(979 * 366 * 675));
}
