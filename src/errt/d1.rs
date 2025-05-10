use crate::errt::t::{ErrT, SErr};
use colored::*;

#[allow(non_snake_case)]
pub fn DErr(err: &SErr, src: &str) {
    let (kind, detail, help, note) = match &err.t {
        ErrT::ExpectName => (
            "expected identifier",
            "Parser expected a variable name or symbol here.",
            Some("use a valid name: letters, numbers, underscores"),
            Some("names cannot start with digits"),
        ),
        ErrT::UnexpectTok => (
            "unexpected token",
            "Found an unexpected token in this context.",
            Some("check for misplaced operators or keywords"),
            Some("try removing or replacing the highlighted token"),
        ),
        ErrT::MissingSemicolon => (
            "missing semicolon",
            "Statements must end with a semicolon (`;`).",
            Some("add `;` at the end of the statement"),
            Some("every declaration needs a semicolon"),
        ),
        ErrT::UnterminatedLiteral => (
            "unterminated literal",
            "String or character literal wasn't properly closed.",
            Some("add a closing quote (`\"`) or apostrophe (`'`)"),
            Some("escape inner quotes with `\\\"`"),
        ),
        ErrT::InvalidNumber => (
            "invalid number",
            "Malformed or out-of-range numeric literal.",
            Some("check digits, radix prefixes, or type suffixes"),
            Some("like `0xFF`, `42u8`, or `1.5f32`"),
        ),
        ErrT::UnknownKeyword => (
            "unknown keyword",
            "Unrecognized keyword or directive.",
            Some("check spelling or refer to documentation"),
            Some("valid: `extern`, `link`, `fn`, `let`, ..."),
        ),
        ErrT::UnmatchedParen => (
            "unmatched parenthesis",
            "Mismatched or unclosed parentheses.",
            Some("ensure every `(` has a closing `)`"),
            Some("use an editor with bracket highlighting"),
        ),
        ErrT::UnmatchedBrace => (
            "unmatched brace",
            "Mismatched or unclosed `{}` blocks.",
            Some("ensure every `{` has a closing `}`"),
            Some("format code with rustfmt or similar tools"),
        ),
        ErrT::UnmatchedBracket => (
            "unmatched bracket",
            "Mismatched or unclosed square brackets.",
            Some("ensure every `[` has a matching `]`"),
            Some("common in array literals or indexing"),
        ),
        ErrT::ExpectOperator => (
            "expected operator",
            "Expected an arithmetic or logical operator.",
            Some("insert an operator like `+`, `*`, `==`, etc."),
            Some("valid: `+ - * / % == != < > <= >=`"),
        ),
        ErrT::InvalidArgToken => (
            "invalid argument token",
            "Invalid token in function argument list.",
            Some("check for misplaced commas or operators"),
            Some("ensure correct syntax for function calls"),
        ),
        ErrT::ExpectExpression => (
            "expected expression",
            "Expected a value, literal, or computation.",
            Some("provide a valid expression or value"),
            Some("could be a literal, var name, or call"),
        ),
        ErrT::Generic(msg) => (
            "error",
            msg.as_str(),
            None,
            None,
        ),
        ErrT::TypeMismatch => (
            "type mismatch",
            "Types of operands do not match.",
            Some("check types of variables and literals"),
            Some("ensure correct type conversions"),
        ),
        ErrT::UnexpectedEof => (
            "unexpected end of file",
            "Unexpected end of file.",
            None,
            None,
        ),
    };

    let line_number = err.line.max(1);
    let line_str = src.lines().nth((line_number - 1) as usize).unwrap_or("");
    
    let (start, end) = match &err.t {
        ErrT::MissingSemicolon => {
            let pos = line_str.trim_end().len();
            (pos, pos + 1)
        },
        _ => (err.start as usize, err.end as usize)
    };

    // Error header
    println!("\nerror[E{:04}]: {}", 
        line_number, 
        kind.bright_red().bold()
    );

    // Location bar
    println!("  --> {}:{}:{}", 
        err.file.bright_blue(),
        line_number,
        start + 1
    );

    // Code context frame
    println!("   ╭─[{}]", "source".bright_black());
    println!("{} │ {}", 
        format!("{:>2}", line_number).bright_black(),
        line_str
    );

    // Error indicator
    println!("   │ {}{}",
        " ".repeat(start),
        "^".repeat(end - start).bright_red().bold()
    );
    println!("   │ {}{}",
        " ".repeat(start),
        detail.bright_red()
    );

    // Help section if available
    if let Some(h) = help {
        println!("   │");
        println!("   ╰─[{}] {}", 
            "help".green().bold(),
            h.bright_white()
        );
    }

    // Note section if available
    if let Some(n) = note {
        println!("      {} {}", 
            "note:".cyan().bold(),
            n.bright_white()
        );
    }
    println!();
}
