#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MoveLeft(usize),
    MoveRight(usize),
    Inc(u8),
    Dec(u8),
    Input,
    Output,
    LoopEntry(usize),
    LoopExit(usize),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}
