use regex::Regex;
use std::time::Instant;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Eq, PartialEq)]
struct PasswordRule {
    lower: usize,
    upper: usize,
    letter: char,
    password: String,
}

impl PasswordRule {
    fn is_valid(&self) -> bool {
        let num_instances = self.password.chars().filter(|&c| c == self.letter).count();
        self.lower <= num_instances && num_instances <= self.upper
    }

    fn is_valid2(&self) -> bool {
        // Exactly one of the numbered positions must correspond to the
        // letter, so use the XOR operator, ^.
        (self.password.chars().nth(self.lower - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.upper - 1).unwrap() == self.letter)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/02")?;

    let passwords = parse_input(&input)?;
    println!("Part 1: {}", part_one(&passwords));
    println!("Part 2: {}", part_two(&passwords));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<PasswordRule>, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref PASSWORD_REGEX: Regex =
            Regex::new(r"^(?P<lower>\d+)-(?P<upper>\d+) (?P<letter>\w): (?P<password>\w+)$")
                .unwrap();
    }

    let mut passwords: Vec<PasswordRule> = vec![];
    for line in input.lines() {
        if let Some(caps) = PASSWORD_REGEX.captures(line) {
            passwords.push(PasswordRule {
                lower: caps.name("lower").unwrap().as_str().parse()?,
                upper: caps.name("upper").unwrap().as_str().parse()?,
                letter: caps.name("letter").unwrap().as_str().parse()?,
                password: caps.name("password").unwrap().as_str().to_string(),
            })
        }
    }
    Ok(passwords)
}

fn part_one(passwords: &[PasswordRule]) -> usize {
    passwords.iter().filter(|p| p.is_valid()).count()
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
            password: "abcde".to_string(),
        },
        PasswordRule {
            lower: 1,
            upper: 3,
            letter: 'b',
            password: "cdefg".to_string(),
        },
        PasswordRule {
            lower: 2,
            upper: 9,
            letter: 'c',
            password: "ccccccccc".to_string(),
        },
    ];
    assert_eq!(parse_input(&input).unwrap(), expected);
    assert_eq!(part_one(&expected), 2);
    assert_eq!(part_two(&expected), 1);
}
