use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Leaf {
    Literal(String),
    Link(u32),
}
use Leaf::*;

#[derive(Debug, Clone)]
enum Rule {
    Terminal(String),
    NonTerminal(Vec<Vec<Leaf>>),
}
use Rule::*;

type Candidate = String;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/19")?;
    let (rules, candidates) = parse_input(&input);
    println!("Part 1: {}", part_one(&rules, &candidates));
    println!("Part 2: {}", part_two(&rules, &candidates));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> (HashMap<u32, Rule>, Vec<Candidate>) {
    let mut rules: HashMap<u32, Rule> = HashMap::new();
    let mut candidates: Vec<Candidate> = Vec::new();

    let mut sections = input.split("\n\n");
    let rules_str = sections.next().unwrap();
    let candidates_str = sections.next().unwrap();

    for line in rules_str.lines() {
        let mut line = line.split(": ");
        let ident: u32 = line.next().unwrap().parse().unwrap();
        let these_rules = line.next().unwrap();
        if these_rules.starts_with('"') {
            rules.insert(
                ident,
                Terminal(these_rules.trim().chars().nth(1).unwrap().to_string()),
            );
        } else {
            let all_rules = these_rules.split(" | ");
            let mut non_terminal_rules = Vec::new();
            for branch in all_rules {
                let branch = branch.split(' ');
                let mut branch_rules = Vec::new();
                for branch_rule in branch {
                    branch_rules.push(Link(branch_rule.parse().unwrap()));
                }
                non_terminal_rules.push(branch_rules)
            }
            rules.insert(ident, NonTerminal(non_terminal_rules));
        }
    }

    for line in candidates_str.lines() {
        candidates.push(line.to_string());
    }

    (rules, candidates)
}

fn create_regex_str(
    rule_id: u32,
    rules: &HashMap<u32, Rule>,
    cache: &mut HashMap<u32, String>,
) -> String {
    // If we've already created this rule's regex, just return it from the cache.
    if let Some(cached_regex) = cache.get(&rule_id) {
        return cached_regex.to_owned();
    }

    // Otherwise, construct it, either by returning the terminal letter if
    // possible, or by combining sub-rules.
    match rules.get(&rule_id).unwrap() {
        Terminal(letter) => letter.to_owned(),
        NonTerminal(sub_rules) => {
            let mut my_regex = String::from("(?:");
            for branch in sub_rules.iter() {
                // Combine each of these between '|'
                for sub_rule in branch.iter() {
                    // Combine these directly
                    match sub_rule {
                        Literal(token) => my_regex.push_str(token),
                        Link(ident) => my_regex.push_str(&create_regex_str(*ident, rules, cache)),
                    }
                }
                my_regex.push('|');
            }
            // Remove the trailing '|'
            my_regex.pop();
            my_regex.push(')');
            cache.insert(rule_id, my_regex.clone());
            my_regex
        }
    }
}

fn count_candidates(rules: &HashMap<u32, Rule>, candidates: &[String]) -> usize {
    let re_string = create_regex_str(0, rules, &mut HashMap::new());
    let re_string_full = format!("^{}$", re_string);
    let re = Regex::new(&re_string_full).unwrap();
    candidates.iter().filter(|c| re.is_match(c)).count()
}

fn part_one(rules: &HashMap<u32, Rule>, candidates: &[String]) -> usize {
    count_candidates(rules, candidates)
}

fn part_two(rules: &HashMap<u32, Rule>, candidates: &[String]) -> usize {
    let mut rules = rules.to_owned();

    // For rule 8, we can insert a '+' after evaluating rule 42, and this will
    // do the right thing.
    *rules.get_mut(&8).unwrap() = NonTerminal(vec![vec![Link(42), Literal("+".to_string())]]);
    let mut manual_rpt_vec = Vec::new();

    // For rule 11, we need to allow 42 to repeat one or more times, and 31 to
    // repeat the same number of times. The regex crate doesn't support
    // recursion so do this manually up to the point where the answer stops
    // changing - turns out this is 4 repeats. Gross I know.
    for num_repeats in 1..=4 {
        manual_rpt_vec.extend(vec![vec![
            Literal("(?:".to_string()),
            Link(42),
            Literal(format!("{{{}}}", num_repeats)),
            Link(31),
            Literal(format!("{{{}}})", num_repeats)),
        ]]);
    }
    *rules.get_mut(&11).unwrap() = NonTerminal(manual_rpt_vec);

    count_candidates(&rules, candidates)
}

#[test]
fn test_examples() {
    let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    let (rules, candidates) = parse_input(input);
    assert_eq!(part_one(&rules, &candidates), 2);

    let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    let (rules, candidates) = parse_input(input);
    assert_eq!(part_one(&rules, &candidates), 3);
    assert_eq!(part_two(&rules, &candidates), 12);
}
