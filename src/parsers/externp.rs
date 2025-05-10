use smallstr::SmallString;
use std::iter::Peekable;

use crate::{
    ast::{Var, ID_CAP, NT},
    errt::t::{ErrT, SErr},
    tokdefs::{Tok, IDT, TT},
};

#[derive(PartialEq, Clone)]
pub enum State {
    ExpectSipName,
    ExpectSipArgOrEq,
    ExpectEq,
    ExpectCName,
    ExpectCArgOrSemicolon,
    Done,
}

/// Parses a SIP extern declaration.
///
/// This function parses a sequence of tokens to extract the SIP function name,
/// its arguments, the C function name, and its arguments. It does this by
/// transitioning through a number of states as it processes the tokens.
///
/// If the parse is successful, it returns an `NT::Extern` variant. If any
/// parsing errors occur, it returns a vector of `SErr`.
pub fn externp(tokiter: &mut std::iter::Peekable<std::vec::IntoIter<Tok>>, filename: &String) -> Result<NT, Vec<SErr>> {
    let mut errs: Vec<SErr> = Vec::new();
    let mut sipname: SmallString<[u8; ID_CAP]> = SmallString::from("uninitialized");
    let mut cname: SmallString<[u8; ID_CAP]> = SmallString::from("uninitialized");
    let mut sipfnargs: Vec<Var> = Vec::new();
    let mut cfnargs: Vec<Var> = Vec::new();

    let mut state = State::ExpectSipName;
    let mut iter: Peekable<&mut Peekable<std::vec::IntoIter<Tok>>> = tokiter.by_ref().peekable();

    while let Some(tok) = iter.next() {
        match state {
            State::ExpectSipName => {
                if let TT::IDENT(IDT::NQ, name) = &tok.tt {
                    sipname = SmallString::from(name.as_str());
                    state = State::ExpectSipArgOrEq;
                } else {
                    errs.push(SErr::new(
                        ErrT::ExpectName,
                        tok.line,
                        tok.start,
                        tok.end,
                        filename.clone(),
                    ));
                }
            }

            State::ExpectSipArgOrEq => match &tok.tt {
                TT::EQS => {
                    state = State::ExpectCName;
                }
                _ => {
                    match parse_typed_var(&tok, &mut iter, filename) {
                        Ok(Some(var)) => {
                            sipfnargs.push(var.clone());
                            if let Var::Variadic = var {
                                state = State::ExpectEq;
                            }
                        }
                        Ok(None) => {
                            errs.push(SErr::new(
                                ErrT::ExpectName,
                                tok.line,
                                tok.start,
                                tok.end,
                                filename.clone(),
                            ));
                        }
                        Err(mut e) => errs.append(&mut e),
                    }
                }
            }

            State::ExpectEq => match &tok.tt {
                TT::EQS => {
                    state = State::ExpectCName;
                }
                _ => {
                    errs.push(SErr::new(
                        ErrT::ExpectOperator,
                        tok.line,
                        tok.start,
                        tok.end,
                        filename.clone(),
                    ));
                }
            }

            State::ExpectCName => {
                if let TT::IDENT(IDT::DQ, name) = &tok.tt {
                    cname = SmallString::from(name.as_str());
                    state = State::ExpectCArgOrSemicolon;
                } else {
                    errs.push(SErr::new(
                        ErrT::ExpectName,
                        tok.line,
                        tok.start,
                        tok.end,
                        filename.clone(),
                    ));
                }
            }

            State::ExpectCArgOrSemicolon => match &tok.tt {
                TT::SemiC => {
                    state = State::Done;
                    break;
                }
                _ => {
                    match parse_typed_var(&tok, &mut iter, filename) {
                        Ok(Some(var)) => {
                            cfnargs.push(var.clone());
                            if let Var::Variadic = var {
                                state = State::ExpectCArgOrSemicolon;
                            }
                        }
                        Ok(None) => {
                            errs.push(SErr::new(
                                ErrT::ExpectName,
                                tok.line,
                                tok.start,
                                tok.end,
                                filename.clone(),
                            ));
                        }
                        Err(mut e) => errs.append(&mut e),
                    }
                }
            }

            State::Done => unreachable!(),
        }
    }

    if state != State::Done {
        errs.push(SErr::new(
            ErrT::MissingSemicolon,
            -1,
            -1,
            -1,
            filename.clone(),
        ));
    }

    if errs.is_empty() {
        Ok(NT::Extern(sipname, sipfnargs, cname, cfnargs))
    } else {
        Err(errs)
    }
}

fn parse_typed_var(
    first_tok: &Tok,
    iter: &mut Peekable<&mut Peekable<std::vec::IntoIter<Tok>>>,
    filename: &str,
) -> Result<Option<Var>, Vec<SErr>> {
    // Handle standalone variadic argument
    if let TT::DDD = first_tok.tt {
        return Ok(Some(Var::Variadic));
    }

    // Handle regular typed variable
    let name = if let TT::IDENT(IDT::NQ, n) = &first_tok.tt {
        SmallString::from(n.as_str())
    } else {
        return Ok(None);
    };

    if let Some(next_tok) = iter.peek().cloned() {
        if let TT::LSmallB = next_tok.tt {
            let _ = iter.next(); // consume LParen

            let type_tok = match iter.next() {
                Some(tok) => tok,
                None => {
                    return Err(vec![SErr::new(
                        ErrT::InvalidNumber,
                        next_tok.line,
                        next_tok.start,
                        next_tok.end,
                        filename.to_string(),
                    )]);
                }
            };

            let rparen = match iter.next() {
                Some(tok) => tok,
                None => {
                    return Err(vec![SErr::new(
                        ErrT::InvalidNumber,
                        next_tok.line,
                        next_tok.start,
                        next_tok.end,
                        filename.to_string(),
                    )]);
                }
            };

            if !matches!(rparen.tt, TT::RSmallB) {
                return Err(vec![SErr::new(
                    ErrT::UnmatchedParen,
                    rparen.line,
                    rparen.start,
                    rparen.end,
                    filename.to_string(),
                )]);
            }

            return match &type_tok.tt {
                TT::F32 => Ok(Some(Var::F32(0.0, name))),
                TT::I32 => Ok(Some(Var::I32(0, name))),
                _ => {
                    Ok(Some(Var::Generic(name)))
                }
            };
        }
    }

    Ok(Some(Var::Generic(name)))
}