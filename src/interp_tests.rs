use super::*;

use buffer::*;

#[test]
fn output_zero() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Output] }.interp(&mut input, &mut output);
    assert_eq!("\0".to_string(), output.to_string());
}

#[test]
fn inc_by_one() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(1), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x01".to_string(), output.to_string());
}

#[test]
fn inc_by_two() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(2), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x02".to_string(), output.to_string());
}

#[test]
fn inc_by_one_and_by_one() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(1), Inc(1), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x02".to_string(), output.to_string());
}

#[test]
fn inc_wrapping() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(255), Inc(1), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x00".to_string(), output.to_string());
}

#[test]
fn dec_by_one() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(2), Dec(1), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x01".to_string(), output.to_string());
}

#[test]
fn dec_by_two() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(2), Dec(2), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x00".to_string(), output.to_string());
}

#[test]
fn dec_by_one_and_by_one() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Inc(3), Dec(1), Dec(1), Output] }.interp(&mut input, &mut output);
    assert_eq!("\x01".to_string(), output.to_string());
}

#[test]
fn dec_wrapping() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program { instructions: vec![Dec(1), Output] }.interp(&mut input, &mut output);
    assert_eq!(vec![255u8], output.to_vec());
}
