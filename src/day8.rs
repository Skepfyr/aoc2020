use std::str::FromStr;

use aoc_runner_derive::*;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    state: State,
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub program_counter: isize,
    pub accumulator: i32,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            state: State {
                program_counter: 0,
                accumulator: 0,
            },
        }
    }

    pub fn step(&mut self) -> State {
        let instruction = self.instructions[self.state.program_counter as usize];
        self.state.program_counter += match instruction.opcode {
            Opcode::Accumulator => {
                self.state.accumulator += instruction.argument;
                1
            }
            Opcode::Jump => instruction.argument as isize,
            Opcode::NoOperation => 1,
        };
        self.state
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub argument: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let opcode = split
            .next()
            .ok_or_else(|| "Empty string is not an instruction".to_owned())?;
        let argument = split
            .next()
            .ok_or_else(|| format!("Instruction missing argument: {}", s))?;
        if split.next().is_some() {
            return Err(format!(
                "Instruction contains unexpected second argument: {}",
                s
            ));
        }
        Ok(Instruction {
            opcode: opcode.parse()?,
            argument: argument.parse::<i32>().map_err(|e| e.to_string())?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Accumulator,
    Jump,
    NoOperation,
}

impl FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "acc" => Opcode::Accumulator,
            "jmp" => Opcode::Jump,
            "nop" => Opcode::NoOperation,
            opcode => return Err(format!("Unrecognised opcode {}", opcode)),
        })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

#[aoc(day8, part1)]
pub fn day8_part1(input: &[Instruction]) -> i32 {
    let mut program = Program::new(input.to_vec());
    let mut visited = vec![false; input.len()];
    loop {
        let state = program.step();
        if visited[state.program_counter as usize] {
            break state.accumulator;
        }
        visited[state.program_counter as usize] = true;
    }
}

#[aoc(day8, part2)]
pub fn day8_part2(input: &[Instruction]) -> i32 {
    'outer: for i in 0..input.len() {
        let mut instructions = input.to_vec();
        match instructions[i].opcode {
            Opcode::Accumulator => continue,
            Opcode::Jump => instructions[i].opcode = Opcode::NoOperation,
            Opcode::NoOperation => instructions[i].opcode = Opcode::Jump,
        }
        let mut program = Program::new(instructions);
        let mut visited = vec![false; input.len()];
        loop {
            let state = program.step();
            if state.program_counter >= program.instructions.len() as isize {
                return state.accumulator;
            }
            if visited[state.program_counter as usize] {
                continue 'outer;
            }
            visited[state.program_counter as usize] = true;
        }
    }
    panic!("No working change found!");
}
