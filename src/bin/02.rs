use std::time::Instant;
#[macro_use]
extern crate serde_scan;

#[derive(Debug, Eq, PartialEq)]
struct PasswordRule<'a> {
    lower: usize,
    upper: usize,
    letter: char,
    password: &'a str,
}

impl PasswordRule<'_> {
    fn is_valid1(&self) -> bool {
        let num_instances = self.password.chars().filter(|&c| c == self.letter).count();
        (self.lower..=self.upper).contains(&num_instances)
    }

    fn is_valid2(&self) -> bool {
        // Exactly one of the numbered positions must correspond to the
        // letter, so use the XOR operator, ^.
        (self.password.chars().nth(self.lower - 1) == Some(self.letter))
            ^ (self.password.chars().nth(self.upper - 1) == Some(self.letter))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/02")?;

    let passwords = parse_input(&input);
    println!("Part 1: {}", part_one(&passwords));
    println!("Part 2: {}", part_two(&passwords));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<PasswordRule> {
    let mut passwords: Vec<PasswordRule> = Vec::new();
    for line in input.lines() {
        let p_res: Result<(usize, usize, char, &str), serde_scan::ScanError> =
            scan!("{}-{} {}: {}" <- line);
        if let Ok(parsed) = p_res {
            passwords.push(PasswordRule {
                lower: parsed.0,
                upper: parsed.1,
                letter: parsed.2,
                password: parsed.3,
            })
        }
    }
    passwords
}

fn part_one(passwords: &[PasswordRule]) -> usize {
    passwords.iter().filter(|p| p.is_valid1()).count()
}

fn part_two(passwords: &[PasswordRule]) -> usize {
    passwords.iter().filter(|p| p.is_valid2()).count()
}

#[test]
fn test_examples() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    let expected = vec![
        PasswordRule {
            lower: 1,
            upper: 3,
            letter: 'a',
            password: "abcde",
        },
        PasswordRule {
            lower: 1,
            upper: 3,
            letter: 'b',
            password: "cdefg",
        },
        PasswordRule {
            lower: 2,
            upper: 9,
            letter: 'c',
            password: "ccccccccc",
        },
    ];
    assert_eq!(parse_input(&input), expected);
    assert_eq!(part_one(&expected), 2);
    assert_eq!(part_two(&expected), 1);
}
