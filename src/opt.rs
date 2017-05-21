use ast::{Instruction, Program};
use ast::Instruction::*;

use loop_helper::LoopHelper;

pub fn optimize(program: &Program) -> Program {
    let mut loop_helper = LoopHelper::new();
    // optimized program
    let mut instructions: Vec<Instruction> = Vec::new();

    for instr in program.instructions.iter() {
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
                let idx = instructions.len();
                instructions.push(loop_helper.loop_entry(idx));
            }
            LoopExit(_) => {
                let idx = instructions.len();
                instructions.push(loop_helper.loop_exit(idx));
            }
            instr => instructions.push(instr),
        }
    }
    loop_helper.relocate(&mut instructions);
    Program { instructions: instructions }
}
