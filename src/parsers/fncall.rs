use crate::{
    ast::{NT, Var},
    errt::t::{ErrT, SErr},
    tokdefs::{IDT, NUMT, TT, Tok},
};
use smallstr::SmallString;
use std::iter::Peekable;

#[allow(dead_code, unused)]
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

        let arg = match current_tok {
            Some(ref tok) => match parse_argument_token(tok, iter, file)? {
                Some(arg) => {
                    current_tok = iter.next();
                    arg
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
            },
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

        match expected_param {
            Var::Generic(_) => args.push(arg),
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
                    errs.push(SErr::new(
                        ErrT::TypeMismatch,
                        0,
                        0,
                        0,
                        file.to_string(),
                    ));
                }
            }
            Var::List(items) => {
                if let Var::List(a_items) = arg {
                    args.push(Var::List(a_items));
                } else {
                    errs.push(SErr::new(
                        ErrT::TypeMismatch,
                        0,
                        0,
                        0,
                        file.to_string(),
                    ));
                }
            }
            Var::Variadic => args.push(Var::Variadic),
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
            errs.push(SErr::new(
                ErrT::UnexpectedEof,
                0,
                0,
                0,
                file.to_string(),
            ));
        }
    }

    if let Some(next) = iter.next() {
        if next.tt != TT::SemiC {
            errs.push(SErr::new(
                ErrT::UnexpectTok,
                next.line,
                next.start,
                next.end,
                file.to_string(),
            ));
        }
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
                if matches!(next.tt, TT::LSmallB) {
                    let val = format!("'{}'", val);
                    Ok(Some(Var::Generic(SmallString::from(val))))
                } else {
                    Ok(Some(Var::Generic(SmallString::from(val.clone()))))
                }
            } else {
                Ok(Some(Var::Generic(SmallString::from(val.clone()))))
            }
        }
        TT::IDENT(IDT::DQ, ref val) => {
            let val = format!("\"{}\"", val);
            Ok(Some(Var::Generic(SmallString::from(val))))
        }
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