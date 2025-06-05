use crate::errt::t::{ErrT, SErr};
use colored::*;

#[allow(non_snake_case)]
pub fn DErr(err: &SErr, src: &str) {
    let io_err_msg;

    let (kind, detail, help, note) = match &err.t {
        ErrT::ExpectName => (
            "expected identifier",
            "Parser expected a variable name or symbol here.",
            Some("Use a valid name: letters, numbers, and underscores."),
            Some("Identifiers cannot start with digits."),
        ),
        ErrT::UnexpectTok => (
            "unexpected token",
            "Found an unexpected token in this context.",
            Some("Check for misplaced operators or keywords."),
            Some("Try removing or replacing the highlighted token."),
        ),
        ErrT::MissingSemicolon => (
            "missing semicolon",
            "Statements must end with a semicolon (`;`).",
            Some("Add a semicolon at the end of the statement."),
            Some("Every declaration requires a semicolon."),
        ),
        ErrT::UnterminatedLiteral => (
            "unterminated literal",
            "String or character literal was not properly closed.",
            Some("Add a closing quote (`\"`) or apostrophe (`'`)."),
            Some("Escape inner quotes using `\\\"` or `\\'`."),
        ),
        ErrT::InvalidNumber => (
            "invalid number",
            "Malformed or out-of-range numeric literal.",
            Some("Verify digits, radix prefixes, or type suffixes."),
            Some("Examples: `0xFF`, `42u8`, `1.5f32`."),
        ),
        ErrT::UnknownKeyword => (
            "unknown keyword",
            "Unrecognized keyword or directive.",
            Some("Check spelling or consult documentation."),
            Some("Valid keywords include `extern`, `clink`, `fn`, `let`, etc."),
        ),
        ErrT::UnmatchedParen => (
            "unmatched parenthesis",
            "Mismatched or unclosed parentheses.",
            Some("Ensure every `(` has a matching `)`."),
            Some("Use an editor with bracket highlighting."),
        ),
        ErrT::UnmatchedBrace => (
            "unmatched brace",
            "Mismatched or unclosed braces `{}`.",
            Some("Ensure every `{` has a matching `}`."),
            Some("Use tools like `rustfmt` for formatting."),
        ),
        ErrT::UnmatchedBracket => (
            "unmatched bracket",
            "Mismatched or unclosed square brackets.",
            Some("Ensure every `[` has a matching `]`."),
            Some("Common in array literals or indexing."),
        ),
        ErrT::ExpectOperator => (
            "expected operator",
            "Expected an arithmetic or logical operator here.",
            Some("Insert an operator like `+`, `*`, `==`, etc."),
            Some("Valid operators: `+ - * / % == != < > <= >=`."),
        ),
        ErrT::InvalidArgToken => (
            "invalid argument token",
            "Invalid token in function argument list.",
            Some("Check for misplaced commas or operators."),
            Some("Ensure correct function call syntax."),
        ),
        ErrT::ExpectExpression => (
            "expected expression",
            "Expected a value, literal, or computation.",
            Some("Provide a valid expression or value."),
            Some("Could be a literal, variable name, or function call."),
        ),
        ErrT::Generic(msg) => ("error", msg.as_str(), None, None),
        ErrT::TypeMismatch => (
            "type mismatch",
            "Operand types do not match.",
            Some("Verify variable and literal types."),
            Some("Ensure proper type conversions."),
        ),
        ErrT::UnexpectedEof => (
            "unexpected end of file",
            "File ended unexpectedly.",
            None,
            None,
        ),
        ErrT::IOErr(file) => {
            io_err_msg = format!("Unable to access file: {}", file);
            (
                "file access error",
                io_err_msg.as_str(),
                Some("Ensure the file exists and is readable."),
                Some("Check the file path, permissions, and spelling."),
            )
        }
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

    println!(
        "\n{}[E{:04}]: {}",
        "error".bright_red().bold(),
        line_number,
        kind.bright_white().bold()
    );

    println!(
        "{} {}:{}:{}",
        "-->".bright_blue(),
        err.file.bright_blue(),
        line_number,
        start + 1
    );

    println!(" {}", "│".bright_black());
    println!(
        " {} {:>4} {} {}",
        "│".bright_black(),
        line_number,
        "│".bright_black(),
        line_str
    );

    let marker = "^".repeat((end - start).max(1));
    println!(
        " {}      {} {}{}",
        "│".bright_black(),
        " ".repeat(start),
        marker.bright_red().bold(),
        format!(" {}", detail).bright_red()
    );

    println!(" {}", "│".bright_black());

    if let Some(h) = help {
        println!(
            "{} {} {}",
            "help:".green().bold(),
            "  ",
            h.bright_white()
        );
    }

    if let Some(n) = note {
        println!(
            "{} {} {}",
            "note:".cyan().bold(),
            "  ",
            n.bright_white()
        );
    }

    println!();
}
