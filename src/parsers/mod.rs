use externp::externp;
use fncall::fncallp;
use linkp::linkp;

use crate::{
    ast::NT,
    errt::t::{ErrT, SErr},
    tokdefs::{Tok, IDT, TT},
};

pub mod linkp;
pub mod externp;
pub mod fncall;

    /// Parse a sequence of tokens into an AST.
    ///
    /// # Arguments
    ///
    /// * `tokens`: The sequence of tokens to parse.
    /// * `filename`: The name of the file being parsed, used for error reporting.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `NT` if the parse is successful, or a `Vec` of `SErr` if any errors occur.
    ///
    /// # Errors
    ///
    /// Errors are returned if unexpected tokens are encountered, or if the end of the file is reached unexpectedly.
pub fn parse(tokens: Vec<Tok>, filename: &String) -> Result<Vec<NT>, Vec<SErr>> {
    let mut errs: Vec<SErr> = Vec::new();
    let mut ast: Vec<NT> = Vec::new();
    let mut tokiter: std::iter::Peekable<std::vec::IntoIter<Tok>> = tokens.into_iter().peekable();

    while let Some(tok) = tokiter.next() {
        match &tok.tt {
            TT::CLN => {
                match linkp(&mut tokiter, filename) {
                    Ok(nds) => ast.push(nds),
                    Err(e) => errs.extend(e),
                }
            }
            TT::EXTERN => {
                match externp(&mut tokiter, filename) {
                    Ok(nds) => ast.push(nds),
                    Err(e) => errs.extend(e),
                }
            }
            TT::IDENT(IDT::NQ, val) => {
                if let Some(next_tok) = tokiter.peek() {
                    if next_tok.tt != TT::EQS {
                        // is a function call
                        match fncallp(&mut tokiter, val.clone(),&ast,filename){
                            Ok(nds) => ast.push(nds),
                            Err(e) => errs.extend(e),
                        }
                    }
                }
            }
            _ => {
                errs.push(SErr {
                    t: ErrT::UnexpectTok,
                    line: tok.line,
                    start: tok.start,
                    end: tok.end,
                    file: filename.to_string(),
                });
            }
        }
    }
    if !errs.is_empty() { Err(errs) } else { Ok(ast) }
}
