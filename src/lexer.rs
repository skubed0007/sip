use crate::tokdefs::{IDT, NUMT, OPT, TT, Tok};

#[inline(always)]
fn is_ident_char(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
}

#[inline(always)]
fn is_digit_or_dot(c: u8) -> bool {
    matches!(c, b'0'..=b'9' | b'.')
}

#[allow(unused)]
pub fn lex(code: &[u8]) -> Vec<Tok> {
    let mut tokens = Vec::with_capacity(256);
    let mut line = 1;
    let mut col = 0;
    let mut current = 0;

    while current < code.len() {
        let b = code[current];
        let start_col = col;
        let start = current;

        match b {
            b' ' | b'\t' => {
                current += 1;
                col += 1;
            }
            b'\n' => {
                current += 1;
                line += 1;
                col = 0;
            }
            b'/' => {
                if current + 1 < code.len() {
                    match code[current + 1] {
                        b'/' => {
                            current += 2;
                            col += 2;
                            while current < code.len() && code[current] != b'\n' {
                                current += 1;
                                col += 1;
                            }
                            continue;
                        }
                        b'*' => {
                            current += 2;
                            col += 2;
                            while current + 1 < code.len() && !code[current..].starts_with(b"*/") {
                                if code[current] == b'\n' {
                                    line += 1;
                                    col = 0;
                                } else {
                                    col += 1;
                                }
                                current += 1;
                            }
                            current += 2;
                            col += 2;
                            continue;
                        }
                        _ => {}
                    }
                }
                tokens.push(Tok {
                    tt: TT::OP(OPT::DIV),
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'=' => {
                tokens.push(Tok {
                    tt: TT::EQS,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'"' => {
                let mut str_end = current + 1;
                while str_end < code.len() && code[str_end] != b'"' {
                    str_end += 1;
                }
                let string = String::from_utf8_lossy(&code[current + 1..str_end]).into_owned();
                col += (str_end - current + 1) as i32;
                tokens.push(Tok {
                    tt: TT::IDENT(IDT::DQ, string),
                    line,
                    start: start_col,
                    end: col - 1,
                });
                current = str_end + 1;
            }
            b'0'..=b'9' => {
                let mut end = current;
                while end < code.len() && is_digit_or_dot(code[end]) {
                    end += 1;
                }
                let slice = &code[current..end];
                let s = unsafe { std::str::from_utf8_unchecked(slice) };
                let tt = if s.contains('.') {
                    TT::NUM(NUMT::F(s.parse().unwrap()))
                } else {
                    TT::NUM(NUMT::I32(s.parse().unwrap()))
                };
                col += (end - current) as i32;
                tokens.push(Tok {
                    tt,
                    line,
                    start: start_col,
                    end: col - 1,
                });
                current = end;
            }
            b'+' => {
                tokens.push(Tok {
                    tt: TT::OP(OPT::ADD),
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'-' => {
                tokens.push(Tok {
                    tt: TT::OP(OPT::SUB),
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'*' => {
                tokens.push(Tok {
                    tt: TT::OP(OPT::MUL),
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'(' => {
                tokens.push(Tok {
                    tt: TT::LSmallB,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b')' => {
                tokens.push(Tok {
                    tt: TT::RSmallB,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'[' => {
                tokens.push(Tok {
                    tt: TT::LBigB,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b']' => {
                tokens.push(Tok {
                    tt: TT::RBigB,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b'.' => {
                if current + 2 < code.len()
                    && code[current + 1] == b'.'
                    && code[current + 2] == b'.'
                {
                    tokens.push(Tok {
                        tt: TT::DDD,
                        line,
                        start: start_col,
                        end: col + 2,
                    });
                    current += 3;
                    col += 3;
                } else {
                    tokens.push(Tok {
                        tt: TT::ErrT(".".to_string()),
                        line,
                        start: start_col,
                        end: col,
                    });
                    current += 1;
                    col += 1;
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let mut end = current;
                while end < code.len() && is_ident_char(code[end]) {
                    end += 1;
                }
                let ident = unsafe { std::str::from_utf8_unchecked(&code[current..end]) };
                col += (end - current) as i32;
                let tt = match ident {
                    "extern" => TT::EXTERN,
                    "link" => TT::LN,
                    "clink" => TT::CLN,
                    "i32" => TT::I32,
                    "f32" => TT::F32,
                    "u8" => TT::U8,
                    "call" => TT::CALL,
                    _ => TT::IDENT(IDT::NQ, ident.to_string()),
                };

                tokens.push(Tok {
                    tt,
                    line,
                    start: start_col,
                    end: col - 1,
                });
                current = end;
            }
            b',' => {
                tokens.push(Tok {
                    tt: TT::Comma,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            b';' => {
                tokens.push(Tok {
                    tt: TT::SemiC,
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
            _ => {
                tokens.push(Tok {
                    tt: TT::ErrT((b as char).to_string()),
                    line,
                    start: start_col,
                    end: col,
                });
                current += 1;
                col += 1;
            }
        }
    }

    tokens
}
