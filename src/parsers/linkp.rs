use smallstr::SmallString;

use crate::{
    ast::{ID_CAP, NT},
    errt::t::{ErrT, SErr},
    tokdefs::{IDT, TT, Tok},
};

/// Parses tokens to extract library names for linking.
/// 
/// This function processes a sequence of tokens and attempts to parse them into
/// a list of library names that are intended for linking. The tokens are expected
/// to follow a specific syntax where each library name is represented as a double-quoted
/// identifier, and the list is terminated by a semicolon.
/// 
/// # Arguments
/// 
/// * `tokiter` - A mutable iterator over tokens, used for parsing.
/// * `filename` - The name of the file being parsed, used for error reporting.
/// 
/// # Returns
/// 
/// Returns a `Result` containing an `NT::Link` variant with the list of library names
/// if successful, or a vector of `SErr` if any parsing errors occur.
/// 
/// # Errors
/// 
/// Errors are returned if no library names are found before the semicolon, or if 
/// unexpected tokens are encountered. Additionally, it reports an error if the 
/// end of the file is reached without encountering a semicolon.

pub fn linkp(tokiter: &mut std::iter::Peekable<std::vec::IntoIter<Tok>>,filename: &String) -> Result<NT, Vec<SErr>> {
    let mut errs: Vec<SErr> = Vec::new();
    let mut libs: Vec<SmallString<[u8; ID_CAP]>> = Vec::new();
    let mut saw_name = false;
    let mut last_tok: Option<Tok> = None;
    #[allow(clippy::while_let_on_iterator)]
    while let Some(tok) = tokiter.next() {
        last_tok = Some(tok.clone());

        match tok.tt {
            TT::SemiC => {
                if !saw_name {
                    errs.push(SErr {
                        t: ErrT::ExpectName,
                        line: tok.line,
                        start: tok.start,
                        end: tok.end,
                        file: filename.to_string(),
                    });
                    return Err(errs);
                }
                return Ok(NT::Link(libs));
            }

            TT::IDENT(IDT::DQ, ref name) => {
                saw_name = true;
                libs.push(SmallString::from(name.as_str()));
            }

            _ => {
                errs.push(SErr {
                    t: ErrT::UnexpectTok,
                    line: tok.line,
                    start: tok.start,
                    end: tok.end,
                    file: filename.to_string(),
                });
            }
        }
    }

    // EOF reached before `;`. Use last_tok’s position for the error.
    if let Some(tok) = last_tok {
        errs.push(SErr {
            t: ErrT::MissingSemicolon,
            line: tok.line,
            start: tok.start,
            end: tok.end,
            file: filename.to_string(),
        });
    } else {
        // No tokens at all after `clink`
        errs.push(SErr {
            t: ErrT::MissingSemicolon,
            line: 0,
            start: 0,
            end: 0,
            file: filename.to_string(),
        });
    }

    Err(errs)
}
