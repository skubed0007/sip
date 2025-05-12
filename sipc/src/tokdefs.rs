#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IDT {
    NQ,
    DQ,
    SQ,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPT {
    ADD,
    MUL,
    DIV,
    SUB,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NUMT {
    F(f32),
    I32(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TT {
    EXTERN,
    IDENT(IDT, String),
    EQS,
    /// `...` -> DOTDOTDOT -> DDD
    DDD,
    NUM(NUMT),
    /// sip link
    LN,
    /// clink
    CLN,
    OP(OPT),
    AT,
    F32,
    CALL,
    I32,
    U8,
    U8Ls(Box<[u8]>),
    LBigB,
    RBigB,
    Comma,
    LSmallB,
    RSmallB,
    ErrT(String),
    SemiC,
}

#[derive(Debug, Clone)]
pub struct Tok {
    pub tt: TT,
    pub line: i32,
    pub start: i32,
    pub end: i32,
}
