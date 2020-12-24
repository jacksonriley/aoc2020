use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Displacement = Vec<Step>;
type Position = (isize, isize);

#[derive(Debug)]
enum Step {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Step {
    fn to_tuple(&self) -> Position {
        // Use two basis vectors of (E, NE)
        match self {
            Self::E => (1, 0),
            Self::W => (-1, 0),
            Self::NE => (0, 1),
            Self::NW => (-1, 1),
            Self::SE => (1, -1),
            Self::SW => (0, -1),
        }
    }

    fn iterator() -> impl Iterator<Item = Position> {
        [Self::E, Self::W, Self::NE, Self::NW, Self::SE, Self::SW]
            .iter()
            .map(|s| s.to_tuple())
    }
}

fn parse_input(input: &str) -> Vec<Displacement> {
    let mut displacements: Vec<Displacement> = Vec::new();
    for line in input.lines() {
        let mut displacement: Vec<Step> = Vec::new();
        let mut line = line.trim().chars();
        while let Some(c) = line.next() {
            // We might match an e or a w and be done, or might have to keep
            // going for the two-letter directions.
            match c {
                'e' => displacement.push(Step::E),
                'w' => displacement.push(Step::W),
                first_char => match line.next().unwrap() {
                    'e' => {
                        if first_char == 'n' {
                            displacement.push(Step::NE)
                        } else {
                            assert_eq!(first_char, 's');
                            displacement.push(Step::SE)
                        }
                    }
                    'w' => {
                        if first_char == 'n' {
                            displacement.push(Step::NW)
                        } else {
                            assert_eq!(first_char, 's');
                            displacement.push(Step::SW)
                        }
                    }
                    other => panic!("Got unexpected second char: {}", other),
                },
            }
        }
        displacements.push(displacement)
    }
    displacements
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/24")?;
    let displacements = parse_input(&input);
    println!("Part 1: {}", part_one(&displacements));
    println!("Part 2: {}", part_two(&displacements));
    println!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

fn get_starting_grid(displacements: &[Displacement]) -> HashSet<Position> {
    let mut black_cells: HashSet<Position> = HashSet::new();
    for displacement in displacements.iter() {
        let mut pos = (0, 0);
        for step in displacement.iter() {
            let step_tuple = step.to_tuple();
            pos.0 += step_tuple.0;
            pos.1 += step_tuple.1;
        }
        if black_cells.contains(&pos) {
            black_cells.remove(&pos);
        } else {
            black_cells.insert(pos);
        }
    }
    black_cells
}

fn part_one(displacements: &[Displacement]) -> usize {
    let black_cells = get_starting_grid(displacements);
    black_cells.len()
}

fn part_two(displacements: &[Displacement]) -> usize {
    let mut active_cells = get_starting_grid(displacements);
    for _ in 0..100 {
        let mut new_active: HashSet<Position> = HashSet::new();
        let mut node_to_num_active: HashMap<Position, u32> = HashMap::new();
        // For each active cell, increase the count of all its neighbours
        // by one - this is then the mapping of all cells with one or more
        // active neighbours to their active neighbour count
        active_cells
            .iter()
            .map(|p| Step::iterator().map(move |s| (s.0 + p.0, s.1 + p.1)))
            .for_each(|neighbours| {
                for n in neighbours {
                    *node_to_num_active.entry(n).or_insert(0) += 1;
                }
            });

        for p in active_cells.iter() {
            // If a cell is active and exactly 1 or 2 of its neighbours are
            // also active, the cell remains active. Otherwise, the cell
            // becomes inactive.
            let num_active_neighbours = match node_to_num_active.get(p) {
                Some(num) => *num,
                None => 0,
            };
            if num_active_neighbours == 1 || num_active_neighbours == 2 {
                new_active.insert(*p);
            }
        }
        for (p, num_active) in node_to_num_active.iter() {
            // If a cell is inactive but exactly 3 of its neighbours are
            // active, the cell becomes active. Otherwise, the cell remains
            // inactive.
            if !active_cells.contains(p) && *num_active == 2 {
                new_active.insert(*p);
            }
        }
        active_cells = new_active;
    }
    active_cells.len()
}

#[test]
fn test_examples() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let displacements = parse_input(&input);
    assert_eq!(part_one(&displacements), 10);
    assert_eq!(part_two(&displacements), 2208);
}
