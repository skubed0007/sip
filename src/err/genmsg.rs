use super::{defs::ErrT, pt2::pt2};
use colored::*;

pub const BRIGHT_RED: Color = Color::TrueColor {
    r: 255,
    g: 85,
    b: 60,
};
pub const BRIGHT_YELLOW: Color = Color::TrueColor {
    r: 255,
    g: 230,
    b: 0,
};
pub const BRIGHT_WHITE: Color = Color::TrueColor {
    r: 255,
    g: 255,
    b: 255,
};
pub const BRIGHT_CYAN: Color = Color::TrueColor {
    r: 0,
    g: 255,
    b: 255,
};

pub const FOOTER_LINE: &str = "╰─────────────────────────────────────────────";

pub fn genmsg(err: ErrT, code: &String) -> String {
    let build_pointer_line = |code_line: &str, column_start: usize, column_end: usize| -> String {
        let mut pointer_line = String::new();
        for i in 0..code_line.len() {
            if i >= column_start && i < column_end {
                pointer_line.push('^');
            } else {
                pointer_line.push(' ');
            }
        }
        pointer_line
    };

    match err {
        ErrT::MissingReturnType {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start-5, column_end-5);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[SyntaxError] Missing return type".color(BRIGHT_RED).bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "  {}",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation =
                "In Sip, all functions must declare a return type explicitly using `@`. This ensures the type of value returned is always known at compile time. A missing return type can lead to ambiguity and unexpected behavior.";
            let help_msg = "After the function's parameter list, add `@` followed by the return type. Example: `fun main() @ i32 { ... }` or `@ (bool, i64)` for multiple return values.";

            format!(
                "{header} {title}\n\
        {arrow} {location}\n\
        {bar} {code_line_colored}\n\
        {bar} {pointer_colored}\n\
        {arrow} Explanation:\n\
        {bar} {}\n\
        {arrow} Help:\n\
        {bar} {}\n\
        {footer}",
                explanation.color(BRIGHT_WHITE),
                help_msg.color(BRIGHT_CYAN).italic()
            )
        }

        ErrT::MissFnName {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start-5, column_end-5);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[SyntaxError] Missing function name".color(BRIGHT_RED).bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "  {}",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation =
                "Every function declaration must have a valid name after the `fun` keyword. The name is used to identify and call the function later in the program.";
            let help_msg = "Write a valid identifier right after `fun`. For example: `fun computeSum()` or `fun start()`.";

            format!(
                "{header} {title}\n\
{arrow} {location}\n\
{bar} {code_line_colored}\n\
{bar} {pointer_colored}\n\
{arrow} Explanation:\n\
{bar} {}\n\
{arrow} Help:\n\
{bar} {}\n\
{footer}",
                explanation.color(BRIGHT_WHITE),
                help_msg.color(BRIGHT_CYAN).italic()
            )
        }

        ErrT::SyntaxError {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start-5, column_end-5);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[SyntaxError] Unexpected token".color(BRIGHT_RED).bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "  {}",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation = "The parser encountered a token that does not belong at this position in the code. This usually means there’s an extra symbol, a missing keyword, or a typo.";
            let help_msg = "Check your code for misplaced punctuation, invalid characters, or missing structure. Refer to the documentation for the correct syntax in this context.";

            format!(
                "{header} {title}\n\
{arrow} {location}\n\
{bar} {code_line_colored}\n\
{bar} {pointer_colored}\n\
{arrow} Explanation:\n\
{bar} {}\n\
{arrow} Help:\n\
{bar} {}\n\
{footer}",
                explanation.color(BRIGHT_WHITE),
                help_msg.color(BRIGHT_CYAN).italic()
            )
        }

        ErrT::InvVarT {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start-5, column_end-5);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[SyntaxError] Invalid variable type".color(BRIGHT_RED).bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "  {}",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation = "The type specified for this variable is unknown or invalid. All variable types must be defined and recognized by the language.";
            let help_msg = "Use a valid, defined type such as `i32`, `f64`, `bool`, or a user-defined struct. Check for typos or ensure the type is declared earlier in your code.";

            format!(
                "{header} {title}\n\
{arrow} {location}\n\
{bar} {code_line_colored}\n\
{bar} {pointer_colored}\n\
{arrow} Explanation:\n\
{bar} {}\n\
{arrow} Help:\n\
{bar} {}\n\
{footer}",
                explanation.color(BRIGHT_WHITE),
                help_msg.color(BRIGHT_CYAN).italic()
            )
        }

        ErrT::ExpectedArgStart {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start-5, column_end-5);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[SyntaxError] Expected '(' after function name".color(BRIGHT_RED).bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "  {}",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation = "Function declarations must begin their parameter list with an opening parenthesis `(` after the name. This indicates the start of argument definitions.";
            let help_msg = "Immediately after naming the function, add an opening parenthesis to start the argument list. Example: `fun doStuff()` or `fun add(a: i32, b: i32)`.";

            format!(
                "{header} {title}\n\
        {arrow} {location}\n\
        {bar} {code_line_colored}\n\
        {bar} {pointer_colored}\n\
        {arrow} Explanation:\n\
        {bar} {}\n\
        {arrow} Help:\n\
        {bar} {}\n\
        {footer}",
                explanation.color(BRIGHT_WHITE),
                help_msg.color(BRIGHT_CYAN).italic()
            )
        }
        _ => pt2(err, code),
    }
}
