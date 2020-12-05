use std::collections::HashSet;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Seat {
    row: u8,
    column: u8,
    seat_id: usize,
}

impl FromStr for Seat {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // e.g. FBFBFFBLLR -> 0101001 001 -> row 41 column 1
        if input.len() != 10
            || !&input[0..=6].chars().all(|c| ['F', 'B'].contains(&c))
            || !input[7..=9].chars().all(|c| ['L', 'R'].contains(&c))
        {
            return Err(format!("Incorrect input: {}", input));
        }
        let row: u8 = input[0..=6]
            .chars()
            .map(|c| match c {
                'F' => 0,
                'B' => 1,
                _ => unreachable!(),
            })
            .fold(0, |acc, b| acc * 2 + b);
        let column: u8 = input[7..=9]
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => unreachable!(),
            })
            .fold(0, |acc, b| acc * 2 + b);
        let seat_id = row as usize * 8 + column as usize;
        Ok(Self {
            row,
            column,
            seat_id,
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/05")?;
    let seats = parse_input(&input);
    println!("Part 1: {}", part_one(&seats));
    println!("Part 2: {}", part_two(&seats).expect("Couldn't find our seat"));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Seat> {
    input.lines().map(|l| Seat::from_str(l).unwrap()).collect()
}

fn part_one(seats: &[Seat]) -> usize {
    seats.iter().map(|s| s.seat_id).max().unwrap()
}

fn part_two(seats: &[Seat]) -> Option<usize> {
    // Our seat is between the minimum and the maximum.
    let minimum = seats.iter().map(|s| s.seat_id).min().unwrap();
    let maximum = seats.iter().map(|s| s.seat_id).max().unwrap();
    let all_ids: HashSet<usize> = seats.iter().map(|s| s.seat_id).collect();
    for id in minimum..maximum {
        if !all_ids.contains(&id) {
            return Some(id);
        }
    }
    None
}

#[test]
fn test_examples() {
    assert_eq!(Seat::from_str(&"BFFFBBFRRR").unwrap().seat_id, 567);
    assert_eq!(Seat::from_str(&"FFFBBBFRRR").unwrap().seat_id, 119);
    assert_eq!(Seat::from_str(&"BBFFBBFRLL").unwrap().seat_id, 820);
}
