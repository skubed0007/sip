use crate::{
    ast::{NT, Var},
    errt::t::{ErrT, SErr},
    tokdefs::{IDT, NUMT, TT, Tok},
};
use smallstr::SmallString;
use std::iter::Peekable;

pub fn fncallp(
    iter: &mut Peekable<std::vec::IntoIter<Tok>>,
    fnname: String,
    ast: &[NT],
    file: &String,
) -> Result<NT, Vec<SErr>> {
    let mut errs = Vec::new();
    let mut args = Vec::new();
    let mut is_variadic = false;

    // Find extern declaration
    let (expected_params, _) = match ast.iter().find_map(|node| {
        if let NT::Extern(name, params, c_name, _) = node {
            if name == &fnname {
                Some((params.clone(), c_name.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }) {
        Some(decl) => decl,
        None => {
            errs.push(SErr::new(ErrT::UnexpectTok, 0, 0, 0, file.to_string()));
            return Err(errs);
        }
    };

    // Expect '('
    match iter.next() {
        Some(tok) if tok.tt == TT::LSmallB => {}
        Some(tok) => {
            errs.push(SErr::new(
                ErrT::UnexpectTok,
                tok.line,
                tok.start,
                tok.end,
                file.to_string(),
            ));
            return Err(errs);
        }
        None => {
            errs.push(SErr::new(
                ErrT::UnexpectedEof,
                0,
                0,
                0,
                file.to_string(),
            ));
            return Err(errs);
        }
    }

    // Parse parameters
    for (param_idx, expected_param) in expected_params.iter().enumerate() {
        // Handle comma between params
        if param_idx > 0 {
            match iter.peek() {
                Some(tok) if tok.tt == TT::Comma => {
                    iter.next();
                }
                Some(tok) => {
                    errs.push(SErr::new(
                        ErrT::UnexpectTok,
                        tok.line,
                        tok.start,
                        tok.end,
                        file.to_string(),
                    ));
                    return Err(errs);
                }
                None => {
                    errs.push(SErr::new(
                        ErrT::UnexpectedEof,
                        0,
                        0,
                        0,
                        file.to_string(),
                    ));
                    return Err(errs);
                }
            }
        }

        // Handle variadic
        if let Var::Variadic = expected_param {
            let mut var_args = Vec::new();
            is_variadic = true;

            loop {
                match iter.peek().cloned() {
                    Some(tok) if tok.tt == TT::RSmallB => {
                        iter.next();
                        break;
                    }
                    Some(tok) if tok.tt == TT::Comma => {
                        iter.next();
                    }
                    Some(tok) => {
                        if let Some(arg) = parse_argument_token(&tok, iter, file)? {
                            var_args.push(arg);
                            iter.next();
                        } else {
                            errs.push(SErr::new(
                                ErrT::UnexpectedEof,
                                0,
                                0,
                                0,
                                file.to_string(),
                            ));
                            return Err(errs);
                        }
                    }
                    None => {
                        errs.push(SErr::new(
                            ErrT::UnexpectedEof,
                            0,
                            0,
                            0,
                            file.to_string(),
                        ));
                        return Err(errs);
                    }
                }
            }

            args.push(Var::List(var_args));
            break;
        }

        // Parse fixed param
        let tok = match iter.peek().cloned() {
            Some(t) => t,
            None => {
                errs.push(SErr::new(
                    ErrT::UnexpectedEof,
                    0,
                    0,
                    0,
                    file.to_string(),
                ));
                return Err(errs);
            }
        };

        let parsed = match parse_argument_token(&tok, iter, file) {
            Ok(Some(arg)) => {
                iter.next();
                Some(arg)
            }
            Ok(None) => {
                errs.push(SErr::new(
                    ErrT::UnexpectedEof,
                    0,
                    0,
                    0,
                    file.to_string(),
                ));
                return Err(errs);
            }
            Err(e) => return Err(e),
        };

        if let Some(arg) = parsed {
            match expected_param {
                Var::I32(_, _) => {
                    if let Var::I32(val, name) = arg {
                        args.push(Var::I32(val, name));
                    } else {
                        errs.push(SErr::new(
                            ErrT::TypeMismatch,
                            tok.line,
                            tok.start,
                            tok.end,
                            file.to_string(),
                        ));
                    }
                }
                Var::F32(_, _) => {
                    if let Var::F32(val, name) = arg {
                        args.push(Var::F32(val, name));
                    } else {
                        errs.push(SErr::new(
                            ErrT::TypeMismatch,
                            tok.line,
                            tok.start,
                            tok.end,
                            file.to_string(),
                        ));
                    }
                }
                Var::List(_) => {
                    if let Var::List(items) = arg {
                        args.push(Var::List(items));
                    } else {
                        errs.push(SErr::new(
                            ErrT::TypeMismatch,
                            tok.line,
                            tok.start,
                            tok.end,
                            file.to_string(),
                        ));
                    }
                }
                Var::Generic(_) => args.push(arg),
                _ => {}
            }
        } else {
            errs.push(SErr::new(
                ErrT::UnexpectedEof,
                tok.line,
                tok.start,
                tok.end,
                file.to_string(),
            ));
            return Err(errs);
        }
    }

    // Handle closing ')'
    if !is_variadic {
        if let Some(tok) = iter.peek() {
            if tok.tt == TT::RSmallB {
                iter.next();
            } else {
                errs.push(SErr::new(
                    ErrT::UnexpectTok,
                    tok.line,
                    tok.start,
                    tok.end,
                    file.to_string(),
                ));
                return Err(errs);
            }
        } else {
            errs.push(SErr::new(
                ErrT::UnexpectedEof,
                0,
                0,
                0,
                file.to_string(),
            ));
            return Err(errs);
        }
    }

    // Expect ';'
    if let Some(tok) = iter.peek() {
        if tok.tt == TT::SemiC {
            iter.next();
        } else {
            errs.push(SErr::new(
                ErrT::UnexpectTok,
                tok.line,
                tok.start,
                tok.end,
                file.to_string(),
            ));
        }
    } else {
        errs.push(SErr::new(
            ErrT::UnexpectedEof,
            0,
            0,
            0,
            file.to_string(),
        ));
    }

    if !errs.is_empty() {
        return Err(errs);
    }

    Ok(NT::FNCALL(SmallString::from(fnname), args))
}

#[allow(dead_code, unused)]
fn parse_argument_token(
    tok: &Tok,
    iter: &mut Peekable<std::vec::IntoIter<Tok>>,
    file: &String,
) -> Result<Option<Var>, Vec<SErr>> {
    match tok.tt {
        TT::NUM(NUMT::I32(val)) => Ok(Some(Var::I32(val, SmallString::new()))),
        TT::NUM(NUMT::F(val)) => Ok(Some(Var::F32(val, SmallString::new()))),
        TT::IDENT(IDT::NQ, ref val) => {
            if let Some(next) = iter.peek() {
                if next.tt == TT::LSmallB {
                    Ok(Some(Var::Generic(SmallString::from(format!("'{}'", val)))))
                } else {
                    Ok(Some(Var::Generic(SmallString::from(val.clone()))))
                }
            } else {
                Ok(Some(Var::Generic(SmallString::from(val.clone()))))
            }
        }
        TT::IDENT(IDT::DQ, ref val) => {
            Ok(Some(Var::Generic(SmallString::from(format!("\"{}\"", val)))))
        }
        TT::LBigB => {
            let mut list_items = Vec::new();
            let mut current_tok = iter.next();

            loop {
                match current_tok {
                    Some(ref tok) if tok.tt == TT::RBigB => break,
                    Some(ref tok) if tok.tt == TT::Comma => current_tok = iter.next(),
                    Some(ref tok) => {
                        if let Some(item) = parse_argument_token(tok, iter, file)? {
                            list_items.push(item);
                        } else {
                            return Err(vec![SErr::new(
                                ErrT::UnexpectedEof,
                                0,
                                0,
                                0,
                                file.to_string(),
                            )]);
                        }
                        current_tok = iter.next();
                    }
                    None => {
                        return Err(vec![SErr::new(
                            ErrT::UnexpectedEof,
                            0,
                            0,
                            0,
                            file.to_string(),
                        )]);
                    }
                }
            }

            Ok(Some(Var::List(list_items)))
        }
        _ => Err(vec![SErr::new(
            ErrT::UnexpectTok,
            tok.line,
            tok.start,
            tok.end,
            file.to_string(),
        )]),
    }
}