use crate::*;
use std::collections::HashSet;

type InstructionSet = (String, i32);

struct State {
    accumulator: i32,
    pos: usize,
    ran: HashSet<usize>,
    tried_swap: HashSet<usize>,
    instructions: Vec<String>,
    finished: bool,
    ran_twice: bool,
    swapped: bool
}

impl State {

    fn new(instructions: Vec<String>) -> Self {
        State {
            accumulator: 0,
            pos: 0,
            ran: HashSet::new(),
            tried_swap: HashSet::new(),
            instructions,
            finished: false,
            ran_twice: false,
            swapped: false
        }
    }

    // Resets while leaving the tried_swap values
    fn reset(&mut self) {
        self.accumulator = 0;
        self.pos = 0;
        self.ran = HashSet::new();
        self.finished = false;
        self.ran_twice = false;
        self.swapped = false;
    }

    fn parse_instruction(inst: String) -> InstructionSet {
        (
            inst[0..3].to_string(),
            inst[4..].parse::<i32>().unwrap()
        )
    }

    fn move_pos(&mut self, by: i32) {
        self.pos = (self.pos as i32 + by) as usize;
        if self.pos >= self.instructions.len() {
            self.finished = true;
        }
    }

    fn acc(&mut self, num: i32) {
        self.accumulator += num;
        self.move_pos(1);
    }

    fn jmp(&mut self, num: i32) {
        self.move_pos(num);
    }

    fn nop(&mut self) {
        self.move_pos(1);
    }

    fn next(&mut self) {
        // If a command is about to run twice, don't run it.
        if self.ran.contains(&self.pos) {
            self.ran_twice = true;
            return;
        }
        let instruction = State::parse_instruction(self.instructions[self.pos].to_string());
        self.ran.insert(self.pos);
        match instruction.0.as_str() {
            "acc" => self.acc(instruction.1),
            "jmp" => self.jmp(instruction.1),
            "nop" => self.nop(),
            _ => panic!("Command parsing error.")
        }
    }

    fn next_with_swap(&mut self) {
        // If a command is about to run twice, our swap didn't work. Let's reset and continue.
        if self.ran.contains(&self.pos) {
            self.reset();
        }
        let instruction = State::parse_instruction(self.instructions[self.pos].to_string());
        self.ran.insert(self.pos);
        match instruction.0.as_str() {
            "acc" => self.acc(instruction.1),
            "jmp" if self.swapped || self.tried_swap.contains(&self.pos) => self.jmp(instruction.1),
            "nop" if self.swapped || self.tried_swap.contains(&self.pos) => self.nop(),
            "jmp" => {
                self.swapped = true;
                self.tried_swap.insert(self.pos);
                self.nop();
            },
            "nop" => {
                self.swapped = true;
                self.tried_swap.insert(self.pos);
                self.jmp(instruction.1);
            }
            _ => panic!("Command parsing error.")
        }
    }
}

fn part1(input: Vec<String>) {
    let mut state = State::new(input);
    while !state.ran_twice {
        state.next();
    }
    part1!(state.accumulator);
}

fn part2(input: Vec<String>) {
    let mut state = State::new(input);

    while !state.finished {
        state.next_with_swap();
    }
    part2!(state.accumulator);
}

// https://adventofcode.com/2020/day/8
pub fn solve(input: String) {
    let input: Vec<String> = input.split("\n").map(|e| e.to_string()).collect();
    part1(input.clone());
    part2(input.clone());
}
