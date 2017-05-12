use std::fs::File;
use std::io::{Read, Write, Result};

mod buffer;
#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod interp_tests;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    MoveLeft(usize),
    MoveRight(usize),
    Inc(u8),
    Dec(u8),
    Input,
    Output,
    LoopEntry(usize),
    LoopExit(usize),
}

use Instruction::*;

#[derive(Debug, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(text: String) -> Program {
        // open brackets while parsing
        let mut bracket_stack = Vec::new();
        // idx to patch with index of LoopEntry to patch in
        let mut relocs = Vec::new();
        // "lex" token stream - aka skip whitespace
        let token = text.chars()
            .filter(move |c| {
                        *c == '>' || *c == '<' || *c == '+' || *c == '-' || *c == '.' ||
                        *c == ',' || *c == '[' || *c == ']'
                    });
        let mut instructions = token
            .enumerate()
            .map(|(idx, c)| {
                match c {
                    '<' => MoveLeft(1),
                    '>' => MoveRight(1),
                    '+' => Inc(1),
                    '-' => Dec(1),
                    '.' => Output,
                    ',' => Input,
                    '[' => {
                        bracket_stack.push(idx);
                        // value must be patched later
                        LoopEntry(std::usize::MAX)
                    }
                    ']' => {
                        if let Some(loop_entry) = bracket_stack.pop() {
                            relocs.push((loop_entry, idx));
                            LoopExit(loop_entry)
                        } else {
                            panic!("Unbalanced {:?} at pc={:}", c, idx);
                        }
                    }
                    c => panic!("Unknown instruction {:?} at pc={:}", c, idx),
                }
            })
            .collect::<Vec<_>>();
        if let Some(unbalanced_idx) = bracket_stack.pop() {
            panic!("Unbalanced {:?} at pc={:}", '[', unbalanced_idx);
        }
        for (idx, value) in relocs {
            if idx >= instructions.len() || LoopEntry(std::usize::MAX) != instructions[idx] {
                panic!("Unexpected instruction {:?} at pc={:} for reloc",
                       instructions[idx],
                       idx);
            } else {
                instructions[idx] = LoopEntry(value);
            }
        }
        Program { instructions: instructions }
    }

    pub fn optimize(&self) -> Program {
        // optimized program
        let mut instructions: Vec<Instruction> = Vec::new();
        // open brackets while parsing
        let mut bracket_stack = Vec::new();
        // idx to patch with index of LoopEntry to patch in
        let mut relocs = Vec::new();

        for (idx, instr) in self.instructions.iter().enumerate() {
            match *instr {
                MoveLeft(offset) => {
                    let last_instr = instructions.last().cloned();
                    if let Some(MoveLeft(old_offset)) = last_instr {
                        let last_idx = instructions.len() - 1;
                        instructions[last_idx] = MoveLeft(old_offset + offset);
                    } else {
                        instructions.push(*instr);
                    }
                }
                MoveRight(offset) => {
                    let last_instr = instructions.last().cloned();
                    if let Some(MoveRight(old_offset)) = last_instr {
                        let last_idx = instructions.len() - 1;
                        instructions[last_idx] = MoveRight(old_offset + offset);
                    } else {
                        instructions.push(*instr);
                    }
                }
                Inc(val) => {
                    let last_instr = instructions.last().cloned();
                    if let Some(Inc(old_val)) = last_instr {
                        let last_idx = instructions.len() - 1;
                        instructions[last_idx] = Inc(old_val.wrapping_add(val));
                    } else {
                        instructions.push(*instr);
                    }
                }
                Dec(val) => {
                    let last_instr = instructions.last().cloned();
                    if let Some(Dec(old_val)) = last_instr {
                        let last_idx = instructions.len() - 1;
                        instructions[last_idx] = Dec(old_val.wrapping_add(val));
                    } else {
                        instructions.push(*instr);
                    }
                }
                LoopEntry(_) => {
                    bracket_stack.push(instructions.len());
                    instructions.push(LoopEntry(std::usize::MAX));
                }
                LoopExit(_) => {
                    if let Some(loop_entry) = bracket_stack.pop() {
                        relocs.push((loop_entry, instructions.len()));
                        instructions.push(LoopExit(loop_entry));
                    } else {
                        panic!("Unbalanced {:?} at pc={:}", instr, idx);
                    }
                }
                instr => instructions.push(instr),
            }
        }
        if let Some(unbalanced_idx) = bracket_stack.pop() {
            panic!("Unbalanced {:?} at pc={:}", '[', unbalanced_idx);
        }
        for (idx, value) in relocs {
            if idx >= instructions.len() || LoopEntry(std::usize::MAX) != instructions[idx] {
                panic!("Unexpected instruction {:?} at pc={:} for reloc",
                       instructions[idx],
                       idx);
            } else {
                instructions[idx] = LoopEntry(value);
            }
        }
        Program { instructions: instructions }
    }

    pub fn interp(&self, input: &mut Read, output: &mut Write) {
        let mut memory = vec![0u8; 30000];

        let mut pc: usize = 0;
        let mut dataptr: usize = 0;

        fn get_char(input: &mut Read) -> u8 {
            let mut buf = [0u8; 1];
            match input.read(&mut buf) {
                Err(_) => panic!("Cannot read from stdin"),
                Ok(_) => buf[0],
            }
        }

        while pc < self.instructions.len() {
            match self.instructions[pc] {
                MoveLeft(offset) => dataptr -= offset,
                MoveRight(offset) => dataptr += offset,
                Inc(increment) => memory[dataptr] = memory[dataptr].wrapping_add(increment),
                Dec(decrement) => memory[dataptr] = memory[dataptr].wrapping_sub(decrement),
                Output => write!(output, "{:}", memory[dataptr] as char).expect("Output error"),
                Input => memory[dataptr] = get_char(input),
                LoopEntry(target) => {
                    if 0 == memory[dataptr] {
                        pc = target;
                    }
                }
                LoopExit(target) => {
                    if 0 != memory[dataptr] {
                        pc = target;
                    }
                }
                // ref c => panic!("Unknown instruction {:?} at pc={:}", c, pc),
            }
            pc += 1;
        }
    }
}

fn load_program(fname: String) -> Result<Program> {
    let mut file = File::open(fname)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    let text = String::from_utf8(contents).unwrap();

    Ok(Program::new(text))
}

fn main() {
    match load_program("examples/mandelbrot.bf".to_string()) {
        Ok(ref mut p) => {
            let p = p.optimize();
            //println!("{:?}", p);
            p.interp(&mut std::io::stdin(), &mut std::io::stdout());
        }
        Err(err) => panic!("Cannot read file because {:?}", err),
    }
}
