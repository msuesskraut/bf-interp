use super::*;

#[test]
fn skip_white_space() {
    assert_eq!(
        Program{instructions: vec![Input, Output]},
        Program::new("    ,    .    ".to_string()));
}

#[test]
fn parse_input_output() {
    assert_eq!(
        Program{instructions: vec![Output, Input]},
        Program::new(".,".to_string()));
}

#[test]
fn parse_inc_dec() {
    assert_eq!(
        Program{instructions: vec![Inc(1u8), Dec(1u8)]},
        Program::new("+-".to_string()));
}

#[test]
fn parse_moves() {
    assert_eq!(
        Program{instructions: vec![MoveRight(1usize), MoveLeft(1usize)]},
        Program::new("><".to_string()));
}

#[test]
fn parse_loop() {
    assert_eq!(
        Program{instructions: vec![LoopEntry(1usize), LoopExit(0usize)]},
        Program::new("[]".to_string()));
}

#[test]
fn parse_program_with_all_instructions() {
    assert_eq!(
        Program{instructions: vec![
            Input,             // 0
            Inc(1u8),          // 1
            MoveRight(1usize), // 2
            Inc(1u8),          // 3
            LoopEntry(6usize), // 4
            Dec(1u8),          // 5
            LoopExit(4usize),  // 6
            MoveLeft(1usize),  // 7
            Output]},          // 8
        Program::new(",+>+[-]<.".to_string()));
}