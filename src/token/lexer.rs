use crate::token::defs::{Token, TT, IDType};

#[allow(unsafe_op_in_unsafe_fn)]
#[inline(always)]
unsafe fn bytes_to_str_unchecked(bytes: &[u8]) -> &str {
    std::str::from_utf8_unchecked(bytes)
}

#[inline(always)]
fn is_alphanumeric(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}

#[inline(always)]
fn keyword_tt(s: &str) -> TT {
    match s {
        "fn" => TT::FN,
        "ret" => TT::Ret,
        "const" => TT::CONST,
        "global" => TT::GLOBAL,
        "i32" => TT::I32,
        "i64" => TT::I64,
        "u8" => TT::U8,
        "f32" => TT::F32,
        "f64" => TT::F64,
        "nil" => TT::Nil,
        _ => TT::Ident(IDType::Nil),
    }
}

#[inline(always)]
pub fn lex(code: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::with_capacity(code.len() / 2); // heuristic pre-alloc
    let mut index = 0;
    let mut line = 1;
    let mut start;

    while index < code.len() {
        let c = code[index];

        match c {
            b'\n' => { line += 1; index += 1; continue; }
            b' ' | b'\r' | b'\t' => { index += 1; continue; }

            // 1-byte tokens
            b'(' => { tokens.push(Token::new(TT::LSmallB, line, index, index + 1)); index += 1; continue; }
            b')' => { tokens.push(Token::new(TT::RsmallB, line, index, index + 1)); index += 1; continue; }
            b'{' => { tokens.push(Token::new(TT::LCurlyB, line, index, index + 1)); index += 1; continue; }
            b'}' => { tokens.push(Token::new(TT::RCurlyB, line, index, index + 1)); index += 1; continue; }
            b'[' => { tokens.push(Token::new(TT::LBigB, line, index, index + 1)); index += 1; continue; }
            b']' => { tokens.push(Token::new(TT::RBigB, line, index, index + 1)); index += 1; continue; }
            b',' => { tokens.push(Token::new(TT::Comma, line, index, index + 1)); index += 1; continue; }
            b';' => { tokens.push(Token::new(TT::SemiColon, line, index, index + 1)); index += 1; continue; }
            b'=' => { tokens.push(Token::new(TT::Eqs, line, index, index + 1)); index += 1; continue; }
            b'@' => { tokens.push(Token::new(TT::At, line, index, index + 1)); index += 1; continue; }
            b'_' => { tokens.push(Token::new(TT::Underscore, line, index, index + 1)); index += 1; continue; }

            b'\'' | b'"' => {
                let quote = c;
                start = index;
                index += 1;
                while index < code.len() && code[index] != quote {
                    index += 1;
                }
                index += 1; // skip closing quote
                let slice = &code[start..index];
                let id_type = if quote == b'\'' { IDType::SQ } else { IDType::DQ };
                tokens.push(Token {
                    TT: TT::Ident(id_type),
                    line,
                    start,
                    end: index,
                    value: Some(unsafe { bytes_to_str_unchecked(slice) }.to_string()),
                });
                continue;
            }

            c if c.is_ascii_digit() => {
                start = index;
                while index < code.len() && code[index].is_ascii_digit() {
                    index += 1;
                }
                let slice = &code[start..index];
                tokens.push(Token {
                    TT: TT::NUM,
                    line,
                    start,
                    end: index,
                    value: Some(unsafe { bytes_to_str_unchecked(slice) }.to_string()),
                });
                continue;
            }

            c if is_alphanumeric(c) => {
                start = index;
                while index < code.len() && is_alphanumeric(code[index]) {
                    index += 1;
                }
                let slice = &code[start..index];
                let s = unsafe { bytes_to_str_unchecked(slice) };
                let tt = keyword_tt(s);
                tokens.push(Token {
                    TT: tt,
                    line,
                    start,
                    end: index,
                    value: Some(s.to_string()),
                });
                continue;
            }

            _ => {
                // skip unknown character
                index += 1;
            }
        }
    }

    tokens
}

#[allow(non_snake_case)]
// Optional: a small helper for repeated token pushes
impl Token {
    #[inline(always)]
    pub fn new(TT: TT, line: usize, start: usize, end: usize) -> Self {
        Self { TT, line, start, end, value: None }
    }
}
