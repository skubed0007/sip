use crate::token::defs::Token;

pub fn print_tokens(tokens: &Vec<Token>) {
    println!("╭─[Tokens]");
    for token in tokens {
        println!(
            "│ TokenType: {:?}, Lexeme: {:?}, Line: {}-{}, Column: {}-{}",
            token.token_type,
            token.lexeme,
            token.line_start.unwrap_or(0),
            token.line_end.unwrap_or(0),
            token.column_start.unwrap_or(0),
            token.column_end.unwrap_or(0)
        );
    }
    println!("╰───────────────────────────────────────────────────");
}