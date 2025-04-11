pub enum ErrT {
    SyntaxError {
        line: usize,
        column_start: usize,
        column_end: usize,
    },
    MissFnName {
        line: usize,
        column_start: usize,
        column_end: usize,
    },
    ExpectedArgStart {
        line: usize,
        column_start: usize,
        column_end: usize,
    },
    MissingReturnType {
        line: usize,
        column_start: usize,
        column_end: usize,
    },
    InvVarT {
        line: usize,
        column_start: usize,
        column_end: usize,
    },
    TupleNil{
        line: usize,
        column_start: usize,
        column_end: usize,
    },
}
