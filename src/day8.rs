use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = &s[..=2];
        let value = s[4..].parse::<i32>().map_err(|_| ())?;

        match instruction {
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            "nop" => Ok(Instruction::Nop),
            _ => Err(()),
        }
    }
}

struct Vm {
    instructions: Vec<Instruction>,
    ptr: usize,
    state: i32,
}

impl Vm {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ptr: 0,
            state: 0,
        }
    }

    fn patch(&mut self, index: usize, instruction: Instruction) {
        if let Some(old) = self.instructions.get_mut(index) {
            *old = instruction;
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
        self.state = 0;
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Acc(value) => self.state += value,
            Instruction::Jmp(offset) => {
                self.ptr = (self.ptr as i32 + offset) as usize;
                return;
            }
            Instruction::Nop => {}
        }

        self.ptr += 1;
    }

    pub fn run_with_breaker(
        &mut self,
        breaker: Box<dyn Fn(Instruction) -> bool>,
    ) -> Result<i32, i32> {
        let mut executed = HashSet::with_capacity(self.instructions.len());

        while let Some(&instruction) = self.instructions.get(self.ptr) {
            if executed.contains(&self.ptr) && breaker(instruction) {
                return Err(self.state);
            }

            executed.insert(self.ptr);
            self.run_instruction(instruction);
        }

        Ok(self.state)
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn solve_part_1(instructions: &[Instruction]) -> i32 {
    let mut vm = Vm::new(instructions.to_vec());
    vm.run_with_breaker(Box::new(|_| true)).unwrap_err()
}

#[aoc(day8, part2)]
fn solve_part_2(instructions: &[Instruction]) -> i32 {
    let jump_indices: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| matches!(i, Instruction::Jmp(_)))
        .map(|(index, _)| index)
        .collect();

    for jump in jump_indices {
        let mut vm = Vm::new(instructions.to_vec());
        vm.patch(jump, Instruction::Nop);

        let result = vm.run_with_breaker(Box::new(|inst| matches!(inst, Instruction::Jmp(_))));

        match result {
            Ok(value) => return value,
            Err(_) => vm.reset(),
        }
    }

    panic!("no result found");
}
