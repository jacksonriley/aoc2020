use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;

type Expression = VecDeque<Token>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OpType {
    Plus,
    Multiply,
    LB,
    RB,
}
use OpType::*;

#[derive(Debug, Clone)]
enum Token {
    Op(OpType),
    Number(u64),
}
use Token::*;

fn calculate_rpn(mut expression: Expression, op_precedence: HashMap<OpType, u8>) -> VecDeque<Token> {
    // Use https://en.wikipedia.org/wiki/Shunting-yard_algorithm
    let mut output_queue: VecDeque<Token> = VecDeque::new();
    let mut op_stack: Vec<OpType> = Vec::new();

    while !expression.is_empty() {
        let tok = expression.pop_front().unwrap();
        match tok {
            Number(n) => output_queue.push_back(Number(n)),
            Op(op) => {
                match op {
                    LB => op_stack.push(LB),
                    RB => {
                        while op_stack.last().unwrap() != &LB {
                            output_queue.push_back(Op(op_stack.pop().unwrap()));
                        }
                        // Discard the LB
                        op_stack.pop();
                    }
                    _ => {
                        while !op_stack.is_empty()
                            && op_stack.last().unwrap() != &LB
                            && op_precedence.get(&op) <= op_precedence.get(op_stack.last().unwrap())
                        {
                            output_queue.push_back(Op(op_stack.pop().unwrap()));
                        }
                        op_stack.push(op);
                    }
                }
            }
        }
    }
    while !op_stack.is_empty() {
        output_queue.push_back(Op(op_stack.pop().unwrap()));
    }
    output_queue
}

fn evaluate_rpn(mut tokens: VecDeque<Token>) -> u64 {
    let mut stack : Vec<u64> = Vec::new();
    while !tokens.is_empty() {
        let tok = tokens.pop_front().unwrap();
        match tok {
            Number(n) => stack.push(n),
            Op(op) => {
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();
                match op {
                    Plus => stack.push (first + second),
                    Multiply => stack.push(first * second),
                    _ => panic!("Didn't exprect op {:?} in RPN!", op),
                }
            }
        }
    }
    stack.pop().unwrap()
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
                    '(' => Some(Token::Op(OpType::LB)),
                    ')' => Some(Token::Op(OpType::RB)),
                    '+' => Some(Token::Op(OpType::Plus)),
                    '*' => Some(Token::Op(OpType::Multiply)),
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
            let prec = vec![(Plus, 0u8), (Multiply, 0u8)];
            let op_prec: HashMap<OpType, u8> = prec.into_iter().collect();
            let rpn = calculate_rpn(e.clone(), op_prec);
            evaluate_rpn(rpn)
        })
        .sum()
}

fn part_two(expressions: &[Expression]) -> u64 {
    expressions
        .iter()
        .map(|e| {
            let prec = vec![(Plus, 1u8), (Multiply, 0u8)];
            let op_prec: HashMap<OpType, u8> = prec.into_iter().collect();
            let rpn = calculate_rpn(e.clone(), op_prec);
            evaluate_rpn(rpn)
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
