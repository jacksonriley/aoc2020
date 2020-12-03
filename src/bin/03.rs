use std::time::Instant;

#[derive(Debug)]
struct TreeMap {
    // Outer Vec is columns, inner Vec is rows
    trees: Vec<Vec<bool>>,
}

impl TreeMap {
    fn from_str(input: &str) -> Self {
        let mut trees = Vec::new();
        for line in input.lines().filter(|l| !l.is_empty()) {
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
        *row.get(x % row.len()).unwrap()
    }

    fn count_trees(&self, y_step: usize, x_step: usize) -> usize {
        let mut num_trees = 0;
        let mut x = 0;
        let mut y = 0;

        while y < self.trees.len() {
            if self.is_tree(x, y) {
                num_trees += 1;
            }
            x += x_step;
            y += y_step;
        }
        num_trees
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/03")?;
    let tree_map = parse_input(&input);
    println!("Part 1: {}", part_one(&tree_map));
    println!("Part 2: {}", part_two(&tree_map));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> TreeMap {
    TreeMap::from_str(input)
}

fn part_one(tree_map: &TreeMap) -> usize {
    tree_map.count_trees(1, 3)
}

fn part_two(tree_map: &TreeMap) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|t| tree_map.count_trees(t.1, t.0))
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
    assert_eq!(part_one(&tree_map), 7);
    assert_eq!(part_two(&tree_map), 336);
}
