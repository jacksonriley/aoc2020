use std::collections::HashMap;
use std::time::Instant;

fn num_paths(adapters: &[u32], index: usize, seen: &mut HashMap<usize, u64>) -> u64 {
    // Recursively find the number of paths to the given index through the
    // given slice, as follows:
    //  * If the index is 0, there is one path to the index.
    //  * Otherwise the number of paths to the given index is simply the sum of
    //    the paths to all of the preceding indices which can reach the given
    //    index.
    // Cache the results for performance (Fibonacci-esque).
    if seen.contains_key(&index) {
        *seen.get(&index).unwrap()
    } else if index == 0 {
        1
    } else {
        let mut total = 0;
        let mut back = 1;
        while back <= index && adapters[index] - adapters[index - back] <= 3 {
            total += num_paths(adapters, index - back, seen);
            back += 1;
        }
        seen.insert(index, total);
        total
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/10")?;
    let adapters = parse_input(&input);
    println!("Part 1: {}", part_one(&adapters));
    println!("Part 2: {}", part_two(&adapters));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<u32> {
    // Return a sorted Vec of the input adapter joltages, including the ingress
    // and egress joltages.
    let mut raw: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    raw.push(0);
    raw.sort_unstable();
    raw.push(raw.last().unwrap() + 3);
    raw
}

fn part_one(adapters: &[u32]) -> usize {
    // Map to a Vec of the differences in the adapters.
    // The answer is the number of 1-jolt differences multiplied by the number
    // of 3-jolt differences.
    let differences: Vec<u32> = adapters.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let jolts_1 = differences.iter().filter(|&d| *d == 1).count();
    let jolts_3 = differences.iter().filter(|&d| *d == 3).count();
    jolts_1 * jolts_3
}

fn part_two(adapters: &[u32]) -> u64 {
    // The number of valid arrangements is the same as the number of paths to
    // the device's adapter.
    num_paths(adapters, adapters.len() - 1, &mut HashMap::new())
}

#[test]
fn test_examples() {
    let short_input = "16
10
15
5
1
11
7
19
6
12
4";
    let numbers = parse_input(short_input);
    assert_eq!(part_one(&numbers), 35);
    assert_eq!(part_two(&numbers), 8);

    let long_input = "28
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
    let numbers = parse_input(long_input);
    assert_eq!(part_one(&numbers), 220);
    assert_eq!(part_two(&numbers), 19208);
}
