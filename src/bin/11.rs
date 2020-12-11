use std::str::FromStr;
use std::time::Instant;

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
                    '.' => Ok(Position::Floor),
                    'L' => Ok(Position::Empty),
                    '#' => Ok(Position::Occupied),
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
    fn tick(&mut self, part: Part) {
        // Apply the evolution rules once to every position and update self.
        let mut new_seat_map: Seats = Vec::new();
        for r in 0..self.rows {
            let mut new_row = Vec::new();
            for c in 0..self.cols {
                let new_seat = match part {
                    Part1 => self.evolve1(r, c),
                    Part2 => self.evolve2(r, c),
                };
                new_row.push(new_seat);
            }
            new_seat_map.push(new_row);
        }
        self.seats = new_seat_map;
    }

    fn evolve1(&mut self, row: usize, col: usize) -> Position {
        match self.seats[row][col] {
            Position::Floor => Position::Floor,
            Position::Empty => {
                // If a seat is empty (L) and there are no occupied
                // seats adjacent to it, the seat becomes occupied.
                if self.get_occ_neighbours(row, col) == 0 {
                    Position::Occupied
                } else {
                    // Otherwise, the seat's state does not change.
                    Position::Empty
                }
            }
            Position::Occupied => {
                // If a seat is occupied (#) and four or more seats
                // adjacent to it are also occupied, the seat becomes
                // empty.
                if self.get_occ_neighbours(row, col) >= 4 {
                    Position::Empty
                } else {
                    // Otherwise, the seat's state does not change.
                    Position::Occupied
                }
            }
        }
    }

    fn evolve2(&mut self, row: usize, col: usize) -> Position {
        match self.seats[row][col] {
            Position::Floor => Position::Floor,
            Position::Empty => {
                // If a seat is empty (L) and there are no occupied
                // seats in view, the seat becomes occupied.
                if self.get_occ_neighbours_sightline(row, col) == 0 {
                    Position::Occupied
                } else {
                    // Otherwise, the seat's state does not change.
                    Position::Empty
                }
            }
            Position::Occupied => {
                // If a seat is occupied (#) and five or more seats
                // in view are also occupied, the seat becomes empty.
                if self.get_occ_neighbours_sightline(row, col) >= 5 {
                    Position::Empty
                } else {
                    // Otherwise, the seat's state does not change.
                    Position::Occupied
                }
            }
        }
    }

    fn get_occ_neighbours(&self, row: usize, col: usize) -> usize {
        // Return the number of occupied neighbours
        let row_i = row as isize;
        let col_i = col as isize;
        let directions = [
            (1, 0),
            (0, 1),
            (1, 1),
            (-1, -1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (1, -1),
        ];
        directions
            .iter()
            .map(|p| (p.0 + row_i, p.1 + col_i))
            .filter(|p| self.in_bounds(*p))
            .filter(|valid| self.seats[valid.0 as usize][valid.1 as usize] == Position::Occupied)
            .count()
    }

    fn get_occ_neighbours_sightline(&self, row: usize, col: usize) -> usize {
        // Return the number of visible occupied seats in any direction
        let row_i = row as isize;
        let col_i = col as isize;
        let directions = [
            (1, 0),
            (0, 1),
            (1, 1),
            (-1, -1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (1, -1),
        ];
        directions
            .iter()
            .filter(|&d| self.occ_in_sightline(row_i, col_i, *d))
            .count()
    }

    fn occ_in_sightline(&self, row: isize, col: isize, direction: (isize, isize)) -> bool {
        // Return whether or not there is an occupied seat in this direction
        let mut new_pos = (row + direction.0, col + direction.1);
        while self.in_bounds(new_pos) {
            match self.seats[new_pos.0 as usize][new_pos.1 as usize] {
                Position::Occupied => return true,
                Position::Empty => return false,
                Position::Floor => {}
            }
            new_pos = (new_pos.0 + direction.0, new_pos.1 + direction.1);
        }
        false
    }

    fn in_bounds(&self, point: (isize, isize)) -> bool {
        // Return whether or not the given point is within the bounds of the seating area
        point.0 >= 0 && (point.0 as usize) < self.rows && point.1 >= 0 && (point.1 as usize) < self.cols
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
    let mut old_seats = clone.seats.clone();
    clone.tick(part);
    while clone.seats != old_seats {
        old_seats = clone.seats.clone();
        clone.tick(part);
    }
    clone
        .seats
        .iter()
        .flatten()
        .filter(|&s| s == &Position::Occupied)
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
