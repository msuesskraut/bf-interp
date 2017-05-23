//! Interpreter tests.

use super::*;

use ast::{Program, Instruction};
use self::Instruction::*;
use buffer::*;

fn execute_and_check(program: &[Instruction], input: &str, exp_output: &[u8]) {
    let mut input = IBuffer::from_str(input);
    let mut output = Vec::new();
    let p = Program { instructions: program.to_vec() };
    interp::interp(&p, &mut input, &mut output);
    assert_eq!(exp_output, output.as_slice());
}

#[test]
fn output_zero() {
    execute_and_check(&[Output], "", &[0]);
}

#[test]
fn inc_by_one() {
    execute_and_check(&[Inc(1), Output], "", &[1]);
}

#[test]
fn inc_by_two() {
    execute_and_check(&[Inc(2), Output], "", &[2]);
}

#[test]
fn inc_by_one_and_by_one() {
    execute_and_check(&[Inc(1), Inc(1), Output], "", &[2]);
}

#[test]
fn inc_wrapping() {
    execute_and_check(&[Inc(255), Inc(1), Output], "", &[0]);
}

#[test]
fn dec_by_one() {
    execute_and_check(&[Inc(2), Dec(1), Output], "", &[1]);
}

#[test]
fn dec_by_two() {
    execute_and_check(&[Inc(2), Dec(2), Output], "", &[0]);
}

#[test]
fn dec_by_one_and_by_one() {
    execute_and_check(&[Inc(3), Dec(1), Dec(1), Output], "", &[1]);
}

#[test]
fn dec_wrapping() {
    execute_and_check(&[Dec(1), Inc(2), Output], "", &[1]);
}

#[test]
fn move_right() {
    execute_and_check(&[Inc(2), MoveRight(1), Inc(1), Output], "", &[1]);
}

#[test]
fn move_left() {
    execute_and_check(&[Inc(2), MoveRight(1), Inc(1), Output, MoveLeft(1), Output],
                      "",
                      &[1, 2]);
}

#[test]
fn move_right_2() {
    execute_and_check(&[Inc(3),
                        MoveRight(1),
                        MoveRight(1),
                        Inc(1),
                        Output,
                        MoveLeft(1),
                        Output,
                        MoveLeft(1)],
                      "",
                      &[1, 0]);
}

#[test]
fn move_right_2_at_once() {
    execute_and_check(&[Inc(3),
                        MoveRight(2),
                        Inc(1),
                        Output,
                        MoveLeft(1),
                        Output,
                        MoveLeft(1)],
                      "",
                      &[1, 0]);
}

#[test]
fn move_left_2() {
    execute_and_check(&[Inc(2),
                        MoveRight(2),
                        Inc(1),
                        Output,
                        MoveLeft(1),
                        MoveLeft(1),
                        Output],
                      "",
                      &[1, 2]);
}

#[test]
fn move_left_2_at_once() {
    execute_and_check(&[Inc(2),
                        MoveRight(2),
                        Inc(1),
                        Output,
                        MoveLeft(1),
                        MoveLeft(1),
                        Output],
                      "",
                      &[1, 2]);
}

#[test]
fn input() {
    execute_and_check(&[Input, Output, Input, Output], "a1", &[b'a', b'1']);
}

#[test]
fn skip_loop() {
    execute_and_check(&[MoveRight(1),
                        Inc(1),
                        MoveLeft(1),
                        LoopEntry(5),
                        Inc(1),
                        LoopExit(3),
                        Output],
                      "",
                      &[0]);
}

#[test]
fn loop_set_zero() {
    execute_and_check(&[Inc(100), LoopEntry(3), Dec(1), LoopExit(1), Output],
                      "",
                      &[0]);
}

#[test]
fn loop_move_value() {
    execute_and_check(&[Inc(100),
                        LoopEntry(6),
                        Dec(1),
                        MoveRight(3),
                        Inc(1),
                        MoveLeft(3),
                        LoopExit(1),
                        Output,
                        MoveRight(3),
                        Output],
                      "",
                      &[0, 100]);
}

#[test]
fn loop_find_zero_left() {
    execute_and_check(&[MoveRight(1),
                        Inc(1),
                        MoveRight(1),
                        Inc(1),
                        MoveRight(1),
                        Inc(1),
                        LoopEntry(8),
                        MoveLeft(1),
                        LoopExit(6),
                        Output],
                      "",
                      &[0]);
}

#[test]
fn loop_find_zero_right() {
    execute_and_check(&[MoveRight(1),
                        Inc(1),
                        MoveRight(1),
                        Inc(1),
                        MoveRight(1),
                        Inc(1),
                        LoopEntry(8),
                        MoveLeft(1),
                        LoopExit(6),
                        Output,
                        Inc(9),
                        Output,
                        LoopEntry(14),
                        MoveRight(1),
                        LoopExit(12),
                        Output],
                      "",
                      &[0, 9, 0]);
}
