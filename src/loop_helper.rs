//! Implements a loop-up algorithm to speedup loop execution at runtime.

use std::usize;

use ast::Instruction;
use Instruction::*;

/// LoopHelper does the book-keeping while assembling a new list of instructions
/// (brainfuck program).
/// For loop-exits it immidately provides the index of the corresponding loop-entry.
/// Loop-entries are first placeholders, which index-value has to be "patched" after
/// assembling the whole program.
///
/// This implementation assumes that Loop-Entries and Loop-Exits are paired and balanced.
/// They may be nested.
#[derive(Debug)]
pub struct LoopHelper {
    bracket_stack: Vec<usize>,
    relocs: Vec<(usize, usize)>,
}

impl LoopHelper {
    /// Creates a new loop helper.
    /// Call before starting to assemble the instruction list.
    pub fn new() -> LoopHelper {
        LoopHelper {
            bracket_stack: Vec::new(),
            relocs: Vec::new(),
        }
    }

    /// Creates a new loop_entry to be inserted into the
    /// instruction list at index `idx`.
    /// This method returns just a placeholder that has to patched after
    /// assembling the whole instruction list with `relocate`.
    pub fn loop_entry(&mut self, idx: usize) -> Instruction {
        self.bracket_stack.push(idx);
        // value must be patched later
        LoopEntry(usize::MAX)
    }

    /// Returns the loop-exit instruction to be inserted at index `idx`.
    ///
    /// # Panics
    ///
    /// Panics if loop-exit is not matched by a previously visitied loop-entry on
    /// the same nesting level.
    pub fn loop_exit(&mut self, idx: usize) -> Instruction {
        if let Some(loop_entry) = self.bracket_stack.pop() {
            self.relocs.push((loop_entry, idx));
            LoopExit(loop_entry)
        } else {
            panic!("Unbalanced {:?} at pc={:}", ']', idx);
        }
    }

    /// Patches loop entry instructions *after* assembly the whole array of instructions.
    ///
    /// # Panics
    ///
    /// Panics if not all loop-entries were matched by corresponding loop-exits.
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
