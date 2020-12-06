use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/06")?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn part_one(input: &str) -> u32 {
    // Bit twiddling solution - map each answer ('a' through 'z') to a u32
    // bitfield. (Choose u32 because 32 > 26).
    // We can simply OR all of these u32s together, which will give us the set
    // of all the answers for a group.
    // Counting ones then counts total answers for a group.
    // Sum all the group totals together.
    input
        .split("\n\n")
        .map(|g| {
            g.bytes()
                .filter(|&x| x != b'\n')
                .fold(u32::MIN, |acc, answer| acc | (1u32 << (answer - b'a')))
                .count_ones()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    // Bit twiddling solution - map each answer ('a' through 'z') to a u32
    // bitfield.
    // For each line (person) in a group, OR these together.
    // AND all of the lines in a group together to get the common answers.
    // Counting ones then counts total answers for a group.
    // Sum all the group totals together.
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| {
                    l.bytes()
                        .fold(u32::MIN, |acc, answer| acc | (1u32 << (answer - b'a')))
                })
                .fold(u32::MAX, |acc, person| acc & person)
                .count_ones()
        })
        .sum()
}

#[test]
fn test_examples() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
    assert_eq!(part_one(&input), 11);
    assert_eq!(part_two(&input), 6);
}
