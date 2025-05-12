use crate::errt::t::{ErrT, SErr};
use colored::*;

#[allow(non_snake_case)]
pub fn DErr(err: &SErr, src: &str) {
    let (kind, detail, help, note) = match &err.t {
        ErrT::ExpectName => (
            "expected identifier",
            "Parser expected a variable name or symbol here.",
            Some("Use a valid name: letters, numbers, underscores"),
            Some("Identifiers cannot start with digits"),
        ),
        ErrT::UnexpectTok => (
            "unexpected token",
            "Found an unexpected token in this context.",
            Some("Check for misplaced operators or keywords"),
            Some("Try removing or replacing the highlighted token"),
        ),
        ErrT::MissingSemicolon => (
            "missing semicolon",
            "Statements must end with a semicolon (`;`).",
            Some("Add `;` at the end of the statement"),
            Some("Every declaration needs a semicolon"),
        ),
        ErrT::UnterminatedLiteral => (
            "unterminated literal",
            "String or character literal wasn't properly closed.",
            Some("Add a closing quote (`\"`) or apostrophe (`'`)"),
            Some("Escape inner quotes with `\\\"` or `\\'`"),
        ),
        ErrT::InvalidNumber => (
            "invalid number",
            "Malformed or out-of-range numeric literal.",
            Some("Check digits, radix prefixes, or type suffixes"),
            Some("Example: `0xFF`, `42u8`, or `1.5f32`"),
        ),
        ErrT::UnknownKeyword => (
            "unknown keyword",
            "Unrecognized keyword or directive.",
            Some("Check spelling or refer to documentation"),
            Some("Valid: `extern`, `clink`, `fn`, `let`, ..."),
        ),
        ErrT::UnmatchedParen => (
            "unmatched parenthesis",
            "Mismatched or unclosed parentheses.",
            Some("Ensure every `(` has a closing `)`"),
            Some("Use an editor with bracket highlighting"),
        ),
        ErrT::UnmatchedBrace => (
            "unmatched brace",
            "Mismatched or unclosed `{}` blocks.",
            Some("Ensure every `{` has a matching `}`"),
            Some("Use auto-format tools like rustfmt"),
        ),
        ErrT::UnmatchedBracket => (
            "unmatched bracket",
            "Mismatched or unclosed square brackets.",
            Some("Ensure every `[` has a matching `]`"),
            Some("Common in array literals or indexing"),
        ),
        ErrT::ExpectOperator => (
            "expected operator",
            "Expected an arithmetic or logical operator.",
            Some("Insert an operator like `+`, `*`, `==`, etc."),
            Some("Valid: `+ - * / % == != < > <= >=`"),
        ),
        ErrT::InvalidArgToken => (
            "invalid argument token",
            "Invalid token in function argument list.",
            Some("Check for misplaced commas or operators"),
            Some("Ensure correct syntax for function calls"),
        ),
        ErrT::ExpectExpression => (
            "expected expression",
            "Expected a value, literal, or computation.",
            Some("Provide a valid expression or value"),
            Some("Could be a literal, variable name, or call"),
        ),
        ErrT::Generic(msg) => ("error", msg.as_str(), None, None),
        ErrT::TypeMismatch => (
            "type mismatch",
            "Types of operands do not match.",
            Some("Check types of variables and literals"),
            Some("Ensure correct type conversions"),
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
        }
        _ => (err.start as usize, err.end as usize),
    };

    // Print error header like: error[E0001]: expected identifier
    println!(
        "\n{}[E{:04}]: {}",
        "error".bright_red().bold(),
        line_number,
        kind.bright_white().bold()
    );

    // --> src/main.sip:12:5
    println!(
        "{} {}:{}:{}",
        "-->".bright_blue(),
        err.file.bright_blue(),
        line_number,
        start + 1
    );

    // Code context
    println!(" {}", "│".bright_black());
    println!(
        " {} {:>4} {} {}",
        "│".bright_black(),
        line_number,
        "│".bright_black(),
        line_str
    );

    // Error underline and message
    let marker = "^".repeat((end - start).max(1));
    println!(
        " {}      {} {}{}",
        "│".bright_black(),
        " ".repeat(start),
        marker.bright_red().bold(),
        format!(" {}", detail).bright_red()
    );

    // Empty line before hints
    println!(" {}", "│".bright_black());

    // Help
    if let Some(h) = help {
        println!(
            "{} {} {}",
            "help:".green().bold(),
            " ".repeat(2),
            h.bright_white()
        );
    }

    // Note
    if let Some(n) = note {
        println!(
            "{} {} {}",
            "note:".cyan().bold(),
            " ".repeat(2),
            n.bright_white()
        );
    }

    println!(); // trailing newline
}
