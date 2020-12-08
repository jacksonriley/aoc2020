use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Instruction {
    Jump(isize),
    Acc(i32),
    Noop(isize),
}

enum ProgramHalt {
    NormalExit(i32),
    Loop(i32),
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: i32,
    program_counter: usize,
}

impl FromStr for Program {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "jmp" => instructions.push(Instruction::Jump(parts.next().unwrap().parse()?)),
                "acc" => instructions.push(Instruction::Acc(parts.next().unwrap().parse()?)),
                "nop" => instructions.push(Instruction::Noop(parts.next().unwrap().parse()?)),
                _ => panic!("Got unexpected instruction: {}!", line),
            }
        }
        Ok(Program {
            instructions,
            accumulator: 0,
            program_counter: 0,
        })
    }
}

impl Program {
    fn run(&mut self) -> ProgramHalt {
        let mut seen_positions = HashSet::new();
        while !seen_positions.contains(&self.program_counter) {
            if self.program_counter == self.instructions.len() {
                return ProgramHalt::NormalExit(self.accumulator);
            }
            seen_positions.insert(self.program_counter);
            match self.instructions[self.program_counter] {
                Instruction::Jump(jump_len) => {
                    // Increment the program counter by the jump_len
                    self.program_counter = (self.program_counter as isize + jump_len) as usize;
                }
                Instruction::Acc(acc_size) => {
                    // Add to the accumulator and increment the program counter
                    // by 1
                    self.accumulator += acc_size;
                    self.program_counter += 1;
                }
                Instruction::Noop(_) => self.program_counter += 1,
            }
        }
        ProgramHalt::Loop(self.accumulator)
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/08")?;
    let program = parse_input(&input);
    println!("Part 1: {}", part_one(&program));
    println!("Part 2: {}", part_two(&program));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Program {
    Program::from_str(&input).unwrap()
}

fn part_one(program: &Program) -> i32 {
    match program.clone().run() {
        ProgramHalt::Loop(ans) => ans,
        _ => panic!("Expected Loop"),
    }
}

fn part_two(program: &Program) -> i32 {
    // Loop through all of the instructions.
    // Each time it's a Jump or a Noop, switch them and try running.
    // If the program exits normally, return the accumulated value.
    // Otherwise, continue
    for (i, inst) in program.instructions.iter().enumerate() {
        let modified = match inst {
            Instruction::Acc(_) => continue,
            Instruction::Jump(val) => Instruction::Noop(*val),
            Instruction::Noop(val) => Instruction::Jump(*val),
        };
        let mut clone = program.clone();
        clone.instructions[i] = modified;
        match clone.run() {
            ProgramHalt::Loop(_) => continue,
            ProgramHalt::NormalExit(ans) => return ans,
        }
    }
    unreachable!();
}

#[test]
fn test_examples() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let program = parse_input(&input);
    assert_eq!(part_one(&program), 5);
    assert_eq!(part_two(&program), 8);
}
