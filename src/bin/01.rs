use std::collections::HashSet;

const TARGET: u32 = 2020;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input/01")?;
    let numbers: HashSet<u32> = input
        .lines()
        .map(|s| s.parse().expect("Couldn't parse into u32"))
        .collect();
    println!(
        "Part 1: {}",
        part_one(&numbers, TARGET).expect("Couldn't find a solution for part 1")
    );
    println!(
        "Part 2: {}",
        part_two(&numbers).expect("Couldn't find a solution for part 2")
    );
    Ok(())
}

fn part_one(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    // O(N) - loop over the numbers and check if the number that would sum to
    // TARGET is in the set.
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
    // which add to TARGET - x, so we can simply reuse part_one.
    // Return None if no triple of numbers is found.
    for x in numbers {
        if let Some(product) = part_one(&numbers, TARGET - x) {
            return Some(product * x);
        }
    }
    None
}

#[test]
fn test_part_one() {
    let input: HashSet<u32> = [1, 2, 3, 4].iter().cloned().collect();
    assert_eq!(part_one(&input, 10), None);
    assert_eq!(part_one(&input, 7), Some(3 * 4));
}

#[test]
fn test_part_two() {
    let input = HashSet::new();
    assert_eq!(part_two(&input), None);
    let input: HashSet<u32> = [1, 2, 3, 4, 2017].iter().cloned().collect();
    assert_eq!(part_two(&input), Some(2 * 2017));
}
