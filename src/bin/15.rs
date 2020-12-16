use std::collections::{hash_map::Entry, HashMap};
use std::time::Instant;

const TARGET1: u32 = 2020;
const TARGET2: u32 = 30_000_000;
// The value of BOUNDARY is a bit of trial and error to get a trade-off between
// a not-insane stack size and a good runtime.
const BOUNDARY: u32 = TARGET2 / 30;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/15")?;
    let starting_numbers = parse_input(&input);
    println!("Part 1: {}", part_one(&starting_numbers));
    println!("Part 2: {}", part_two(&starting_numbers));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn get_nth(start: &[u32], n: u32) -> u32 {
    // On each iteration, we need
    //  * the number to consider
    //  * the last turn on which each number was seen.

    // Uses https://github.com/timvisee/advent-of-code-2020/blob/master/day15b/src/main.rs
    // Without blowing out the stack, use an array for as many of the lower
    // numbers (overall more common?) as possible, and a HashMap for the higher
    // numbers. This means that more of the accesses are cached, resuling in
    // higher performance.
    let mut high: HashMap<u32, u32> = HashMap::with_capacity(1024 * 256);
    let mut low = [0u32; BOUNDARY as usize];

    for (val, turn) in start
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, i + 1))
        .rev()
        .skip(1)
    {
        low[val as usize] = turn as u32;
    }
    let mut to_consider = *start.last().unwrap();
    for i in start.len() as u32..n {
        if to_consider < BOUNDARY {
            let lownum = &mut low[to_consider as usize];
            to_consider = if *lownum == 0 { 0 } else { i - *lownum };
            *lownum = i;
        } else {
            match high.entry(to_consider) {
                Entry::Occupied(mut occup) => to_consider = i - occup.insert(i),
                Entry::Vacant(vacant) => {
                    vacant.insert(i);
                    to_consider = 0;
                }
            }
        }
    }
    to_consider
}

fn part_one(start: &[u32]) -> u32 {
    get_nth(start, TARGET1)
}

fn part_two(start: &[u32]) -> u32 {
    get_nth(start, TARGET2)
}

#[test]
#[ignore]
fn test_part_one_examples() {
    // Tests are run in separate threads with less stack size, so this test
    // needs to be run with `RUST_MIN_STACK=5000000 cargo test -- --ignored`
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
}

#[test]
#[ignore]
fn test_part_two_examples() {
    // This test takes a while to run, so is ignored.
    // It can be run with `RUST_MIN_STACK=5000000 cargo test -- --ignored`
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
