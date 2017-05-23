//! Parser `parser::parse` for brainfuck.

use ast::Program;
use Instruction::*;
use loop_helper::LoopHelper;

/// Parses (and lexes) the input `text` and returns an ast.
/// Whitespace in `text` ist skipped.
/// Whitespace is anything else, then the 8 legal brainfuck
/// symbols: `+`, `-`, `<`, `>`, `[`, `]`, `.` and `,`.
///
/// # Panics
///
/// Panics on parsing errors: unbalanced `[` and `]`.
pub fn parse(text: String) -> Program {
    let mut loop_helper = LoopHelper::new();
    // "lex" token stream - aka skip whitespace
    let token = text.chars()
        .filter(move |c| {
                    *c == '>' || *c == '<' || *c == '+' || *c == '-' || *c == '.' ||
                    *c == ',' || *c == '[' || *c == ']'
                });
    let mut instructions = token
        .enumerate()
        .map(|(idx, c)| match c {
                 '<' => MoveLeft(1),
                 '>' => MoveRight(1),
                 '+' => Inc(1),
                 '-' => Dec(1),
                 '.' => Output,
                 ',' => Input,
                 '[' => loop_helper.loop_entry(idx),
                 ']' => loop_helper.loop_exit(idx),
                 c => panic!("Unknown instruction {:?} at pc={:}", c, idx),
             })
        .collect::<Vec<_>>();
    loop_helper.relocate(&mut instructions);
    Program { instructions: instructions }
}
