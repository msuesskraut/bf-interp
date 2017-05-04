use super::*;

use buffer::*;

#[test]
fn output_zero() {
    let mut input = IBuffer::new("".to_string());
    let mut output = OBuffer::new();
    Program{instructions: vec![Output]}.interp(&mut input, &mut output);
    assert_eq!("\0".to_string(), output.to_string());
}