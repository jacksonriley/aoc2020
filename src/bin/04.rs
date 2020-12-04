use std::collections::HashMap;
use std::time::Instant;

fn passport_from_str(input: &str) -> HashMap<&str, &str> {
    let mut passport = HashMap::new();
    for kv in input.split(|c| c == ' ' || c == '\n') {
        let mut kv = kv.split(':');
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();
        passport.insert(key, value);
    }
    passport
}

fn check_valid1(passport: &HashMap<&str, &str>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys
        .iter()
        .map(|&k| passport.contains_key(k))
        .all(|x| x)
}

fn check_valid2(passport: &HashMap<&str, &str>) -> bool {
    passport
        .iter()
        .map(|(k, v)| match *k {
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
                if v.starts_with('#') {
                    v.len() == 7
                        && v[1..]
                            .chars()
                            .filter(|c| {
                                c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)
                            })
                            .count()
                            == 6
                } else {
                    false
                }
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
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

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input.split("\n\n").map(|p| passport_from_str(&p)).collect()
}

fn part_one(passports: &[HashMap<&str, &str>]) -> usize {
    passports.iter().filter(|p| check_valid1(&p)).count()
}

fn part_two(passports: &[HashMap<&str, &str>]) -> usize {
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

    let part2_invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    let passports = parse_input(&part2_invalid);
    assert_eq!(part_two(&passports), 0);

    let part2_valid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let passports = parse_input(&part2_valid);
    assert_eq!(part_two(&passports), 4);
}
