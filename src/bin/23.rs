use std::collections::VecDeque;
use std::time::Instant;

type Cups = VecDeque<u32>;

fn cup_wrapping_sub_one(cup: u32, max_cup: u32) -> u32 {
    // Subtracts one, wrapping to stay in the range 1..=NUM_CUPS
    if cup == 1 {
        max_cup
    } else {
        cup - 1
    }
}

fn make_move(cups: &Cups, num_times: usize, max_cup: u32) -> Cups {
    let mut cups = cups.clone();
    for i in 0..num_times {
        if i % 1_000 == 0 {
            println!("{}", i);
        }
        // The destination label is the first label below the current cup's
        // label which is not in the group just picked up.
        let mut dest_label = cup_wrapping_sub_one(cups[0], max_cup);
        while cups.iter().skip(1).take(3).any(|c| c == &dest_label) {
            dest_label = cup_wrapping_sub_one(dest_label, max_cup)
        }
        let dest_pos = cups.iter().position(|c| c == &dest_label).unwrap();

        // The picked-up cups are written to the point after this dest_pos.
        // This can be done by rotating cups[1..dest_pos] left by 3
        cups.make_contiguous()[1..=dest_pos].rotate_left(3);

        // Finally, rotate the slice such that the new current cup is at the
        // start.
        cups.rotate_left(1)
    }
    cups
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/23")?;
    let cups = parse_input(&input);
    println!("Cups: {:?}", cups);
    println!("Part 1: {}", part_one(&cups));
    println!("Part 2: {}", part_two(&cups));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Cups {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn part_one(cups: &Cups) -> String {
    let mut moved_cups = make_move(&cups, 100, 9);
    let pos_1 = moved_cups.iter().position(|c| c == &1).unwrap();
    moved_cups.rotate_left(pos_1);
    moved_cups
        .iter()
        .skip(1)
        .map(|c| std::char::from_u32(*c + '0' as u32).unwrap())
        .collect()
}

fn part_two(cups: &Cups) -> u64 {
    let mut big_boy: Cups = (1..=1_000_000_u32).collect();
    for (i, c) in cups.iter().enumerate() {
        big_boy[i] = *c;
    }
    let mut moved_cups = make_move(&big_boy, 10_000_000, 1_000_000);
    let pos_1 = moved_cups.iter().position(|c| c == &1).unwrap();
    moved_cups.rotate_left(pos_1);
    moved_cups[1] as u64 * moved_cups[2] as u64
}

#[test]
fn test_examples() {
    let input = "389125467";
    let cups = parse_input(&input);
    assert_eq!(part_one(&cups), "67384529");
}
