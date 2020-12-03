use std::time::Instant;

#[derive(Debug)]
struct TreeMap {
    // Outer Vec is columns, inner Vec is rows
    trees: Vec<Vec<bool>>,
}

impl TreeMap {
    fn from_str(input: &str) -> Self {
        let mut trees = Vec::new();
        for line in input.lines().filter(|l| l.len() >= 1) {
            let inner: Vec<bool> = line
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect();
            trees.push(inner);
        }
        TreeMap { trees }
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {
        // |----> x
        // v y
        let row = self.trees.get(y).unwrap();
        let mut x = x;
        loop {
            if x < row.len() {
                return *row.get(x).unwrap();
            } else {
                x -= row.len()
            }
        }
    }

    fn is_finished(&self, y: usize) -> bool {
        y >= self.trees.len()
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/03")?;
    let tree_map = parse_input(&input);
    println!("Part 1: {}", part_one(&tree_map, 1, 3));
    println!("Part 2: {}", part_two(&tree_map));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> TreeMap {
    TreeMap::from_str(input)
}

fn part_one(tree_map: &TreeMap, y_step: usize, x_step: usize) -> u32 {
    // Right 3, down 1
    let mut x = 0;
    let mut y = 0;
    let mut num_trees = 0;
    while !tree_map.is_finished(y) {
        if tree_map.is_tree(x, y) {
            num_trees += 1
        }
        x += x_step;
        y += y_step;
    }
    num_trees
}

fn part_two(tree_map: &TreeMap) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|t| part_one(&tree_map, t.1, t.0))
        .product()
}
#[test]
fn test_examples() {
    let input = "
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";
    let tree_map = parse_input(&input);
    assert_eq!(part_one(&tree_map, 1, 3), 7);
    assert_eq!(part_two(&tree_map), 336);
}
