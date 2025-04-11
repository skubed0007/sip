use crate::err::genmsg::*;

use super::defs::ErrT;
use colored::*;
pub fn pt2(err: ErrT, code: &String) -> String {
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
        ErrT::TupleNil {
            line,
            column_start,
            column_end,
        } => {
            let lines: Vec<&str> = code.lines().collect();
            let code_line = lines.get(line).unwrap_or(&"");
            let pointer_line = build_pointer_line(code_line, column_start, column_end);

            let header = "╭─✦".color(BRIGHT_RED).bold();
            let arrow = "├──".color(BRIGHT_YELLOW);
            let bar = "│".color(BRIGHT_WHITE);
            let footer = FOOTER_LINE.color(BRIGHT_YELLOW);

            let title = "[TypeError] Invalid use of `nil` in tuple"
                .color(BRIGHT_RED)
                .bold();
            let location = format!(
                "at line {} , col {}..{}",
                (line + 1).to_string().bold().color(BRIGHT_RED),
                (column_start + 1).to_string().color(BRIGHT_CYAN),
                column_end.to_string().color(BRIGHT_CYAN)
            );
            let code_line_colored = code_line.color(BRIGHT_WHITE);
            let pointer_colored = format!(
                "{}{}",
                "  ",
                pointer_line.replace("^", &"^".color(BRIGHT_RED).bold().to_string())
            );

            let explanation = "The `nil` type represents an absence of value and cannot be used as a component inside a tuple. Tuples must consist of valid, concrete types so they can carry actual values at runtime.";
            let help_msg = "Remove `nil` from the tuple and replace it with a valid type. If you want to represent an optional value, consider using a union or custom enum instead.";

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
        _ => String::new(),
    }
}
