use std::fs::File;
use std::io::{Read, Stdin, Result};

#[derive(Debug)]
struct Program {
    instructions: Vec<char>
}

impl Program {
    pub fn new(text: String) -> Program
    {
        Program {
            instructions: text.chars().filter(move |c| {
                *c == '>' || *c == '<' || *c == '+' || *c == '-' ||
                *c == '.' || *c == ',' || *c == '[' || *c == ']'
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
                '>' => dataptr += 1,
                '<' => dataptr -= 1,
                '+' => memory[dataptr] = memory[dataptr].wrapping_add(1),
                '-' => memory[dataptr] = memory[dataptr].wrapping_sub(1),
                '.' => print!("{:}", memory[dataptr] as char),
                ',' => memory[dataptr] = get_char(&mut stdin),
                '[' => if 0 == memory[dataptr] {
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    pc += 1;
                    while (bracket_nesting > 0) && (pc < self.instructions.len()) {
                        match self.instructions[pc] {
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
                        panic!("unmachted '[' at pc={:}", saved_pc);
                    }
                },
                ']' => if 0 != memory[dataptr] {
                    let mut bracket_nesting = 1;
                    let saved_pc = pc;

                    while (bracket_nesting > 0) && (pc > 0) {
                        pc -= 1;
                        match self.instructions[pc] {
                            '[' => bracket_nesting -= 1,
                            ']' => bracket_nesting += 1,
                            _ => (),
                        };
                    }
                    if 0 != bracket_nesting {
                        panic!("unmachted ']' at pc={:}", saved_pc);
                    }
                },
                c => panic!("Unknown instruction {:?} at pc={:}", c, pc),
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
