#[derive(Debug,Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Fun,
    LazyP,
    ZeroP,
    Ident,
    SemiColon,
    Comma,
    EOL,
    LSmallB,
    RSmallB,
    LCurlyB,
    RCurlyB,
    LBigB,
    RBigB,
    Nil,
    Exclamation,
    At,
    Struct,
    Enum,
    Ret,
}


#[derive(Debug,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
    pub column_start: Option<usize>,
    pub column_end: Option<usize>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: Option<String>,
        line_start: Option<usize>,
        line_end: Option<usize>,
        column_start: Option<usize>,
        column_end: Option<usize>,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            line_start,
            line_end,
            column_start,
            column_end,
        }
    }
}

pub struct Lexer<'a> {
    pub source: &'a String,
    pub tokens: Vec<Token>,
    pub current: usize,
    pub line: usize,
    pub column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn add_token(
        &mut self,
        token_type: TokenType,
        lexeme: Option<String>,
        line: usize,
        start_col: usize,
        end_col: usize,
    ) {

        self.tokens.push(Token {
            token_type,
            lexeme,
            line_start: Some(line),
            line_end: Some(line),
            column_start: Some(start_col),
            column_end: Some(end_col),
        });
    }


    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}