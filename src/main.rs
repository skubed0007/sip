use std::env;
use std::fs::File;
use std::process::exit;
use std::time::Instant;

use memmap2::Mmap;
use help::help;
use parser::Parser;
use token::lexer::lex;

pub mod help;
pub mod token;
pub mod ast;
pub mod parser;
pub mod err;

fn main() {
    let mut args = env::args();
    let _bin = args.next();
    let command = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing command.\n");
        help();
        exit(1);
    });
    let filepath = args.next().unwrap_or_else(|| {
        eprintln!("Error: Missing file path.\n");
        help();
        exit(1);
    });

    if command == "help" {
        help();
        return;
    }

    match command.as_str() {
        "build" => {
            // Record the time taken to open the file
            let start_open = Instant::now();
            let file = match File::open(&filepath) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error opening file!\nerr => {:?}", e);
                    exit(1);
                }
            };
            let open_duration = start_open.elapsed();

            // Record the time taken to memory-map the file
            let start_map = Instant::now();
            let mmap = unsafe {
                match Mmap::map(&file) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("error memory-mapping file!\nerr => {:?}", e);
                        exit(1);
                    }
                }
            };
            let map_duration = start_map.elapsed();

            // Record the time taken to lex the file contents
            let start_lex = Instant::now();
            let tokens = lex(&mmap[..]); // pass raw &[u8] directly
            let lex_duration = start_lex.elapsed();

            // Print the times in a nice format
            println!("File opened in: {:.2?}", open_duration);
            println!("File memory-mapped in: {:.2?}", map_duration);
            println!("File tokenized in: {:.2?}", lex_duration);
            //println!("{:?}", tokens);
            //println!("tokens:\n{:#?}",tokens);
            let mut parser = Parser::new(tokens);
            parser.parse();
            if parser.erroccur(){
                parser.show_errs(&mmap);
            }
            // Optionally, print the tokens for debugging
        }

        _ => {
            eprintln!("Unknown command: '{}'\n", command);
            help();
            exit(1);
        }
    }
}
