use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/10")?;
    let numbers = parse_input(&input);
    println!("Part 1: {}", part_one(&numbers));
    // println!("Part 2: {}", part_two(&numbers, PREAMBLE_LEN));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_one(numbers: &[u32]) -> usize {
    // Sort the adapters, and then build up a 2-tuple, the first of which is
    // the number of 1-jolt differences, the second is the number of 3-jolt
    // differences. Their multiple is the answer to part 1.
    let mut sorted_numbers = numbers.to_owned();
    sorted_numbers.push(0);
    sorted_numbers.sort_unstable();
    sorted_numbers.push(sorted_numbers.last().unwrap() + 3);
    let jolts = sorted_numbers.windows(2).fold((0, 0), |acc, x| {
        if x[1] - x[0] == 1 {
            (acc.0 + 1, acc.1)
        } else if x[1] - x[0] == 3 {
            (acc.0, acc.1 + 1)
        } else {
            acc
        }
    });
    jolts.0 * jolts.1
}

#[test]
fn test_examples() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let numbers = parse_input(input);
    assert_eq!(part_one(&numbers), 220);
}
