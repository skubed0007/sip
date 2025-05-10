use colored::Colorize;
use errt::d1::DErr;
use lexer::lex;
use parsers::parse;
use std::{env, fs::File, process};
use memmap2::Mmap;

pub mod ast;
pub mod errt;
pub mod lexer;
pub mod parsers;
pub mod codegen;
pub mod tokdefs;

fn main() {
    if enable_ansi_support::enable_ansi_support().is_err() {
        eprintln!(
            "{} {}\n{}",
            "╭─".bright_black(),
            "Failed to enable ANSI support.".bold().red(),
            "╰─ Ensure your terminal supports ANSI escape codes.".bright_black()
        );
        process::exit(1);
    }
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!(
            "{} {}\n{}",
            "╭─".bright_black(),
            "No input file provided.".bold().red(),
            "╰─ Usage: sip <source-file>".bright_black()
        );
        process::exit(1);
    }

    let path = &args[1];
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} {}\n{}",
                "╭─".bright_black(),
                format!("Failed to open file `{}`: {}", path, e).bold().red(),
                "╰─ Ensure the path is correct and the file is accessible.".bright_black()
            );
            process::exit(1);
        }
    };

    let mmap = unsafe {
        match Mmap::map(&file) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "{} {}\n{}",
                    "╭─".bright_black(),
                    format!("Failed to memory-map file: {}", e).bold().red(),
                    "╰─ Try using a smaller file or check permissions.".bright_black()
                );
                process::exit(1);
            }
        }
    };

    let code = &mmap[..];

    let tokens = lex(code);
    //println!("tokens:\n{:#?}", &tokens);

    match parse(tokens,path) {
        Ok(ast) => {
            println!("ast:\n{:#?}", ast);
        }
        Err(es) => {
            if !es.is_empty() {
                println!(
                    "{}\n{}",
                    "Parsing failed: errors occurred during compilation."
                        .bold()
                        .red(),
                    "The following issues were found:".bright_black()
                );
                for err in es {
                    let code_str = match std::str::from_utf8(code) {
                        Ok(s) => s.to_string(),
                        Err(e) => {
                            eprintln!(
                                "{} {}\n{}",
                                "╭─".bright_black(),
                                format!("File is not valid UTF-8: {}", e).bold().red(),
                                "╰─ Only UTF-8 encoded source files are supported.".bright_black()
                            );
                            process::exit(1);
                        }
                    };

                    DErr(&err, &code_str);
                    println!();
                }
            }
        }
    }
}
