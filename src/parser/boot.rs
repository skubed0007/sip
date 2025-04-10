use crate::token::defs::{Token, TokenType};

use super::defs::NodeT;

pub fn bootp(tokens: &Vec<Token>) -> Vec<NodeT> {
    let /*mut*/ nodes = Vec::new();
    let mut tokiter = tokens.iter().peekable();
    while let Some(token) = tokiter.next() {
        match token.token_type {
            TokenType::Fun => {
                //parse function
                //let mut fnname = String::new();
            }
            _ => {}
        }
    }
    nodes
}
