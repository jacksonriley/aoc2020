use std::collections::VecDeque;
use std::time::Instant;

type Expression = VecDeque<Token>;

#[derive(Debug)]
enum Token {
    LB,
    RB,
    Plus,
    Multiply,
    Number(u32),
}

fn calculate(expression: &mut Expression) -> u32 {
    
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/18")?;
    let expressions = parse_input(&input);
    println!("Part 1: {}", part_one(&expressions));
    // println!("Part 2: {}", part_two(&all_info));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Expression> {
    let mut expressions = Vec::new();
    for line in input.lines() {
        expressions.push(
            line.chars()
                .filter_map(|c| match c {
                    '(' => Some(Token::LB),
                    ')' => Some(Token::RB),
                    '+' => Some(Token::Plus),
                    '*' => Some(Token::Multiply),
                    maybe_num => match maybe_num.to_digit(10) {
                        Some(n) => Some(Token::Number(n)),
                        None => None,
                    },
                })
                .collect(),
        )
    }
    expressions
}

fn part_one(expressions: &[Expression]) -> u32 {
    println!("{:?}", &expressions[..3]);
    expressions.iter().map(|e| calculate(e.clone())).sum()
}

#[test]
fn test_examples() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
    let all_info = parse_input(&input);
    assert_eq!(part_one(&all_info), 71);

    let input = "class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
    let all_info = parse_input(&input);
    assert_eq!(part_two(&all_info), 11);
}
