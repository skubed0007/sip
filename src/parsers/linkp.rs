use crate::{
    ast::{CLINKT, ID_CAP, NT},
    errt::t::{ErrT, SErr},
    lexer::lex,
    parsers::parse,
    tokdefs::{IDT, TT, Tok},
};
use memmap2::Mmap;
use smallstr::SmallString;
use std::{fs::File, path::Path};

pub fn linkp(
    tokiter: &mut std::iter::Peekable<std::vec::IntoIter<Tok>>,
    filename: &String,
    ast: &mut Vec<NT>,
) -> Result<NT, Vec<SErr>> {
    let mut errs: Vec<SErr> = Vec::new();
    let mut libs: Vec<(SmallString<[u8; ID_CAP]>, CLINKT)> = Vec::new();
    let mut saw_name = false;
    let mut last_tok: Option<Tok> = None;

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
                        file: filename.clone(),
                    });
                    return Err(errs);
                }
                if !errs.is_empty() {
                    return Err(errs);
                }
                return Ok(NT::Link(libs));
            }

            TT::IDENT(IDT::DQ, ref name) => {
                saw_name = true;
                let name_str = name.as_str();

                let is_path = name_str.starts_with('/')
                    || name_str.starts_with("./")
                    || name_str.starts_with("../");

                if is_path {
                    let path = Path::new(name_str);

                    if !path.exists() {
                        eprintln!("error: clink path not found: {}", name_str);
                        errs.push(SErr {
                            t: ErrT::IOErr(format!("File does not exist: {}", name_str)),
                            line: tok.line,
                            start: tok.start,
                            end: tok.end,
                            file: filename.clone(),
                        });
                        continue;
                    }

                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ext == "sip" {
                            match File::open(path) {
                                Ok(file) => match unsafe { Mmap::map(&file) } {
                                    Ok(mmap) => {
                                        let tokens = lex(&mmap);
                                        match parse(tokens, filename) {
                                            Ok(parsed_ast) => ast.extend(parsed_ast),
                                            Err(mut perr) => errs.append(&mut perr),
                                        }
                                    }
                                    Err(e) => errs.push(SErr {
                                        t: ErrT::IOErr(format!(
                                            "Failed to memory map {}: {}",
                                            name_str, e
                                        )),
                                        line: tok.line,
                                        start: tok.start,
                                        end: tok.end,
                                        file: filename.clone(),
                                    }),
                                },
                                Err(e) => errs.push(SErr {
                                    t: ErrT::IOErr(format!("Cannot open {}: {}", name_str, e)),
                                    line: tok.line,
                                    start: tok.start,
                                    end: tok.end,
                                    file: filename.clone(),
                                }),
                            }
                            // For .sip files: do NOT push into libs — just parse and add AST
                            continue;
                        }
                    }

                    // For other extensions or no extension, push path lib
                    libs.push((SmallString::from(name_str), CLINKT::PATH));
                } else {
                    libs.push((SmallString::from(name_str), CLINKT::LIB));
                }
            }

            _ => {
                errs.push(SErr {
                    t: ErrT::UnexpectTok,
                    line: tok.line,
                    start: tok.start,
                    end: tok.end,
                    file: filename.clone(),
                });
            }
        }
    }

    if let Some(tok) = last_tok {
        errs.push(SErr {
            t: ErrT::MissingSemicolon,
            line: tok.line,
            start: tok.start,
            end: tok.end,
            file: filename.clone(),
        });
    } else {
        errs.push(SErr {
            t: ErrT::MissingSemicolon,
            line: 0,
            start: 0,
            end: 0,
            file: filename.clone(),
        });
    }

    Err(errs)
}
