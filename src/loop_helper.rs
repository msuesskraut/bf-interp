use std::usize;

use ast::Instruction;
use Instruction::*;

#[derive(Debug)]
pub struct LoopHelper {
    bracket_stack: Vec<usize>,
    relocs: Vec<(usize, usize)>,
}

impl LoopHelper {
    pub fn new() -> LoopHelper {
        LoopHelper {
            bracket_stack: Vec::new(),
            relocs: Vec::new(),
        }
    }

    pub fn loop_entry(&mut self, idx: usize) -> Instruction {
        self.bracket_stack.push(idx);
        // value must be patched later
        LoopEntry(usize::MAX)
    }

    pub fn loop_exit(&mut self, idx: usize) -> Instruction {
        if let Some(loop_entry) = self.bracket_stack.pop() {
            self.relocs.push((loop_entry, idx));
            LoopExit(loop_entry)
        } else {
            panic!("Unbalanced {:?} at pc={:}", ']', idx);
        }
    }

    pub fn relocate(mut self, instructions: &mut Vec<Instruction>) {
        if let Some(unbalanced_idx) = self.bracket_stack.pop() {
            panic!("Unbalanced {:?} at pc={:}", '[', unbalanced_idx);
        }
        for (idx, value) in self.relocs {
            if idx >= instructions.len() || LoopEntry(usize::MAX) != instructions[idx] {
                panic!("Unexpected instruction {:?} at pc={:} for reloc",
                       instructions[idx],
                       idx);
            } else {
                instructions[idx] = LoopEntry(value);
            }
        }
    }
}
