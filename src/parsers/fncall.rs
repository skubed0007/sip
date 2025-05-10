use crate::{
    ast::{NT, Var},
    errt::t::{ErrT, SErr},
    tokdefs::{IDT, NUMT, TT, Tok},
};
use smallstr::SmallString;
use std::iter::Peekable;

#[allow(dead_code, unused)]

/// Parse a function call.
///
/// # Arguments
///
/// * `iter`: The tokens to parse.
/// * `fnname`: The name of the function to call.
/// * `ast`: The AST of the file.
/// * `file`: The name of the file being parsed.
///
/// # Returns
///
/// A `Result` containing the parsed AST node for the function call, or a `Vec` of `SErr`s containing
/// any errors encountered during parsing.
pub fn fncallp(
    iter: &mut Peekable<std::vec::IntoIter<Tok>>,
    fnname: String,
    ast: &[NT],
    file: &String,
) -> Result<NT, Vec<SErr>> {
    let mut errs = Vec::new();
    let mut args: Vec<Var> = Vec::new();

    let mut extern_decl = None;
    for node in ast {
        if let NT::Extern(name, params, c_name, _) = node {
            if name.as_str() == fnname {
                extern_decl = Some((params.clone(), c_name.clone()));
                break;
            }
        }
    }

    let (expected_params, _) = match extern_decl {
        Some(decl) => decl,
        None => {
            errs.push(SErr::new(ErrT::UnexpectTok, 0, 0, 0, file.to_string()));
            return Err(errs);
        }
    };

    match iter.next() {
        Some(tok) if matches!(tok.tt, TT::LSmallB) => {}
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
            errs.push(SErr::new(ErrT::UnexpectedEof, 0, 0, 0, file.to_string()));
            return Err(errs);
        }
    }

    let mut current_tok = iter.next();
    for (param_idx, expected_param) in expected_params.iter().enumerate() {
        if param_idx > 0 {
            match current_tok {
                Some(ref tok) if matches!(tok.tt, TT::Comma) => {
                    current_tok = iter.next();
                }
                Some(ref tok) => {
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
                    errs.push(SErr::new(ErrT::UnexpectedEof, 0, 0, 0, file.to_string()));
                    return Err(errs);
                }
            }
        }

        let arg = match current_tok {
            Some(ref tok) => match parse_argument_token(tok, iter, file)? {
                Some(arg) => {
                    current_tok = iter.next();
                    arg
                }
                None => {
                    errs.push(SErr::new(ErrT::UnexpectedEof, 0, 0, 0, file.to_string()));
                    return Err(errs);
                }
            },
            None => {
                errs.push(SErr::new(ErrT::UnexpectedEof, 0, 0, 0, file.to_string()));
                return Err(errs);
            }
        };

        match expected_param {
            Var::Generic(_) => {
                args.push(arg);
            }
            Var::I32(_, _) => {
                if let Var::I32(a_val, a_name) = arg {
                    args.push(Var::I32(a_val, a_name));
                } else {
                    errs.push(SErr::new(
                        ErrT::TypeMismatch,
                        0,
                        0,
                        0,
                        "Expected I32 type".to_owned(),
                    ));
                }
            }
            Var::F32(_, _) => {
                if let Var::F32(a_val, a_name) = arg {
                    args.push(Var::F32(a_val, a_name));
                } else {
                    errs.push(SErr::new(ErrT::TypeMismatch, 0, 0, 0, file.to_string()));
                }
            }
            Var::List(items) => {
                if let Var::List(a_items) = arg {
                    args.push(Var::List(a_items));
                } else {
                    errs.push(SErr::new(ErrT::TypeMismatch, 0, 0, 0, file.to_string()));
                }
            }
            Var::Variadic => {
                args.push(Var::Variadic);
            }
        }
    }

    match current_tok {
        Some(ref tok) if matches!(tok.tt, TT::RSmallB) => {}
        Some(ref tok) => {
            errs.push(SErr::new(
                ErrT::UnexpectTok,
                tok.line,
                tok.start,
                tok.end,
                file.to_string(),
            ));
        }
        None => {
            errs.push(SErr::new(ErrT::UnexpectedEof, 0, 0, 0, file.to_string()));
        }
    }
    if iter.peek().is_some() && iter.next().unwrap().tt != TT::SemiC {
        errs.push(SErr::new(ErrT::UnexpectTok, 0, 0, 0, file.to_string()));
    }
    if !errs.is_empty() {
        return Err(errs);
    }

    Ok(NT::fncall(SmallString::from(fnname.clone()), args))
}

/// Parses a token as a function call argument, converting it into a `Var` type.
///
/// # Arguments
///
/// * `tok` - The current token being parsed.
/// * `iter` - A mutable iterator over the remaining tokens.
/// * `file` - The name of the file being parsed, used for error reporting.
///
/// # Returns
///
/// A `Result` containing an `Option<Var>` if successfully parsed, or a vector of `SErr` if an error occurs.
///
/// # Errors
///
/// Returns an error if an unexpected token is encountered or if the end of the input is reached unexpectedly.

#[allow(dead_code, unused)]
fn parse_argument_token(
    tok: &Tok,
    iter: &mut Peekable<std::vec::IntoIter<Tok>>,
    file: &String,
) -> Result<Option<Var>, Vec<SErr>> {
    match tok.tt {
        TT::NUM(NUMT::I32(val)) => Ok(Some(Var::I32(val, SmallString::from("")))),
        TT::NUM(NUMT::F(val)) => Ok(Some(Var::F32(val, SmallString::from("")))),
        TT::IDENT(IDT::NQ, ref val) => {
            if let Some(next) = iter.next() {
                if matches!(next.tt, TT::LSmallB) {
                    return Ok(Some(Var::Generic(SmallString::from(val.clone()))));
                }
            }
            Ok(Some(Var::Generic(SmallString::from(val.clone()))))
        }
        TT::IDENT(IDT::DQ, ref val) => Ok(Some(Var::Generic(SmallString::from(val.clone())))),
        TT::LBigB => {
            let mut list_items = Vec::new();
            let mut current_tok = iter.next();

            loop {
                match current_tok {
                    Some(ref tok) if matches!(tok.tt, TT::RBigB) => {
                        current_tok = iter.next();
                        break;
                    }
                    Some(ref tok) if matches!(tok.tt, TT::Comma) => {
                        current_tok = iter.next();
                    }
                    Some(ref tok) => {
                        if let Some(item) = parse_argument_token(tok, iter, file)? {
                            list_items.push(item);
                            current_tok = iter.next();
                        } else {
                            return Err(vec![SErr::new(
                                ErrT::UnexpectedEof,
                                0,
                                0,
                                0,
                                file.to_string(),
                            )]);
                        }
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
