use std::time::Instant;

const TARGET1: usize = 2020;
const TARGET2: usize = 30_000_000;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/15")?;
    let starting_numbers = parse_input(&input);
    println!("Part 1: {}", part_one(&starting_numbers));
    println!("Part 2: {}", part_two(&starting_numbers));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn get_nth(start: &[usize], n: usize) -> usize {
    // On each iteration, we need
    //  * the number to consider
    //  * the last turn on which each number was seen.
    let mut last_turn: Vec<usize> = vec![0; n];
    for (val, turn) in start
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .rev()
        .skip(1)
    {
        last_turn[val] = turn;
    }
    let mut to_consider = *start.last().unwrap();
    for i in start.len() + 1..=n {
        match last_turn[to_consider] {
            0 => {
                last_turn[to_consider] = i - 1;
                to_consider = 0;
            }
            turn => {
                last_turn[to_consider] = i - 1;
                to_consider = i - turn - 1;
            }
        }
    }
    to_consider
}

fn part_one(start: &[usize]) -> usize {
    get_nth(start, TARGET1)
}

fn part_two(start: &[usize]) -> usize {
    get_nth(start, TARGET2)
}

#[test]
fn test_examples() {
    let input = "0,3,6";
    let starting_numbers = parse_input(&input);
    assert_eq!(part_one(&starting_numbers), 436);

    let starting_numbers = vec![1, 3, 2];
    assert_eq!(part_one(&starting_numbers), 1);

    let starting_numbers = vec![2, 1, 3];
    assert_eq!(part_one(&starting_numbers), 10);

    let starting_numbers = vec![1, 2, 3];
    assert_eq!(part_one(&starting_numbers), 27);

    let starting_numbers = vec![2, 3, 1];
    assert_eq!(part_one(&starting_numbers), 78);

    let starting_numbers = vec![3, 2, 1];
    assert_eq!(part_one(&starting_numbers), 438);

    let starting_numbers = vec![3, 1, 2];
    assert_eq!(part_one(&starting_numbers), 1836);

    let starting_numbers = vec![0, 3, 6];
    assert_eq!(part_two(&starting_numbers), 175594);

    let starting_numbers = vec![1, 3, 2];
    assert_eq!(part_two(&starting_numbers), 2578);

    let starting_numbers = vec![2, 1, 3];
    assert_eq!(part_two(&starting_numbers), 3544142);

    let starting_numbers = vec![1, 2, 3];
    assert_eq!(part_two(&starting_numbers), 261214);

    let starting_numbers = vec![2, 3, 1];
    assert_eq!(part_two(&starting_numbers), 6895259);

    let starting_numbers = vec![3, 2, 1];
    assert_eq!(part_two(&starting_numbers), 18);

    let starting_numbers = vec![3, 1, 2];
    assert_eq!(part_two(&starting_numbers), 362);
}
