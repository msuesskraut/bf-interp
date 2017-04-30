use std::fs::File;
use std::io::{Read, Stdin, Result};

#[derive(Debug)]
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
    pub fn new(text: String) -> Program
    {
        // "lex" token stream - aka skip whitespace
        let token = text.chars().filter(move |c| {
            *c == '>' || *c == '<' || *c == '+' || *c == '-' ||
            *c == '.' || *c == ',' || *c == '[' || *c == ']'
        }).collect::<Vec<_>>();
        Program {
            instructions : token.iter().enumerate().map(|(idx, &c)| {
                match c {
                    '>' => MoveLeft(1),
                    '<' => MoveRight(1),
                    '+' => Inc(1),
                    '-' => Dec(1),
                    '.' => Output,
                    ',' => Input,
                    '[' => {
                        let mut bracket_nesting = 1;
                        let mut pc = idx + 1;
    
                        while (bracket_nesting > 0) && (pc < token.len()) {
                            match token[pc] {
                                '[' => bracket_nesting += 1,
                                ']' => bracket_nesting -= 1,
                                _ => (),
                            };
                            pc += 1;
                        }
                        if 0 == bracket_nesting {
                            pc -= 1;
                        }
                        else {
                            panic!("unmachted '[' at pc={:}", idx);
                        }
                        LoopEntry(pc)
                    },
                    ']' => {
                        let mut bracket_nesting = 1;
                        let mut pc = idx;

                        while (bracket_nesting > 0) && (pc > 0) {
                            pc -= 1;
                            match token[pc] {
                                '[' => bracket_nesting -= 1,
                                ']' => bracket_nesting += 1,
                                _ => (),
                            };
                        }
                        if 0 != bracket_nesting {
                            panic!("unmachted ']' at pc={:}", idx);
                        }
                        LoopExit(pc)
                    },
                    c => panic!("Unknown instruction {:?} at pc={:}", c, idx),
                }
            }).collect::<Vec<_>>()
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
                MoveLeft(1usize) => dataptr += 1,
                MoveRight(1usize) => dataptr -= 1,
                Inc(1u8) => memory[dataptr] = memory[dataptr].wrapping_add(1),
                Dec(1u8) => memory[dataptr] = memory[dataptr].wrapping_sub(1),
                Output => print!("{:}", memory[dataptr] as char),
                Input => memory[dataptr] = get_char(&mut stdin),
                LoopEntry(target) => if 0 == memory[dataptr] {
                    pc = target;
                },
                LoopExit(target) => if 0 != memory[dataptr] {
                    pc = target;
                },
                ref c => panic!("Unknown instruction {:?} at pc={:}", c, pc),
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
        Ok(p) => {
            //println!("{:?}", p);
            p.interp();
        },
        Err(err) => panic!("Cannot read file because {:?}", err),
    }
}
