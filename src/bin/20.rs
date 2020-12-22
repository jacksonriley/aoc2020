use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pixel {
    Fill,
    Empty,
}

impl Pixel {
    fn to_digit(&self) -> char {
        match self {
            Self::Fill => '1',
            Self::Empty => '0',
        }
    }
}

type Id = u64;

#[derive(Debug)]
struct Tile {
    id: Id,
    pixels: Vec<Vec<Pixel>>,

    // Each tile has eight distinct sides (representable by a u16 with
    // Fill -> 1, Empty -> 0).
    //             0   1     2      3
    // Let this be Top Right Bottom Left
    sides: [u16; 4],

    // A mapping of side index to tile id for adjacent tiles.
    // This will therefore be of size {2,3,4} and have keys drawn from [0, 1, 2, 3]
    adjacent_tiles: HashMap<usize, Id>,
    orientation_fixed: bool,
}

impl Tile {
    fn from_str(input: &str) -> Self {
        let mut pixels = Vec::new();

        let mut lines = input.trim().lines();
        let id: u64 = lines
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse()
            .unwrap();

        for line in lines {
            pixels.push(
                line.chars()
                    .map(|c| match c {
                        '#' => Pixel::Fill,
                        '.' => Pixel::Empty,
                        _ => panic!("Got unexpected character in line {}", line),
                    })
                    .collect(),
            );
        }
        let sides = Self::calculate_sides(&pixels);
        Tile {
            id,
            pixels,
            sides,
            adjacent_tiles: HashMap::new(),
            orientation_fixed: false,
        }
    }

    fn calculate_sides(pixels: &[Vec<Pixel>]) -> [u16; 4] {
        // Represent sides as u16 for hopefully faster matching.
        // Orientation of mapping round the tile must be cyclical so they can
        // be rotated:
        // ----->
        // ^    |
        // |    |
        // |    v
        // <----
        // Hence the `.rev()` for bottom_str and left_str below
        let top_str: String = pixels[0].iter().map(|p| p.to_digit()).collect();

        let bottom_str: String = pixels
            .last()
            .unwrap()
            .iter()
            .map(|p| p.to_digit())
            .rev()
            .collect();

        let left_str: String = pixels.iter().map(|v| v[0].to_digit()).rev().collect();

        let right_str: String = pixels
            .iter()
            .map(|v| v.last().unwrap().to_digit())
            .collect();

        [
            u16::from_str_radix(&top_str, 2).unwrap(),
            u16::from_str_radix(&right_str, 2).unwrap(),
            u16::from_str_radix(&bottom_str, 2).unwrap(),
            u16::from_str_radix(&left_str, 2).unwrap(),
        ]
    }

    fn rotate(&mut self) {
        // Rotates 90 degrees clockwise
        rotate_pixels(&mut self.pixels);

        // Also rotate the sides
        self.sides.rotate_right(1);
    }

    fn flip(&mut self) {
        // Flips left-right
        for row in self.pixels.iter_mut() {
            row.reverse();
        }

        // Also flip the sides - as we're flipping left-right, this means that
        // Top becomes Top reverse
        // Right becomes Left reverse
        // Bottom becomes Bottom reverse
        // Left becomes Right reverse
        self.sides = [
            self.sides[0].reverse_bits() >> 6,
            self.sides[3].reverse_bits() >> 6,
            self.sides[2].reverse_bits() >> 6,
            self.sides[1].reverse_bits() >> 6,
        ];
    }
}

fn solve_jigsaw(tiles: &mut Vec<Tile>) {
    // Feels like a bit of a dumb algorithm, perhaps there's better? O(N^2).
    // Maintain a stack of tiles to process and a vec of tiles that are
    // definitely done (can't be a neighbour of any of the tiles left to
    // process).
    // While there are tiles to process, pop a tile and find all of its
    // neighbours by, for each side, looking for a tile in `tiles` which either
    //  * has a side matching the reverse of our side
    //  * has a side matching our side and whose orientation is permitted to
    //    change.
    // If there's a match, flip and rotate it into the correct orientation,
    // record the link between the tiles, and if the matched tile has four
    // neighbours, mark it as done. Otherwise, put the matched tile on the top
    // of the stack to be processed soon.
    // Once all sides of the popped tile have been checked, mark it as done.
    // Continue until the stack of tiles to process is empty.
    let mut done: Vec<Tile> = Vec::new();

    while let Some(mut tile_to_process) = tiles.pop() {
        tile_to_process.orientation_fixed = true;
        for (i, side) in tile_to_process
            .sides
            .iter()
            .map(|s| s.reverse_bits() >> 6)
            .enumerate()
        {
            // We're hunting for another tile with the reversed side to match
            // with ours.

            if let Some(side_match_idx) = tiles.iter().position(|t| {
                t.sides.contains(&side)
                    || (t.sides.contains(&(side.reverse_bits() >> 6)) && !t.orientation_fixed)
            }) {
                let mut side_match = tiles.remove(side_match_idx);
                if !side_match.orientation_fixed {
                    if !side_match.sides.contains(&side) {
                        // Needs to be flipped
                        side_match.flip();
                    }
                    while side_match.sides[(i + 2) % 4] != side {
                        // Needs to be rotated such that the i+2'th side matches with
                        // the i'th side of the anchor tile.
                        side_match.rotate();
                    }
                }
                assert_eq!(side, side_match.sides[(i + 2) % 4]);
                side_match.orientation_fixed = true;

                tile_to_process.adjacent_tiles.insert(i, side_match.id);
                side_match
                    .adjacent_tiles
                    .insert((i + 2) % 4, tile_to_process.id);

                if side_match.adjacent_tiles.len() == 4 {
                    // Remove this tile from further consideration - it has
                    // four neighbours.
                    done.push(side_match);
                } else {
                    // Push this back onto tiles so that it's popped next.
                    tiles.push(side_match);
                }
            }
        }
        // We're now done with this tile - all existing neighbours have been
        // found, so push it onto done.
        done.push(tile_to_process);
    }

    // Re-fill tiles
    tiles.extend(done);
}

