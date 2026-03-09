#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Stack namipulation
    Push(i64),
    Dup,
    Swap,
    Drop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Heap Access
    Store,
    Load,

    // Flow Control
    Label(u64),
    Call(u64),
    Jump(u64),
    JumpZero(u64),
    JumpNeg(u64),
    Return,
    End,

    // I/O
    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
}

