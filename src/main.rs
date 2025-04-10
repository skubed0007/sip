use std::env::args;

use dbgs::tok::print_tokens;
use token::defs::Lexer;


//mods go here
pub mod token;
pub mod dbgs;
pub mod parser;


fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let code = {
        match std::fs::read_to_string(input_file) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error reading file {}: {}", input_file, e);
                std::process::exit(1);
            }
        }
    };
    let mut lexer = Lexer::new(&code);
    lexer.lex();
    let tokens = lexer.get_tokens();
    print_tokens(&tokens);
    //tokenize
}