fn trim_pixels(pixels: &[Vec<Pixel>]) -> Vec<Vec<Pixel>> {
    // Remove the outer 1-layer of pixels
    let mut output = pixels.to_owned();
    output.remove(0);
    output.pop();
    for row in output.iter_mut() {
        row.remove(0);
        row.pop();
    }
    output
}

fn rotate_pixels(pixels: &mut Vec<Vec<Pixel>>) {
    // Rotates 90 degrees clockwise
    let mut new_pixels: Vec<Vec<Pixel>> = Vec::new();
    let l = pixels[0].len();
    for i in 0..l {
        let mut new_row: Vec<Pixel> = Vec::new();
        for j in 1..=l {
            new_row.push(pixels[l - j][i]);
        }
        new_pixels.push(new_row);
    }
    *pixels = new_pixels;
}

fn flip_pixels(pixels: &mut Vec<Vec<Pixel>>) {
    // Flips left-right
    for row in pixels.iter_mut() {
        row.reverse();
    }
}

fn get_row(left: &Tile, map: &HashMap<Id, &Tile>) -> Vec<Vec<Pixel>> {
    let mut curr = left;
    let mut tile_block: Vec<Vec<Pixel>> = trim_pixels(&curr.pixels);
    while let Some(curr_id) = curr.adjacent_tiles.get(&1) {
        curr = map.get(curr_id).unwrap();
        for (r, row) in trim_pixels(&curr.pixels).into_iter().enumerate() {
            for p in row.into_iter() {
                tile_block[r].push(p);
            }
        }
    }
    tile_block
}

fn remove_borders(tiles: &[Tile]) -> Vec<Vec<Pixel>> {
    // Find the top left corner - the one that has no neighbours to the left
    // (3) or above (0)
    let mut top_left = tiles
        .iter()
        .find(|t| !t.adjacent_tiles.contains_key(&0) && !t.adjacent_tiles.contains_key(&3))
        .unwrap();

    let map: HashMap<Id, &Tile> = tiles.iter().map(|t| (t.id, t)).collect();
    let mut sea_map: Vec<Vec<Pixel>> = get_row(top_left, &map);

    // Iterate from top left to top right, then the row below, etc.
    let mut next_left = top_left.adjacent_tiles.get(&2);
    while let Some(top_left_id) = next_left {
        top_left = map.get(top_left_id).unwrap();

        sea_map.extend(get_row(top_left, &map));
        next_left = top_left.adjacent_tiles.get(&2);
    }

    sea_map
}

fn count_sea_monsters(sea_map: &[Vec<Pixel>]) -> usize {
    // Look for monsters, like
    //  --------------------
    // |                  # |
    // |#    ##    ##    ###|
    // | #  #  #  #  #  #   |
    //  --------------------
    // Enclosure is 3 pixels high and 20 pixels long.

    let rel_coords = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];
    let mut num_monsters = 0;
    for r in 0..sea_map.len() - 2 {
        for c in 0..sea_map[0].len() - 19 {
            if rel_coords
                .iter()
                .all(|(rr, rc)| sea_map[r + rr][c + rc] == Pixel::Fill)
            {
                num_monsters += 1;
            }
        }
    }

    num_monsters
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/20")?;
    let mut tiles = parse_input(&input);
    println!("Part 1: {}", part_one(&mut tiles));
    println!("Part 2: {}", part_two(&tiles));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn part_one(tiles: &mut Vec<Tile>) -> u64 {
    solve_jigsaw(tiles);
    tiles
        .iter()
        // The corners only have two neighbours
        .filter(|t| t.adjacent_tiles.len() == 2)
        .map(|t| t.id)
        .product()
}

fn part_two(tiles: &[Tile]) -> usize {
    let mut sea_map = remove_borders(tiles);

    let mut num_monsters = 0;

    'outer: for _ in 0..2 {
        for _ in 0..4 {
            rotate_pixels(&mut sea_map);
            num_monsters = count_sea_monsters(&sea_map);
            if num_monsters != 0 {
                break 'outer;
            }
        }
        flip_pixels(&mut sea_map);
    }

    // Sea monsters have 15 body parts. Assume no sea monsters overlap.
    sea_map
        .iter()
        .flatten()
        .filter(|&p| *p == Pixel::Fill)
        .count()
        - 15 * num_monsters
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(|t| Tile::from_str(t)).collect()
}

#[test]
fn test_examples() {
    let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    let mut tiles = parse_input(&input);
    assert_eq!(part_one(&mut tiles), 20899048083289);
    assert_eq!(part_two(&mut tiles), 273);
}
