//! The ast-module defines the data-structures for a brainfuck
//! program.

/// A single brainfuck instruction for normal brainfuck code:
/// `+`, `-`, `<`, `>`, `.`, `,`, `[` and `]`.
///
/// It also supports optimized instructions like
/// increment by 4 `Inc(4)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    /// Move data-pointer to left by the given offset.
    /// `<` is `MoveLeft(1)`.
    /// Moving left from data-pointer `0` panics.
    MoveLeft(usize),
    /// Move data-pointer to right by the given offset.
    /// `>` is `MoveRight(1)`.
    /// Moving right behind the maximum size of the band panics.
    /// This size is in general undefined, but one can assume that there are several 1000.
    MoveRight(usize),
    /// Increments memory cell at data-pointer by given value.
    /// `+` is `Inc(1)`.
    /// Incremeting above 255 wraps.
    Inc(u8),
    /// Decrements memory cell at data-pointer by given value.
    /// `-` is `Dec(1)`.
    /// Decremeting below 0 wraps.
    Dec(u8),
    /// Reads one byte from input-stream and
    /// writes it to the memory cell at the current data-pointer.
    Input,
    /// Writes the byte in the memory cell at the current data-pointer as
    /// `char` to output stream.
    Output,
    /// If memory cell at current data-pointer is `0`,
    /// then jump to instruction behind given index in array of instructions
    /// (see `Program`).
    /// This implements `[` with the index being the position of the corresponding `]`.
    /// The index avoids looking up the index to jump to at runtime.
    LoopEntry(usize),
    /// If memory cell at current data-pointer is _not_ `0`,
    /// then jump to instruction behind given index in array of instructions (see `Program`).
    /// This implements `]` with the index being the position of the corresponding `[`.
    /// The index avoids looking up the index to jump to at runtime.
    LoopExit(usize),
}

/// A program is just an array (`vec`) of `Instruction`s.
/// For loop-instructions, the offset is just an index into this array.
#[derive(Debug, PartialEq)]
pub struct Program {
    /// The array of instructions.
    /// Each instruction has a defined index index in this array.
    pub instructions: Vec<Instruction>,
}
