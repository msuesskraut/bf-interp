//! Tests for `parser::parse`.

use super::*;
use Instruction::*;

use parser::parse;

#[test]
fn skip_white_space() {
    assert_eq!(Program { instructions: vec![Input, Output] },
               parse("    ,    .    ".to_string()));
}

#[test]
fn parse_input_output() {
    assert_eq!(Program { instructions: vec![Output, Input] },
               parse(".,".to_string()));
}

#[test]
fn parse_inc_dec() {
    assert_eq!(Program { instructions: vec![Inc(1u8), Dec(1u8)] },
               parse("+-".to_string()));
}

#[test]
fn parse_moves() {
    assert_eq!(Program { instructions: vec![MoveRight(1usize), MoveLeft(1usize)] },
               parse("><".to_string()));
}

#[test]
fn parse_loop() {
    assert_eq!(Program { instructions: vec![LoopEntry(1usize), LoopExit(0usize)] },
               parse("[]".to_string()));
}

#[test]
fn parse_program_with_all_instructions() {
    assert_eq!(Program {
                   instructions: vec![Input, // 0
                                      Inc(1u8), // 1
                                      MoveRight(1usize), // 2
                                      Inc(1u8), // 3
                                      LoopEntry(6usize), // 4
                                      Dec(1u8), // 5
                                      LoopExit(4usize), // 6
                                      MoveLeft(1usize), // 7
                                      Output],
               }, // 8
               parse(",+>+[-]<.".to_string()));
}

#[test]
#[should_panic]
fn parse_unbalanced_move_left() {
    let _ = parse("[".to_string());
}

#[test]
#[should_panic]
fn parse_unbalanced_move_right() {
    let _ = parse("]".to_string());
}
