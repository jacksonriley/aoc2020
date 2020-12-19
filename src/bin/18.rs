use std::collections::VecDeque;
use std::time::Instant;

type Expression = VecDeque<Token>;

#[derive(Debug, Clone)]
enum Token {
    LB,
    RB,
    Plus,
    Multiply,
    Number(u64),
}
use Token::*;

fn calculate(expression: &mut Expression) -> u64 {
    // Basic idea is that we recursively calculate the expression from left to
    // right, calling the function again for every left bracket we meet, and
    // returning at every right bracket or when the expression is finished.
    let mut acc = 0;
    while !expression.is_empty() {
        let tok = expression.pop_front().unwrap();
        match tok {
            RB => break,
            Number(n) => {
                let op = expression.pop_front().unwrap();
                let b = expression.pop_front().unwrap();
                match b {
                    Number(m) => match op {
                        Plus => acc += n + m,
                        Multiply => acc += n * m,
                        _ => panic!("Didn't expect 'op' {:?}", op),
                    },
                    LB => {
                        let right = calculate(expression);
                        match op {
                            Plus => acc += n + right,
                            Multiply => acc += n * right,
                            _ => panic!("Didn't expect 'op' {:?}", op),
                        }
                    }
                    _ => panic!("Didn't expect b {:?}", b),
                }
            }
            LB => acc = calculate(expression),
            Plus => {
                let right = expression.pop_front().unwrap();
                match right {
                    Number(m) => acc += m,
                    LB => acc += calculate(expression),
                    _ => panic!("Didn't expect right {:?}", right),
                }
            }
            Multiply => {
                let right = expression.pop_front().unwrap();
                match right {
                    Number(m) => acc *= m,
                    LB => acc *= calculate(expression),
                    _ => panic!("Didn't expect right {:?}", right),
                }
            }
        }
    }
    acc
}

fn calculate2(expression: &mut Expression, from_mul: bool) -> u64 {
    // Differences from the first part:
    //  * when we meet a multiply, we call the function so as to effectively
    //    have a lower precedence. This is so non-extensible it's not even
    //    funny.
    //  * pass in to each function call whether or not we've come from a
    //    multiply or a left bracket - we don't want to pop right brackets if
    //    we've come from a multiply.
    let mut acc = 0;
    while !expression.is_empty() {
        let tok = expression.pop_front().unwrap();
        match tok {
            RB => {
                if from_mul {
                    // Don't want to actually pop the RB in this case
                    expression.push_front(RB)
                }
                break;
            }
            Number(n) => {
                let op = expression.pop_front();
                match op {
                    Some(Plus) => {
                        let b = expression.pop_front().unwrap();
                        match b {
                            Number(m) => acc += n + m,
                            LB => acc += n + calculate2(expression, false),
                            _ => panic!("Didn't expect b {:?}", b),
                        }
                    }
                    Some(Multiply) => {
                        acc += n * calculate2(expression, true);
                    }
                    Some(RB) => {
                        if from_mul {
                            expression.push_front(RB);
                            return n;
                        }
                    }
                    None => return n,
                    _ => panic!("Didn't expect 'op' {:?}", op),
                }
            }
            LB => acc = calculate2(expression, false),
            Plus => {
                let right = expression.pop_front().unwrap();
                match right {
                    Number(m) => acc += m,
                    LB => acc += calculate2(expression, false),
                    _ => panic!("Didn't expect right {:?}", right),
                }
            }
            Multiply => acc *= calculate2(expression, true),
        }
    }
    acc
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/18")?;
    let expressions = parse_input(&input);
    println!("Part 1: {}", part_one(&expressions));
    println!("Part 2: {}", part_two(&expressions));
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
                        Some(n) => Some(Token::Number(n as u64)),
                        None => None,
                    },
                })
                .collect(),
        )
    }
    expressions
}

fn part_one(expressions: &[Expression]) -> u64 {
    expressions
        .iter()
        .map(|e| {
            let mut c = e.clone();
            calculate(&mut c)
        })
        .sum()
}

fn part_two(expressions: &[Expression]) -> u64 {
    expressions
        .iter()
        .map(|e| {
            let mut c = e.clone();
            calculate2(&mut c, false)
        })
        .sum()
}

#[test]
fn test_examples() {
    let input = "2 * 3 + (4 * 5)";
    let expressions = parse_input(&input);
    assert_eq!(part_one(&expressions), 26);
    assert_eq!(part_two(&expressions), 46);

    let input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    let expressions = parse_input(&input);
    assert_eq!(part_one(&expressions), 437);
    assert_eq!(part_two(&expressions), 1445);

    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    let expressions = parse_input(&input);
    assert_eq!(part_one(&expressions), 12240);
    assert_eq!(part_two(&expressions), 669060);

    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    let expressions = parse_input(&input);
    assert_eq!(part_one(&expressions), 13632);
    assert_eq!(part_two(&expressions), 23340);
}
