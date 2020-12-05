use std::time::Instant;

fn get_seat_id(input: &str) -> u16 {
    // The seat id is simply the FBLR representation mapped to 0 and 1 and
    // interpreted as a binary number.
    input
        .chars()
        .map(|c| match c {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .fold(0, |acc, digit| acc * 2 + digit)
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/05")?;
    let seats = parse_input(&input);
    println!("Part 1: {}", part_one(&seats));
    println!("Part 2: {}", part_two(&seats));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<u16> {
    input.lines().map(|l| get_seat_id(l)).collect()
}

fn part_one(seats: &[u16]) -> u16 {
    *seats.iter().max().unwrap()
}

fn part_two(seats: &[u16]) -> u16 {
    // Our seat is between the minimum and the maximum. Find the id in the
    // slice for which id + 2 is in the slice, but not id + 1. Then, id + 1 is
    // our seat. Not worth building a HashSet for such a short input.
    seats
        .iter()
        .find(|&id| !seats.contains(&(id + 1)) && seats.contains(&(id + 2)))
        .unwrap()
        + 1
}

#[test]
fn test_examples() {
    assert_eq!(get_seat_id(&"BFFFBBFRRR"), 567);
    assert_eq!(get_seat_id(&"FFFBBBFRRR"), 119);
    assert_eq!(get_seat_id(&"BBFFBBFRLL"), 820);
}
