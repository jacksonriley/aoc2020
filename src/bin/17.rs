use std::collections::HashSet;
use std::time::Instant;

type Position = Vec<i32>;

#[derive(PartialEq, Debug)]
struct ConwayCube {
    dimension: usize,
    active: HashSet<Position>,
    directions: Vec<Position>,
}

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    let mut initial_active = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let '#' = c {
                initial_active.insert((x, y));
            }
        }
    }
    initial_active
}

impl ConwayCube {
    fn new(dimension: usize, initial_active: &HashSet<(usize, usize)>) -> Self {
        let mut active = HashSet::new();
        for low_dim_active in initial_active.iter() {
            let mut active_pos = vec![low_dim_active.0 as i32, low_dim_active.1 as i32];
            while active_pos.len() != dimension {
                active_pos.push(0i32);
            }
            active.insert(active_pos);
        }
        Self {
            dimension,
            active,
            directions: get_directions(dimension),
        }
    }

    fn tick(&mut self, num_ticks: usize) -> usize {
        for _ in 0..num_ticks {
            let mut new_active: HashSet<Position> = HashSet::new();
            let inactive_neighbours: HashSet<Position> = self
                .active
                .iter()
                .flat_map(|p| self.get_neighbours(p).into_iter())
                .filter(|n| !self.active.contains(n))
                .collect();
            for p in self.active.iter() {
                // If a cube is active and exactly 2 or 3 of its neighbors are
                // also active, the cube remains active. Otherwise, the cube
                // becomes inactive.
                let num_active_neighbours = self.get_num_active_neighbours(&p);
                if num_active_neighbours == 2 || num_active_neighbours == 3 {
                    new_active.insert(p.to_vec());
                }
            }
            for p in inactive_neighbours {
                // If a cube is inactive but exactly 3 of its neighbors are
                // active, the cube becomes active. Otherwise, the cube remains
                // inactive.
                let num_active_neighbours = self.get_num_active_neighbours(&p);
                if num_active_neighbours == 3 {
                    new_active.insert(p);
                }
            }
            self.active = new_active;
        }

        self.active.len()
    }

    fn get_neighbours(&self, point: &[i32]) -> Vec<Position> {
        let mut neighbours = Vec::new();
        for d in &self.directions {
            neighbours.push(vector_add(&point, &d));
        }
        neighbours
    }

    fn get_num_active_neighbours(&self, point: &[i32]) -> usize {
        let neighbours = self.get_neighbours(point);
        neighbours
            .iter()
            .filter(|&n| self.active.contains(n))
            .count()
    }
}

fn get_directions(dimension: usize) -> Vec<Position> {
    let mut directions = vec![vec![0i32; dimension]];
    for d in 0..dimension {
        let mut ones = directions.clone();
        for v in &mut ones {
            v[d] = 1;
        }
        let mut minus_ones = directions.clone();
        for v in &mut minus_ones {
            v[d] = -1;
        }
        directions.extend(ones);
        directions.extend(minus_ones);
    }
    // Don't include the all-zero element!
    directions.remove(0);

    directions
}

fn vector_add(a: &[i32], b: &[i32]) -> Position {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/17")?;
    let initial = parse_input(&input);
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 1: {}", part_one(&initial));
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 2: {}", part_two(&initial));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn part_one(initial_active: &HashSet<(usize, usize)>) -> usize {
    let mut cube = ConwayCube::new(3, initial_active);
    cube.tick(6)
}

fn part_two(initial_active: &HashSet<(usize, usize)>) -> usize {
    let mut cube = ConwayCube::new(4, initial_active);
    cube.tick(6)
}

#[test]
fn test_examples() {
    let input = ".#.
..#
###";
    let initial = parse_input(input);
    assert_eq!(part_one(&initial), 112);
    assert_eq!(part_two(&initial), 848);
}
