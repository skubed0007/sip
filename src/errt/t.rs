#[derive(Debug, Clone)]
/// Parser error kinds.
pub enum ErrT {
    IOErr(String),
    TypeMismatch,
    UnexpectedEof,
    InvalidArgToken,
    /// Expected an identifier/name but found something else.
    ExpectName,

    /// Found an unexpected token.
    UnexpectTok,

    /// Missing a semicolon (`;`) at the end of a statement.
    MissingSemicolon,

    /// Unterminated string or character literal.
    UnterminatedLiteral,

    /// Invalid number format (e.g. bad digits, overflow).
    InvalidNumber,

    /// Unknown keyword or directive.
    UnknownKeyword,

    /// Mismatched or missing parentheses `(` or `)`.
    UnmatchedParen,

    /// Mismatched or missing braces `{` or `}`.
    UnmatchedBrace,

    /// Mismatched or missing brackets `[` or `]`.
    UnmatchedBracket,

    /// Expected an operator but found something else.
    ExpectOperator,

    /// Expected an expression but none was found.
    ExpectExpression,

    /// Generic catch‑all for other parse errors.
    Generic(String),
}

#[derive(Debug, Clone)]
pub struct SErr {
    pub t: ErrT,
    pub line: i32,
    pub start: i32,
    pub end: i32,
    pub file: String,
}

impl SErr {
    pub fn new(t: ErrT, line: i32, start: i32, end: i32, file: String) -> Self {
        Self {
            t,
            line,
            start,
            end,
            file,
        }
    }
}
