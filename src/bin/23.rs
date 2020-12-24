use std::time::Instant;

type Cups = Vec<Cup>;

// Poor man's linked list - just store the next value, and use a Vec of all
// Cups to access it
#[derive(Debug, Clone)]
struct Cup {
    value: u32,
    next: u32,
}

fn cup_wrapping_sub_one(cup: u32, max_cup: u32) -> u32 {
    // Subtracts one, wrapping to stay in the range 1..=NUM_CUPS
    if cup == 1 {
        max_cup
    } else {
        cup - 1
    }
}

fn make_move(cups: &Cups, first_cup: u32, num_times: usize, max_cup: u32) -> Cups {
    let mut cups = cups.to_owned();
    let mut current_cup_val = first_cup;
    for _ in 0..num_times {
        // Pick up the three cups after the current cup
        let mut picked_cups: Vec<u32> = Vec::new();
        let mut picked_head = cups.get(current_cup_val as usize - 1).unwrap().next;
        for _ in 0..3 {
            picked_cups.push(cups.get(picked_head as usize - 1).unwrap().value);
            picked_head = cups.get(picked_head as usize - 1).unwrap().next;
        }

        // The destination label is the first label below the current cup's
        // label which is not in the group just picked up.
        let mut dest_label = cup_wrapping_sub_one(current_cup_val, max_cup);
        while picked_cups.contains(&dest_label) {
            dest_label = cup_wrapping_sub_one(dest_label, max_cup)
        }
        let dest_cup_next = cups.get(dest_label as usize - 1).unwrap().next;

        // The picked-up cups are written to the point after this dest_cup.
        // This can be done by
        //  * making the current cup point to the picked tail's next
        //  * making the picked tail point to dest_cup's next
        //  * making dest_cup point to the picked head
        let picked_tail_cup_value = *picked_cups.last().unwrap();
        let picked_tail_cup_next = cups.get(picked_tail_cup_value as usize - 1).unwrap().next;
        let picked_head_cup_value = picked_cups[0];

        cups.get_mut(current_cup_val as usize - 1).unwrap().next = picked_tail_cup_next;
        cups.get_mut(picked_tail_cup_value as usize - 1)
            .unwrap()
            .next = dest_cup_next;
        cups.get_mut(dest_label as usize - 1).unwrap().next = picked_head_cup_value;

        current_cup_val = cups.get(current_cup_val as usize - 1).unwrap().next;
    }
    cups.to_vec()
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/23")?;
    let (cups, first_cup) = parse_input(&input);
    println!("Cups: {:?}", cups);
    println!("Part 1: {}", part_one(&cups, first_cup));
    println!("Part 2: {}", part_two(&cups, first_cup));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> (Cups, u32) {
    // Return a Vec of Cups, where the last Cup points to the first Cup.
    let mut cups: Vec<Cup> = Vec::new();
    let values: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut windows = values.windows(2);
    while let Some(window) = windows.next() {
        cups.push(Cup {
            value: window[0],
            next: window[1],
        });
    }
    cups.push(Cup {
        value: *values.last().unwrap(),
        next: values[0],
    });

    let first_cup = cups[0].value;
    // Sort the cups so that the cup with value n is at index n - 1
    cups.sort_by_key(|c| c.value);
    (cups, first_cup)
}

fn part_one(cups: &Cups, first_cup: u32) -> String {
    let moved_cups = make_move(&cups, first_cup, 100, 9);
    println!("Moved cups: {:?}", moved_cups);
    let mut output_string = String::new();
    let mut cup_val = moved_cups.get(0).unwrap().next;
    while cup_val != 1 {
        output_string.push(std::char::from_u32(cup_val + '0' as u32).unwrap());
        cup_val = moved_cups.get(cup_val as usize - 1).unwrap().next;
    }
    output_string
}

fn part_two(cups: &Cups, first_cup: u32) -> u64 {
    let mut big_boy: Cups = (1..=1_000_000_u32)
        .map(|n| Cup {
            value: n,
            next: n + 1,
        })
        .collect();
    for (i, c) in cups.iter().enumerate() {
        big_boy[i] = c.clone();
    }

    // Fix up circular pointers
    let small_end = cups.iter().find(|cup| cup.next == first_cup).unwrap().value;
    big_boy[small_end as usize - 1].next = cups.len() as u32 + 1;
    big_boy.last_mut().unwrap().next = first_cup;

    let moved_cups = make_move(&big_boy, first_cup, 10_000_000, 1_000_000);
    let post_one_cup = moved_cups.get(0).unwrap().next;
    let post_post_one_cup = moved_cups.get(post_one_cup as usize - 1).unwrap().next;

    post_one_cup as u64 * post_post_one_cup as u64
}

#[test]
fn test_examples() {
    let input = "389125467";
    let (cups, first_cup) = parse_input(&input);
    assert_eq!(part_one(&cups, first_cup), "67384529");
    assert_eq!(part_two(&cups, first_cup), 149245887792);
}
