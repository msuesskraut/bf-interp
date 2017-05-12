use super::*;

#[test]
fn merge_inc() {
    assert_eq!(Program { instructions: vec![Inc(12)] },
               Program { instructions: vec![Inc(3), Inc(4), Inc(5)] }.optimize());
}

#[test]
fn merge_inc_segment() {
    assert_eq!(Program { instructions: vec![Inc(2), Output, Inc(3)] },
               Program { instructions: vec![Inc(1), Inc(1), Output, Inc(2), Inc(1)] }.optimize());
}

#[test]
fn merge_dec() {
    assert_eq!(Program { instructions: vec![Dec(12)] },
               Program { instructions: vec![Dec(3), Dec(4), Dec(5)] }.optimize());
}

#[test]
fn merge_dec_segment() {
    assert_eq!(Program { instructions: vec![Dec(2), Output, Dec(3)] },
               Program { instructions: vec![Dec(1), Dec(1), Output, Dec(2), Dec(1)] }.optimize());
}

#[test]
fn merge_move_left() {
    assert_eq!(Program { instructions: vec![MoveLeft(12)] },
               Program { instructions: vec![MoveLeft(3), MoveLeft(4), MoveLeft(5)] }.optimize());
}

#[test]
fn merge_move_left_segment() {
    assert_eq!(Program { instructions: vec![MoveLeft(2), Output, MoveLeft(3)] },
               Program {
                       instructions: vec![MoveLeft(1),
                                          MoveLeft(1),
                                          Output,
                                          MoveLeft(2),
                                          MoveLeft(1)],
                   }
                   .optimize());
}

#[test]
fn merge_move_right() {
    assert_eq!(Program { instructions: vec![MoveRight(12)] },
               Program { instructions: vec![MoveRight(3), MoveRight(4), MoveRight(5)] }.optimize());
}

#[test]
fn merge_move_right_segment() {
    assert_eq!(Program { instructions: vec![MoveRight(2), Output, MoveRight(3)] },
               Program {
                       instructions: vec![MoveRight(1),
                                          MoveRight(1),
                                          Output,
                                          MoveRight(2),
                                          MoveRight(1)],
                   }
                   .optimize());
}

#[test]
fn keep_loop() {
    assert_eq!(Program { instructions: vec![Inc(9), LoopEntry(3), Dec(3), LoopExit(1)] },
               Program {
                       instructions: vec![Inc(4),
                                          Inc(5),
                                          LoopEntry(6),
                                          Dec(1),
                                          Dec(1),
                                          Dec(1),
                                          LoopExit(2)],
                   }
                   .optimize())
}
