use std::time::Instant;

#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
}
use Part::*;

#[derive(Debug)]
enum Action {
    // Movements
    MoveForward(i32),
    MoveNorth(i32),
    MoveSouth(i32),
    MoveEast(i32),
    MoveWest(i32),
    // Turns
    Right,
    Left,
    Back,
}
use Action::*;

#[derive(Debug)]
struct Ship {
    heading: (i32, i32),
    pos: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    fn do_action(&mut self, action: &Action, part: Part) {
        match part {
            Part1 => match action {
                MoveForward(step) => {
                    self.pos = (
                        self.pos.0 + step * self.heading.0,
                        self.pos.1 + step * self.heading.1,
                    )
                }
                MoveNorth(_) => self.pos = do_cardinal_move(self.pos, action),
                MoveSouth(_) => self.pos = do_cardinal_move(self.pos, action),
                MoveEast(_) => self.pos = do_cardinal_move(self.pos, action),
                MoveWest(_) => self.pos = do_cardinal_move(self.pos, action),
                _ => self.heading = do_turn(self.heading, action),
            },
            Part2 => match action {
                MoveForward(step) => {
                    self.pos = (
                        self.pos.0 + step * self.waypoint.0,
                        self.pos.1 + step * self.waypoint.1,
                    )
                }
                MoveNorth(_) => self.waypoint = do_cardinal_move(self.waypoint, action),
                MoveSouth(_) => self.waypoint = do_cardinal_move(self.waypoint, action),
                MoveEast(_) => self.waypoint = do_cardinal_move(self.waypoint, action),
                MoveWest(_) => self.waypoint = do_cardinal_move(self.waypoint, action),
                _ => self.waypoint = do_turn(self.waypoint, action),
            },
        }
    }
    fn new() -> Self {
        Self {
            heading: (1, 0),
            pos: (0, 0),
            waypoint: (10, 1),
        }
    }
}

fn do_turn(direction: (i32, i32), action: &Action) -> (i32, i32) {
    match action {
        Right => (direction.1, -direction.0),
        Left => (-direction.1, direction.0),
        Back => (-direction.0, -direction.1),
        _ => panic!(
            "Got a move action when a turn action was expected: {:?}",
            action
        ),
    }
}

fn do_cardinal_move(original: (i32, i32), action: &Action) -> (i32, i32) {
    match action {
        MoveNorth(step) => (original.0, original.1 + step),
        MoveSouth(step) => (original.0, original.1 - step),
        MoveEast(step) => (original.0 + step, original.1),
        MoveWest(step) => (original.0 - step, original.1),
        _ => panic!("Got a non-cardinal move: {:?}", action),
    }
}

fn manhattan_distance(pos: (i32, i32)) -> u32 {
    pos.0.abs() as u32 + pos.1.abs() as u32
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/12")?;
    let actions = parse_input(&input);
    println!("Part 1: {}", part_one(&actions));
    println!("Part 2: {}", part_two(&actions));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Action> {
    let mut actions = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let verb = chars.next().unwrap();
        let value: i32 = chars.collect::<String>().parse().unwrap();
        let action = match verb {
            'F' => MoveForward(value),
            'N' => MoveNorth(value),
            'S' => MoveSouth(value),
            'E' => MoveEast(value),
            'W' => MoveWest(value),
            'L' => match value {
                90 => Left,
                180 => Back,
                270 => Right,
                _ => panic!("Got unexpected degree value {}", value),
            },
            'R' => match value {
                90 => Right,
                180 => Back,
                270 => Left,
                _ => panic!("Got unexpected degree value {}", value),
            },
            _ => panic!("Got bad character {}", verb),
        };
        actions.push(action);
    }
    actions
}

fn do_part(part: Part, actions: &[Action]) -> u32 {
    let mut ship = Ship::new();
    for action in actions.iter() {
        ship.do_action(action, part);
    }
    manhattan_distance(ship.pos)
}

fn part_one(actions: &[Action]) -> u32 {
    do_part(Part1, actions)
}

fn part_two(actions: &[Action]) -> u32 {
    do_part(Part2, actions)
}

#[test]
fn test_examples() {
    let input = "F10
N3
F7
R90
F11";
    let actions = parse_input(input);
    assert_eq!(
        actions,
        vec!(
            MoveForward(10),
            MoveNorth(3),
            MoveForward(7),
            Right,
            MoveForward(11)
        )
    );
    assert_eq!(part_one(&actions), 25);
    assert_eq!(part_two(&actions), 286);
}
