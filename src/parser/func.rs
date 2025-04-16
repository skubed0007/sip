use super::Parser;
use crate::{
    ast::defs::{FUNC, NodeT, Var, VarT},
    err::def::ErrT,
    token::defs::{IDType, TT, Token},
};
use std::{borrow::Cow, process::exit};

#[allow(unused)]
impl Parser {
    pub fn pfunc(&mut self) {
        let fname = match &self.cur.TT {
            TT::Ident(id_type) if *id_type != IDType::DQ && *id_type != IDType::SQ => {
                self.token_val().unwrap_or(Cow::Borrowed("_"))
            }
            _ => {
                self.add_err(ErrT::FuncNameNotFound, self.cur.line, self.cur.start, self.cur.end);
                Cow::Borrowed("_")
            }
        };

        self.mv();

        if self.cur.TT != TT::LSmallB {
            self.add_err(ErrT::SyntaxErr("expected a '('"), self.cur.line, self.cur.start, self.cur.end);
        }
        self.mv();

        let mut params = Vec::new();
        while self.cur.TT != TT::RsmallB {
            let param_type = self.cur.clone();
            self.mv();

            if self.cur.TT == TT::Ident(IDType::Nil) {
                if let Some(name) = self.token_val() {
                    params.push((param_type, name));
                } else {
                    self.add_err(ErrT::ExpectParamName, self.cur.line, self.cur.start, self.cur.end);
                }
                self.mv();
            } else {
                self.add_err(ErrT::ExpectParamName, self.cur.line, self.cur.start, self.cur.end);
            }

            if self.cur.TT == TT::Comma {
                self.mv();
            }

            if self.pos >= self.tokens.len() {
                self.add_err(ErrT::SyntaxErr("unexpected end of input"), self.cur.line, self.cur.start, self.cur.end);
                break;
            }
        }

        self.mv();

        if self.cur.TT != TT::At {
            self.add_err(ErrT::ExpectedAT, self.cur.line, self.cur.start, self.cur.end);
        }
        self.mv();

        let mut ret_type = Token { TT: TT::I32, line: 0, start: 0, end: 0, value: None }; // Default type

        // Check for valid return type
        if matches!(self.cur.TT, TT::F32 | TT::F64 | TT::I32 | TT::I64 | TT::U8) {
            ret_type = self.cur.clone(); // Update ret_type if matched
            self.mv();
        } else {
            self.add_err(ErrT::SyntaxErr("Expected valid return type"), self.cur.line, self.cur.start, self.cur.end);
        }

        if self.cur.TT != TT::LCurlyB {
            self.add_err(ErrT::SyntaxErr("expected { to start function body"), self.cur.line, self.cur.start, self.cur.end);
        }
        self.mv();

        let body_tokens = self.collect_block_tokens();

        // Validate return type against function body
        self.validate_return_type(&body_tokens, &ret_type);

        let func_node = FUNC {
            name: fname.into_owned(),
            args: (!params.is_empty()).then_some(
                params.into_iter().map(|(tok, name)| Var {
                    name: name.into_owned(),
                    dyna: false,
                    TT: self.map_token_to_vartype(&tok),
                }).collect()
            ),
            body: vec![], // TODO: parse body tokens into AST
            ret: Var {
                name: self.token_val_from(&ret_type).unwrap_or_else(|| "_".into()).into_owned(),
                dyna: false,
                TT: self.map_token_to_vartype(&ret_type),
            },
        };

        self.add_node(NodeT::FUNCNode(func_node));
    }

