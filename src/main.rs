use std::fs::File;
use std::io::{Read, Stdin, Result};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    pub fn new(text: String) -> Program {
        // open brackets while parsing
        let mut bracket_stack = Vec::new();
        // idx to patch with index of LoopEntry to patch in
        let mut relocs = Vec::new();
        // "lex" token stream - aka skip whitespace
        let token = text.chars().filter(move |c| {
            *c == '>' || *c == '<' || *c == '+' || *c == '-' ||
            *c == '.' || *c == ',' || *c == '[' || *c == ']'
        });
        let mut instructions = token.enumerate().map(|(idx, c)| {
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
                },
                ']' => {
                    if let Some(loop_entry) = bracket_stack.pop() {
                        relocs.push((loop_entry, idx));
                        LoopExit(loop_entry)
                    }
                    else {
                        panic!("Unbalanced {:?} at pc={:}", c, idx);
                    }
                },
                c => panic!("Unknown instruction {:?} at pc={:}", c, idx),
            }
        }).collect::<Vec<_>>();
        if let Some(unbalanced_idx) = bracket_stack.pop() {
            panic!("Unbalanced {:?} at pc={:}", '[', unbalanced_idx);
        }
        for (idx, value) in relocs {
            if idx >= instructions.len() || LoopEntry(std::usize::MAX) != instructions[idx] {
                panic!("Unexpected instruction {:?} at pc={:} for reloc", instructions[idx], idx);
            }
            else {
                instructions[idx] = LoopEntry(value);
            }
        }
        Program {
            instructions : instructions
        }
    }

    pub fn interp(&self) {
        let mut memory = vec![0u8; 30000];

        let mut pc:usize = 0;
        let mut dataptr:usize = 0;
        let mut stdin = std::io::stdin();

        fn get_char(stdin: &mut Stdin) -> u8 {
            let mut buf = [0u8; 1];
            match stdin.read(&mut buf) {
                Err(_) => panic!("Cannot read from stdin"),
                Ok(_) => buf[0]
            }
        }

        while pc < self.instructions.len() {
            match self.instructions[pc] {
                MoveLeft(offset) => dataptr -= offset,
                MoveRight(offset) => dataptr += offset,
                Inc(increment) => memory[dataptr] = memory[dataptr].wrapping_add(increment),
                Dec(decrement) => memory[dataptr] = memory[dataptr].wrapping_sub(decrement),
                Output => print!("{:}", memory[dataptr] as char),
                Input => memory[dataptr] = get_char(&mut stdin),
                LoopEntry(target) => if 0 == memory[dataptr] {
                    pc = target;
                },
                LoopExit(target) => if 0 != memory[dataptr] {
                    pc = target;
                },
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
            //println!("{:?}", p);
            p.interp();
        },
        Err(err) => panic!("Cannot read file because {:?}", err),
    }
}
