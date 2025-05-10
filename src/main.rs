use codegen::codegen::cgen;
use colored::Colorize;
use errt::d1::DErr;
use lexer::lex;
use memmap2::Mmap;
use parsers::parse;
use std::{env, fs::{File, self}, process};

pub mod ast;
pub mod codegen;
pub mod errt;
pub mod lexer;
pub mod parsers;
pub mod tokdefs;

fn main() {
    // Ensure ANSI support is enabled for colored output
    if enable_ansi_support::enable_ansi_support().is_err() {
        eprintln!(
            "{} {}\n{}",
            "╭─".bright_black(),
            "Failed to enable ANSI support.".bold().red(),
            "╰─ Ensure your terminal supports ANSI escape codes.".bright_black()
        );
        process::exit(1);
    }

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the source file is provided
    if args.len() < 2 {
        eprintln!(
            "{} {}\n{}",
            "╭─".bright_black(),
            "No input file provided.".bold().red(),
            "╰─ Usage: sip <source-file>".bright_black()
        );
        process::exit(1);
    }

    // Path to the source file
    let path = &args[1];

    // Attempt to open the source file
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} {}\n{}",
                "╭─".bright_black(),
                format!("Failed to open file `{}`: {}", path, e)
                    .bold()
                    .red(),
                "╰─ Ensure the path is correct and the file is accessible.".bright_black()
            );
            process::exit(1);
        }
    };

    // Memory-map the file for reading
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

    // Get the code from the memory-mapped file
    let code = &mmap[..];

    // Tokenize the code
    let tokens = lex(code);

    // Parse the tokens and generate AST
    match parse(tokens, path) {
        Ok(ast) => {
            println!("AST:\n{:#?}", ast);

            // Generate C code from the AST
            let c_code = cgen(&ast);

            // Define the output C file path
            let c_file_path = "generated_code.c";

            // Write the generated C code to the file
            match fs::write(c_file_path, &c_code) {
                Ok(_) => {
                    println!("Generated C code saved to `{}`.", c_file_path);

                    // Compile the C code using Zig
                    let status = process::Command::new("zig")
                        .arg("cc")
                        .arg(c_file_path)
                        .arg("-o")
                        .arg("a.out")
                        .status();

                    // Check if Zig compilation succeeded
                    match status {
                        Ok(st) if st.success() => {
                            println!("C code compiled successfully! Output: a.out");
                        }
                        _ => {
                            eprintln!(
                                "{} {}\n{}",
                                "╭─".bright_black(),
                                "Failed to compile C code with Zig.".bold().red(),
                                "╰─ Make sure Zig is installed and the C code is valid.".bright_black()
                            );
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} {}\n{}",
                        "╭─".bright_black(),
                        format!("Failed to write C code to `{}`: {}", c_file_path, e)
                            .bold()
                            .red(),
                        "╰─ Check if the file path is valid and writable.".bright_black()
                    );
                    process::exit(1);
                }
            }
        }
        Err(es) => {
            // Error handling for parsing
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
