use crate::{
    err::defs::ErrT,
    token::defs::{Token, TokenType},
};

use super::defs::{FunDef, NodeT, Var, VarType};

#[allow(unused)]
pub fn bootp(tokens: &Vec<Token>) -> (Vec<NodeT>, Vec<ErrT>) {
    let mut nodes = Vec::new();
    let mut errlist: Vec<ErrT> = Vec::new();
    let mut line = 0;
    let mut tokiter = tokens.iter().peekable();

    while let Some(token) = tokiter.next() {
        match token.token_type {
            TokenType::EOL => {
                line += 1;
            }

            TokenType::Fun => {
                // Parse function name
                let fnname = {
                    let mut collected_idents = Vec::new();
                    while let Some(nxttok) = tokiter.peek() {
                        match (nxttok.token_type, &nxttok.lexeme) {
                            (TokenType::Ident, Some(name)) => {
                                collected_idents.push(name.clone());
                                tokiter.next();
                            }
                            (TokenType::LSmallB, _) => break,
                            _ => {
                                errlist.push(ErrT::SyntaxError {
                                    line,
                                    column_start: nxttok.column_start.unwrap_or(0),
                                    column_end: nxttok.column_end.unwrap_or(0),
                                });
                                break;
                            }
                        }
                    }

                    if collected_idents.len() != 1 {
                        errlist.push(ErrT::MissFnName {
                            line,
                            column_start: token.column_start.unwrap_or(0),
                            column_end: token.column_end.unwrap_or(0),
                        });
                        continue;
                    }

                    collected_idents.pop().unwrap()
                };

                // Expect argument start `(`
                match tokiter.next() {
                    Some(tok) if tok.token_type == TokenType::LSmallB => {}
                    Some(tok) => {
                        errlist.push(ErrT::ExpectedArgStart {
                            line,
                            column_start: tok.column_start.unwrap_or(0),
                            column_end: tok.column_end.unwrap_or(0),
                        });
                        continue;
                    }
                    None => break,
                }

                // Parse arguments
                let mut args: Vec<Var> = Vec::new();
                let mut expecting_type = true;
                let mut current_type: Option<VarType> = None;

                while let Some(tok) = tokiter.peek() {
                    match tok.token_type {
                        TokenType::RSmallB => {
                            tokiter.next();
                            break;
                        }

                        TokenType::Comma => {
                            tokiter.next();
                            expecting_type = true;
                        }

                        TokenType::Ident => {
                            let val = tok.lexeme.clone().unwrap_or_default();
                            if expecting_type {
                                current_type = parse_type(&val, &mut errlist, line, tok);
                                tokiter.next();
                                expecting_type = false;
                            } else {
                                if let Some(vtype) = current_type {
                                    args.push(Var {
                                        name: val,
                                        var_type: vtype,
                                        size: None,
                                    });
                                } else {
                                    errlist.push(ErrT::SyntaxError {
                                        line,
                                        column_start: tok.column_start.unwrap_or(0),
                                        column_end: tok.column_end.unwrap_or(0),
                                    });
                                }
                                tokiter.next();
                                current_type = None;
                                expecting_type = true;
                            }
                        }

                        _ => {
                            errlist.push(ErrT::SyntaxError {
                                line,
                                column_start: tok.column_start.unwrap_or(0),
                                column_end: tok.column_end.unwrap_or(0),
                            });
                            tokiter.next();
                        }
                    }
                }

                // Parse return type
                let mut return_types = Vec::new();
                let mut has_return = false;

                if let Some(at_tok) = tokiter.peek() {
                    if at_tok.token_type == TokenType::At {
                        has_return = true;
                        tokiter.next(); // Consume `@`

                        if let Some(typ_tok) = tokiter.peek() {
                            if typ_tok.token_type == TokenType::LSmallB {
                                tokiter.next(); // Consume `(`
                                while let Some(next) = tokiter.peek() {
                                    match next.token_type {
                                        TokenType::Ident => {
                                            let name = next.lexeme.clone().unwrap_or_default();
                                            if let Some(rt) =
                                                parse_type(&name, &mut errlist, line, next)
                                            {
                                                return_types.push(rt);
                                            }
                                            tokiter.next();
                                        }
                                        TokenType::Comma => {
                                            tokiter.next();
                                        }
                                        TokenType::RSmallB => {
                                            tokiter.next();
                                            break;
                                        }
                                        _ => {
                                            errlist.push(ErrT::SyntaxError {
                                                line,
                                                column_start: next.column_start.unwrap_or(0),
                                                column_end: next.column_end.unwrap_or(0),
                                            });
                                            break;
                                        }
                                    }
                                }
                            } else {
                                // Single return type (not a tuple)
                                let name = typ_tok.lexeme.clone().unwrap_or_default();
                                if let Some(rt) = parse_type(&name, &mut errlist, line, typ_tok) {
                                    return_types.push(rt);
                                }
                                tokiter.next();
                            }
                        }
                    }
                }

                if !has_return {
                    errlist.push(ErrT::MissingReturnType {
                        line,
                        column_start: token.column_start.unwrap_or(0),
                        column_end: token.column_end.unwrap_or(0),
                    });
                }
                if return_types.contains(&VarType::Nil) && return_types.len() != 1 {
                    errlist.push(ErrT::TupleNil { line: line, column_start: token.column_start.unwrap(), column_end: token.column_end.unwrap() });
                } 
                // Store function node
                let node = NodeT::FunDef(FunDef {
                    name: Box::leak(fnname.into_boxed_str()),
                    args: Some(args),
                    body: vec![],
                    ret_type: if return_types.len() == 1 {
                        Some(return_types[0].clone())
                    } else if return_types.len() > 1 {
                        Some(VarType::Tuple(return_types))
                    } else {
                        None
                    },
                });

                nodes.push(node);
            }

            _ => {}
        }
    }

    (nodes, errlist)
}

/// Helper function to map identifier strings to VarType enum.
fn parse_type(
    val: &str,
    errlist: &mut Vec<ErrT>,
    line: usize,
    tok: &Token,
) -> Option<VarType> {
    match val {
        "i32" => Some(VarType::I32),
        "i64" => Some(VarType::I64),
        "f32" => Some(VarType::F32),
        "f64" => Some(VarType::F64),
        "char" => Some(VarType::Char),
        "lazypage" => Some(VarType::LazyPage),
        "zeropage" => Some(VarType::ZeroPage),
        "nil" => Some(VarType::Nil),
        _ => {
            errlist.push(ErrT::InvVarT {
                line,
                column_start: tok.column_start.unwrap_or(0),
                column_end: tok.column_end.unwrap_or(0),
            });
            None
        }
    }
}
