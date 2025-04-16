
#[derive(Debug)]
pub enum ErrT{
    ///when func name is not found
    FuncNameNotFound,
    ///Syntax err
    /// `&'statoc err` -> what I expected?
    SyntaxErr(&'static str),
    ///expected param name
    ExpectParamName,
    ///special type for @
    ExpectedAT,
    ///ret type mistmatch
    RetTMisMatch(&'static str),
    ///Ret not found
    RetTNF,
    ///Var type invalid
    InvVT,
}
#[derive(Debug)]
pub struct PErr{
    pub et: ErrT,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}
