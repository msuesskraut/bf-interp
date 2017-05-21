//! Interpreter of brainfuck ast.

use std::io::{Read, Write};

use ast::Program;
use ast::Instruction::*;

/// Interprets the brainfuck `program`.
/// Reads (instruction `Input` aka `,`) from input.
/// Writes (instruction `Output` aka `.`) to output.
/// The size of the band is in general undefined.
/// Currentl,y its 30000 bytes.
pub fn interp(program: &Program, input: &mut Read, output: &mut Write) {
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

    while pc < program.instructions.len() {
        match program.instructions[pc] {
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
