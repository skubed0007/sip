pub mod func;
pub mod init;
use crate::{
    ast::defs::NodeT,
    err::def::{ErrT, PErr}, err::edis::d1::diserr,
    token::defs::{Token, TT},
};

/// A simple parser that manages tokens and builds an AST.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    cur: Token,
    ast: Vec<NodeT>,
   pub  errs: Vec<PErr>,
}

impl Parser {
    /// Creates a new Parser instance with the provided tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of tokens produced by your lexer.
    ///
    /// # Returns
    ///
    /// A `Parser` set to the first token in the sequence.
    #[inline(always)]
    pub fn new(tokens: Vec<Token>) -> Self {
        let cur = tokens.first().cloned().unwrap_or(Token {
            TT: TT::Nil,
            line: 0,
            start: 0,
            end: 0,
            value: None,
        });
        Self {
            tokens,
            pos: 0,
            cur,
            ast: Vec::new(),
            errs: Vec::new(),
        }
    }

    /// Advances the parser to the next token.
    ///
    /// # Returns
    ///
    /// `Some(token)` if there is a next token, or `None` if the end of input has been reached.
    #[inline(always)]
    pub fn mv(&mut self) {
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
            self.cur = self.tokens[self.pos].clone();
        }
    }

    /// Returns a reference to the current token.
    #[inline(always)]
    pub fn current(&self) -> &Token {
        &self.cur
    }

    /// Peeks at the token one ahead without advancing the parser.
    ///
    /// # Returns
    ///
    /// `Some(&Token)` if available, or `None` if at the end.
    #[inline(always)]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    /// Consumes the current token if it matches the expected one.
    ///
    /// # Arguments
    ///
    /// * `expected` - A reference to the Token expected.
    ///
    /// # Returns
    ///
    /// Returns `true` if the token was successfully consumed, `false` otherwise.
    #[inline(always)]
    pub fn eat(&mut self, expected: &Token) -> bool
    where
        Token: PartialEq,
    {
        if &self.cur == expected {
            self.mv();
            true
        } else {
            false
        }
    }

    /// Drops all tokens that have been passed over. This removes
    /// tokens from the start of the vector up to the current position.
    ///
    /// After this call, `pos` is reset to 0 and `tokens` contains only
    /// the tokens at and after the current token.
    #[inline(always)]
    pub fn privdrop(&mut self) {
        self.tokens.drain(0..self.pos);
        self.pos = 0;
    }

    /// Look back at a token that has already been passed over.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of tokens back from the current token.
    ///
    /// # Returns
    ///
    /// `Some(&Token)` if it exists, or `None` otherwise.
    #[inline(always)]
    pub fn look_back(&self, offset: usize) -> Option<&Token> {
        if offset <= self.pos {
            self.tokens.get(self.pos - offset)
        } else {
            None
        }
    }

    /// Adds an AST node.
    #[inline(always)]
    pub fn add_node(&mut self, node: NodeT) {
        self.ast.push(node);
    }

    /// Returns the constructed AST.
    #[inline(always)]
    pub fn get_ast(&self) -> &Vec<NodeT> {
        &self.ast
    }
    ///add a err to err list
    #[inline(always)]
    pub fn add_err(&mut self, et: ErrT, line: usize, start: usize, end: usize) {
        self.errs.push(PErr {
            line,
            et,
            start,
            end,
        });
    }
    pub fn erroccur(&self) -> bool {
        //println!("called erroccur");
        !self.errs.is_empty()
    }
    pub fn show_errs(&self,code: &memmap2::Mmap) {
        //println!("calling show errs");
        for err in &self.errs {
           // println!("calling diserr");
           println!( "{}", diserr(err, &code));
        }
    }
}
