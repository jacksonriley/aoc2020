use aoc2020::find_all_positions;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Bags<'a> = HashMap<&'a str, HashSet<(u32, &'a str)>>;
type ParentBags<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/07")?;
    let (bags, parent_bags) = parse_input(&input);
    println!("Time: {}µs", now.elapsed().as_micros());
    println!("Part 1: {}", part_one(&parent_bags));
    println!("Part 2: {}", part_two(&bags));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> (Bags, ParentBags) {
    // Possible line formats:
    // shiny teal bags contain 1 posh green bag, 5 pale indigo bags, 1 mirrored purple bag.
    // pale coral bags contain no other bags.
    let mut bags = HashMap::new();
    let mut parent_bags = HashMap::new();
    for line in input.lines() {
        let mut pc = line.split(" bags contain ");
        let parent = pc.next().unwrap();
        let mut children = HashSet::new();
        let children_str = pc.next().unwrap();
        if children_str != "no other bags." {
            for child in children_str.split(", ") {
                let mut space_positions = find_all_positions(child, ' ');
                let num_pos: usize = space_positions.next().unwrap();
                let post_iden: usize = space_positions.nth(1).unwrap();
                let num: u32 = child[..num_pos].parse().unwrap();
                let iden = &child[(num_pos + 1)..post_iden];
                children.insert((num, iden));
                parent_bags
                    .entry(iden)
                    .or_insert_with(HashSet::new)
                    .insert(parent);
            }
        }
        bags.insert(parent, children);
    }
    (bags, parent_bags)
}

fn recursive_count(bags: &Bags, key: &str) -> u32 {
    let mut count = 0;
    if let Some(set) = bags.get(key) {
        for (inner_bag_count, colour) in set.iter() {
            count += inner_bag_count * recursive_count(bags, &colour);
        }
    }
    count + 1
}

fn recursive_get_all_parents<'a>(parent_bags: &ParentBags<'a>, child: &str) -> HashSet<&'a str> {
    match parent_bags.get(child) {
        None => HashSet::new(),
        Some(parents) => parents
            .iter()
            .map(|p| recursive_get_all_parents(parent_bags, p))
            .fold(parents.clone(), |a, b| a.union(&b).cloned().collect()),
    }
}

fn part_one(parent_bags: &ParentBags) -> usize {
    recursive_get_all_parents(parent_bags, "shiny gold").len()
}

fn part_two(bags: &Bags) -> u32 {
    recursive_count(bags, "shiny gold") - 1
}

#[test]
fn test_parsing() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dotted black bags contain no other bags.";
    let (bags, parent_bags) = parse_input(&input);
    let mut expected: Bags = HashMap::new();
    let mut expected_lr = HashSet::new();
    expected_lr.insert((1u32, "bright white"));
    expected_lr.insert((2u32, "muted yellow"));
    expected.insert(&"light red", expected_lr);
    expected.insert(&"dotted black", HashSet::new());

    let mut expected_parent_bags = HashMap::new();
    let mut bw_parents = HashSet::new();
    bw_parents.insert("light red");
    expected_parent_bags.insert("bright white", bw_parents.clone());
    expected_parent_bags.insert("muted yellow", bw_parents);
    assert_eq!(bags, expected);
    assert_eq!(parent_bags, expected_parent_bags);
}

#[test]
fn test_examples() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let (bags, parent_bags) = parse_input(&input);
    assert_eq!(part_one(&parent_bags), 4);
    assert_eq!(part_two(&bags), 32);
}
