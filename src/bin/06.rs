use std::collections::HashSet;
use std::time::Instant;

struct Group {
    answers: Vec<Vec<char>>,
}

impl Group {
    fn from_str(input: &str) -> Self {
        let mut answers: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            answers.push(line.chars().filter(|c| c.is_ascii_lowercase()).collect())
        }
        Self { answers }
    }

    fn get_num_all_answers(&self) -> usize {
        let set: HashSet<&char> = self.answers.iter().flatten().collect();
        set.len()
    }

    fn get_num_everyone_answered(&self) -> usize {
        let mut everyone_answered_count = 0;
        for c in &self.answers[0] {
            if self.answers.iter().all(|v| v.contains(&c)) {
                everyone_answered_count += 1
            }
        }
        everyone_answered_count
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/06")?;
    let groups = parse_input(&input);
    println!("Part 1: {}", part_one(&groups));
    println!("Part 2: {}", part_two(&groups));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Group> {
    input.split("\n\n").map(Group::from_str).collect()
}

fn part_one(groups: &[Group]) -> usize {
    groups.iter().map(|g| g.get_num_all_answers()).sum()
}

fn part_two(groups: &[Group]) -> usize {
    groups.iter().map(|g| g.get_num_everyone_answered()).sum()
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
    let groups = parse_input(&input);
    assert_eq!(part_one(&groups), 11);
    assert_eq!(part_two(&groups), 6);
}
