use super::defs::{Lexer, TokenType};
use crate::token::lexer::TokenType::*;

#[allow(unused, dead_code)]
impl<'a> Lexer<'a> {
    pub fn lex(&mut self) {
        let mut chars = self.source.char_indices().peekable();
        let mut line = 1;
        let mut column = 1;

        let mut curword = String::new();
        let mut token_start_col = column;

        while let Some((i, c)) = chars.next() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    if curword.is_empty() {
                        token_start_col = column;
                    }
                    curword.push(c);
                    column += 1;
                }

                // Word & symbol delimiters
                ' ' | '\t' | '\r' | '\n' | ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']' | '!' | '@' => {
                    // Flush word
                    if !curword.is_empty() {
                        self.match_word(&curword, line, token_start_col, column);
                        curword.clear();
                    }

                    let token = match c {
                        '\n' => Some(TokenType::EOL),
                        ';' => Some(TokenType::SemiColon),
                        ',' => Some(TokenType::Comma),
                        '(' => Some(TokenType::LSmallB),
                        ')' => Some(TokenType::RSmallB),
                        '{' => Some(TokenType::LCurlyB),
                        '}' => Some(TokenType::RCurlyB),
                        '[' => Some(TokenType::LBigB),
                        ']' => Some(TokenType::RBigB),
                        '!' => Some(TokenType::Exclamation),
                        '@' => Some(TokenType::At),
                        _ => None,
                    };

                    if let Some(tk) = token {
                        self.add_token(tk, None, line, column, column + 1);
                    }

                    if c == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }

                // Unknowns (skip)
                _ => {
                    column += 1;
                }
            }
        }

        // Final flush
        if !curword.is_empty() {
            self.match_word(&curword, line, token_start_col, column);
        }
    }

    fn match_word(&mut self, word: &str, line: usize, start_col: usize, end_col: usize) {
        let token = match word {
            "fun" => Fun,
            "lazypage" => LazyP,
            "zeropage" => ZeroP,
            "nil" => Nil,
            _ => TokenType::Ident,
        };

        self.add_token(token, Some(word.to_string()), line, start_col, end_col);
    }
}
