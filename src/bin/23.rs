use std::time::Instant;

fn cup_wrapping_sub_one(cup: u32, max_cup: u32) -> u32 {
    // Subtracts one, wrapping to stay in the range 1..=max_cup
    if cup == 1 {
        max_cup
    } else {
        cup - 1
    }
}

fn make_move(mut cups: Vec<u32>, first_cup: u32, num_times: usize) -> Vec<u32> {
    let max_cup = cups.len() as u32;
    let mut current_cup = first_cup;
    for _ in 0..num_times {
        // Pick up the three cups after the current cup
        let picked_head = cups[current_cup as usize - 1];
        let picked_mid = cups[picked_head as usize - 1];
        let picked_tail = cups[picked_mid as usize - 1];

        // The destination label is the first label below the current cup's
        // label which is not in the group just picked up.
        let mut dest_label = cup_wrapping_sub_one(current_cup, max_cup);
        while dest_label == picked_head || dest_label == picked_mid || dest_label == picked_tail {
            dest_label = cup_wrapping_sub_one(dest_label, max_cup)
        }
        let dest_cup_next = cups[dest_label as usize - 1];

        // The picked-up cups are written to the point after this dest_cup.
        // This can be done by
        //  * making the current cup point to the picked tail's next
        //  * making the picked tail point to dest_cup's next
        //  * making dest_cup point to the picked head
        let picked_tail_cup_next = cups[picked_tail as usize - 1];

        cups[current_cup as usize - 1] = picked_tail_cup_next;
        cups[picked_tail as usize - 1] = dest_cup_next;
        cups[dest_label as usize - 1] = picked_head;

        current_cup = cups[current_cup as usize - 1];
    }
    cups
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/23")?;
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn create_cup_vec(input: &str, max_cup: u32) -> (Vec<u32>, u32) {
    // Return a Vec of cups, where the last cup points to the first cup. Also
    // return the first cup.
    // The cup with label n is at position n - 1, and has the value of the next
    // cup's label. A Vec-based linked list, I guess?
    let mut cups: Vec<u32> = vec![0u32; max_cup as usize];
    let mut values: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if max_cup as usize > values.len() {
        values.extend((values.len() as u32 + 1)..=max_cup);
    }

    let windows = values.windows(2);
    for window in windows {
        cups[window[0] as usize - 1] = window[1]
    }

    // Make the tail point to the head
    cups[*values.last().unwrap() as usize - 1] = values[0];

    let first_cup = values[0];
    (cups, first_cup)
}

fn part_one(input: &str) -> String {
    let (cups, first_cup) = create_cup_vec(input, 9);
    let moved_cups = make_move(cups, first_cup, 100);
    let mut output_string = String::new();
    let mut cup_val = moved_cups[0];
    while cup_val != 1 {
        output_string.push(std::char::from_u32(cup_val + '0' as u32).unwrap());
        cup_val = moved_cups[cup_val as usize - 1];
    }
    output_string
}

fn part_two(input: &str) -> u64 {
    let (cups, first_cup) = create_cup_vec(input, 1_000_000);

    let moved_cups = make_move(cups, first_cup, 10_000_000);
    let post_one_cup = moved_cups[0];
    let post_post_one_cup = moved_cups[post_one_cup as usize - 1];

    post_one_cup as u64 * post_post_one_cup as u64
}

#[test]
fn test_part_one_example() {
    let input = "389125467";
    assert_eq!(part_one(&input), "67384529");
}

#[test]
#[ignore]
fn test_part_two_example() {
    // This test takes a while to run, so is ignored.
    // It can be run with `cargo test -- --ignored`
    let input = "389125467";
    assert_eq!(part_two(&input), 149245887792);
}
