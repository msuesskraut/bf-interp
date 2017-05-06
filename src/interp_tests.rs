use super::*;

use buffer::*;

fn execute_and_check(program: &[Instruction], input: &str, exp_output: &[u8]) {
    let mut input = IBuffer::from_str(input);
    let mut output = Vec::new();
    Program { instructions: program.to_vec() }.interp(&mut input, &mut output);
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
