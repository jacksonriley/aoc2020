use std::collections::HashMap;
use std::time::Instant;

fn passport_from_str(input: &str) -> HashMap<String, String> {
    let mut passport = HashMap::new();
    for kv in input.split(|c| c == ' ' || c == '\n') {
        let mut kv = kv.split(':');
        let key = kv.next().unwrap().to_string();
        let value = kv.next().unwrap().to_string();
        passport.insert(key, value);
    }
    passport
}

fn check_valid1(passport: &HashMap<String, String>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys
        .iter()
        .map(|&k| passport.contains_key(k))
        .all(|x| x)
}

fn check_valid2(passport: &HashMap<String, String>) -> bool {
    passport
        .iter()
        .map(|(k, v)| match k.as_str() {
            "byr" => {
                let year = v.parse::<u32>().unwrap();
                1920 <= year && year <= 2002
            }
            "iyr" => {
                let year = v.parse::<u32>().unwrap();
                2010 <= year && year <= 2020
            }
            "eyr" => {
                let year = v.parse::<u32>().unwrap();
                2020 <= year && year <= 2030
            }
            "hgt" => {
                if v.ends_with("cm") {
                    let height: u32 = v.trim_end_matches("cm").parse().unwrap();
                    150 <= height && height <= 193
                } else if v.ends_with("in") {
                    let height: u32 = v.trim_end_matches("in").parse().unwrap();
                    59 <= height && height <= 76
                } else {
                    false
                }
            }
            "hcl" => {
                if v.starts_with("#") {
                    if v.len() == 7
                        && v[1..]
                            .chars()
                            .filter(|c| {
                                c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)
                            })
                            .count()
                            == 6
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
            "pid" => v.len() == 9 && v.chars().filter(|c| c.is_numeric()).count() == 9,
            "cid" => true,
            _ => false,
        })
        .all(|x| x)
        && check_valid1(&passport)
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/04")?;
    let passports: Vec<HashMap<_, _>> = parse_input(&input);
    println!("Part 1: {}", part_one(&passports));
    println!("Part 2: {}", part_two(&passports));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<HashMap<String, String>> {
    input.split("\n\n").map(|p| passport_from_str(&p)).collect()
}

fn part_one(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|p| check_valid1(&p)).count()
}

fn part_two(passports: &Vec<HashMap<String, String>>) -> usize {
    passports.iter().filter(|p| check_valid2(&p)).count()
}

#[test]
fn test_examples() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    let passports = parse_input(&input);
    assert_eq!(part_one(&passports), 2);
}
