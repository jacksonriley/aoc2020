use std::str::FromStr;
use std::time::Instant;

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, -1),
    (-1, 0),
    (0, -1),
    (-1, 1),
    (1, -1),
];

#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
}
use Part::*;

type Seats = Vec<Vec<Position>>;

#[derive(PartialEq, Clone)]
enum Position {
    Floor,
    Empty,
    Occupied,
}
use Position::*;

#[derive(PartialEq, Clone)]
struct SeatMap {
    seats: Seats,
    cols: usize,
    rows: usize,
}

impl FromStr for SeatMap {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Outer Vec is rows
        let mut seats: Seats = Vec::new();
        for line in input.lines() {
            let row = line
                .chars()
                .map(|c| match c {
                    '.' => Ok(Floor),
                    'L' => Ok(Empty),
                    '#' => Ok(Occupied),
                    _ => Err(format!("Got bad character {}", c)),
                })
                .collect::<Result<Vec<Position>, Self::Err>>()?;
            seats.push(row);
        }
        let cols = seats[0].len();
        let rows = seats.len();
        Ok(Self { seats, cols, rows })
    }
}

impl SeatMap {
    fn tick(&mut self, part: Part) -> bool {
        // Apply the evolution rules once to every position and update self.
        // Return whether or not the map changed.
        let mut changed = false;
        let mut new_seat_map: Seats = Vec::new();
        for r in 0..self.rows {
            let mut new_row = Vec::new();
            for c in 0..self.cols {
                let new_seat = match part {
                    Part1 => self.evolve1(r, c),
                    Part2 => self.evolve2(r, c),
                };
                if !changed && new_seat != self.seats[r][c] {
                    changed = true;
                }
                new_row.push(new_seat);
            }
            new_seat_map.push(new_row);
        }
        self.seats = new_seat_map;
        changed
    }

    fn evolve1(&mut self, row: usize, col: usize) -> Position {
        match self.seats[row][col] {
            Floor => Floor,
            Empty => {
                // If a seat is empty (L) and there are no occupied
                // seats adjacent to it, the seat becomes occupied.
                if self.get_occ_neighbours(row as isize, col as isize) == 0 {
                    Occupied
                } else {
                    // Otherwise, the seat's state does not change.
                    Empty
                }
            }
            Occupied => {
                // If a seat is occupied (#) and four or more seats
                // adjacent to it are also occupied, the seat becomes
                // empty.
                if self.get_occ_neighbours(row as isize, col as isize) >= 4 {
                    Empty
                } else {
                    // Otherwise, the seat's state does not change.
                    Occupied
                }
            }
        }
    }

    fn evolve2(&mut self, row: usize, col: usize) -> Position {
        match self.seats[row][col] {
            Floor => Floor,
            Empty => {
                // If a seat is empty (L) and there are no occupied
                // seats in view, the seat becomes occupied.
                if self.get_occ_neighbours_sightline(row as isize, col as isize) == 0 {
                    Occupied
                } else {
                    // Otherwise, the seat's state does not change.
                    Empty
                }
            }
            Occupied => {
                // If a seat is occupied (#) and five or more seats
                // in view are also occupied, the seat becomes empty.
                if self.get_occ_neighbours_sightline(row as isize, col as isize) >= 5 {
                    Empty
                } else {
                    // Otherwise, the seat's state does not change.
                    Occupied
                }
            }
        }
    }

    fn get_occ_neighbours(&self, row: isize, col: isize) -> usize {
        // Return the number of occupied neighbours
        DIRECTIONS
            .iter()
            .map(|p| (p.0 + row, p.1 + col))
            .filter(|p| self.in_bounds(*p) && self.seats[p.0 as usize][p.1 as usize] == Occupied)
            .count()
    }

    fn get_occ_neighbours_sightline(&self, row: isize, col: isize) -> usize {
        // Return the number of visible occupied seats in any direction
        DIRECTIONS
            .iter()
            .filter(|&d| self.occ_in_sightline(row, col, *d))
            .count()
    }

    fn occ_in_sightline(&self, row: isize, col: isize, direction: (isize, isize)) -> bool {
        // Return whether or not there is an occupied seat in this direction
        let mut new_pos = (row + direction.0, col + direction.1);
        while self.in_bounds(new_pos) {
            match self.seats[new_pos.0 as usize][new_pos.1 as usize] {
                Occupied => return true,
                Empty => return false,
                Floor => {}
            }
            new_pos = (new_pos.0 + direction.0, new_pos.1 + direction.1);
        }
        false
    }

    fn in_bounds(&self, point: (isize, isize)) -> bool {
        // Return whether or not the given point is within the bounds of the seating area
        point.0 >= 0
            && (point.0 as usize) < self.rows
            && point.1 >= 0
            && (point.1 as usize) < self.cols
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/11")?;
    let seat_map = parse_input(&input);
    println!("Part 1: {}", part_one(&seat_map));
    println!("Part 2: {}", part_two(&seat_map));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> SeatMap {
    SeatMap::from_str(input).unwrap()
}

fn run_part(seat_map: &SeatMap, part: Part) -> usize {
    let mut clone = seat_map.clone();
    while clone.tick(part) {
        continue;
    }
    clone
        .seats
        .iter()
        .flatten()
        .filter(|&s| s == &Occupied)
        .count()
}

fn part_one(seat_map: &SeatMap) -> usize {
    run_part(seat_map, Part1)
}

fn part_two(seat_map: &SeatMap) -> usize {
    run_part(seat_map, Part2)
}

#[test]
fn test_examples() {
    let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    let seat_map = parse_input(input);
    assert_eq!(part_one(&seat_map), 37);
    assert_eq!(part_two(&seat_map), 26);
}