    fn token_val(&self) -> Option<Cow<'static, str>> {
        self.cur.value.as_ref().map(|v| Cow::Owned(v.clone()))
    }

    fn token_val_from(&self, tok: &Token) -> Option<Cow<'static, str>> {
        tok.value.as_ref().map(|v| Cow::Owned(v.clone()))
    }

    fn collect_block_tokens(&mut self) -> Vec<Token> {
        let mut body_tokens = Vec::new();
        let mut brace_count = 1;

        while brace_count > 0 {
            if self.pos >= self.tokens.len() {
                self.add_err(ErrT::SyntaxErr("unexpected end of function body"), self.cur.line, self.cur.start, self.cur.end);
                break;
            }

            let tok = self.cur.clone();
            match tok.TT {
                TT::LCurlyB => brace_count += 1,
                TT::RCurlyB => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        self.mv();
                        break;
                    }
                }
                _ => {}
            }

            body_tokens.push(tok);
            self.mv();
        }
        body_tokens
    }

    fn validate_return_type(&mut self, body: &[Token], ret_type: &Token) {
        let mut return_found = false;

        for (i, token) in body.iter().enumerate() {
            if token.TT == TT::Ret {
                return_found = true;

                if let Some(ret_val) = body.get(i + 1) {
                    if !self.validate_token_against_return_type(ret_val, ret_type) {
                        self.add_err(ErrT::RetTMisMatch("Return type mismatch"), self.cur.line, self.cur.start, self.cur.end);
                    }
                    return;
                } else {
                    self.add_err(ErrT::RetTMisMatch("unexpected return value"), self.cur.line, self.cur.start, self.cur.end);
                    return;
                }
            }
        }

        // Handle case if no return statement is found
        if !return_found {
            let last_expr = body.iter().rev().find(|tok| tok.TT != TT::SemiColon);
            match last_expr {
                Some(token) if !self.validate_token_against_return_type(token, ret_type) => {
                    self.add_err(ErrT::RetTMisMatch("Last expr does not match return type"), self.cur.line, self.cur.start, self.cur.end);
                }
                None => {
                    self.add_err(ErrT::RetTNF, self.cur.line, self.cur.start, self.cur.end);
                    exit(1);
                }
                _ => {}
            }
        }
    }

    fn validate_token_against_return_type(&mut self, token: &Token, ret_type: &Token) -> bool {
        let expected = self.token_val_from(ret_type).unwrap_or_else(|| "_".into());
        match expected.as_ref() {
            "i32" => token.TT == TT::NUM && token.value.as_ref().and_then(|v| v.parse::<i32>().ok()).is_some(),
            "i64" => token.TT == TT::NUM && token.value.as_ref().and_then(|v| v.parse::<i64>().ok()).is_some(),
            "f32" => token.TT == TT::NUM && token.value.as_ref().and_then(|v| v.parse::<f32>().ok()).is_some(),
            "f64" => token.TT == TT::NUM && token.value.as_ref().and_then(|v| v.parse::<f64>().ok()).is_some(),
            "u8"  => token.TT == TT::NUM && token.value.as_ref().and_then(|v| v.parse::<u8>().ok()).is_some(),
            _ => {
                self.add_err(ErrT::RetTMisMatch("Unsupported return type"), self.cur.line, self.cur.start, self.cur.end);
                false
            }
        }
    }

    fn map_token_to_vartype(&mut self, tok: &Token) -> VarT {
        match tok.TT {
            TT::I32 | TT::Ident(IDType::Nil) if self.token_val_from(tok).as_deref() == Some("i32") => VarT::I32(0),
            TT::I64 | TT::Ident(IDType::Nil) if self.token_val_from(tok).as_deref() == Some("i64") => VarT::I64(0),
            TT::F32 | TT::Ident(IDType::Nil) if self.token_val_from(tok).as_deref() == Some("f32") => VarT::F32(0.0),
            TT::F64 | TT::Ident(IDType::Nil) if self.token_val_from(tok).as_deref() == Some("f64") => VarT::F64(0.0),
            TT::U8  | TT::Ident(IDType::Nil) if self.token_val_from(tok).as_deref() == Some("u8")  => VarT::U8(0),
            _ => {
                self.add_err(ErrT::InvVT, self.cur.line, self.cur.start, self.cur.end);
                VarT::I32(0)
            }
        }
    }
}
