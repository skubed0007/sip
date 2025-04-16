#[derive(Debug, Clone,PartialEq, Eq)]
pub enum IDType {
    DQ,
    SQ,
    Nil,
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub enum TT {
    FN,
    Ident(IDType),
    LSmallB,
    LCurlyB,
    LBigB,
    RsmallB,
    RCurlyB,
    RBigB,
    Eqs,
    At,
    Comma,
    I32,
    NUM,
    I64,
    U8,
    F32,
    F64,
    Ret,
    Underscore,
    CONST,
    GLOBAL,
    SemiColon,
    Nil,
}
#[allow(non_snake_case)]
#[derive(Debug, Clone,PartialEq, Eq)]
pub struct Token {
    pub TT: TT,
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub value: Option<String>, // Optional, used only for Ident
}
