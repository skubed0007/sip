use crate::err::def::{ErrT, PErr};
use memmap2::Mmap;
use colored::*;

pub fn diserr(e: &PErr, mmap: &Mmap) -> String {
    let full_src = std::str::from_utf8(&mmap[..]).unwrap_or("<invalid utf8>");
    let line = e.line;
    let src_line = {
        match e.et {
            ErrT::RetTNF | ErrT::RetTMisMatch(_) => {
                full_src.lines().nth(line.saturating_sub(2)).unwrap_or("<unknown line>")
            },
            _ => {
                full_src.lines().nth(line.saturating_sub(1)).unwrap_or("<unknown line>")
            }
        }
    };
    let full_line_len = src_line.chars().count();
    let full_underline = "─".repeat(full_line_len);
    let indicator_len = if e.end > e.start { e.end - e.start } else { 1 };
    let indicator = format!("{}{}", " ".repeat(e.start), "^".repeat(indicator_len));

    // Error type descriptions with bold colors
    let err_desc = match &e.et {
        ErrT::FuncNameNotFound   => "Function name not found",
        ErrT::SyntaxErr(desc)    => desc,
        ErrT::ExpectParamName    => "Expected parameter name",
        ErrT::ExpectedAT         => "Expected '@' before return type",
        ErrT::RetTMisMatch(desc) => desc,
        ErrT::RetTNF             => "Return value not found",
        ErrT::InvVT              => "Invalid variable type",
    };

    // Help suggestions based on the error type
    let help_msg = match &e.et {
        ErrT::FuncNameNotFound   => "Provide a valid function name.",
        ErrT::SyntaxErr(_)       => "",
        ErrT::ExpectParamName    => "Each parameter must have an identifier.",
        ErrT::ExpectedAT         => "Ensure exactly one '@' appears after the parameter list.",
        ErrT::RetTMisMatch(_)    => "Verify that the return type and value match.",
        ErrT::RetTNF             => "Insert a return statement with an appropriate value.",
        ErrT::InvVT              => "Check the declared type for correctness.",
    };

    // Structure for visual formatting with bold colors
    let header = format!("╭── [ERROR] {}", err_desc.red().bold());
    let location = format!("│   ├─ [LOCATION] Line: {} | Column: {}", line, e.start).yellow().bold();
    let source_header = "│   ├─ [SOURCE]".to_string().cyan().bold();
    let source_line = format!("│   │   {}", src_line).white().bold();
    let underline_line = format!("│   │   {}", full_underline.red().bold());
    let indicator_line = format!("│   │   {}", indicator.red().bold());
    let explanation = format!("│   ├─ [EXPLANATION] {}", err_desc.magenta().bold());
    let help_line = if !help_msg.is_empty() {
        format!("│   └─ [HELP] {}", help_msg.green().italic().bold())
    } else {
        "│   └─ [HELP] Unable to give you help for this".to_string().dimmed().to_string()
    };

    // Closing border with rounded corners, bold
    let footer = "╰────────────────────────────────────────────".green().bold();

    // Final output assembly
    format!(
r#"
{} 
{}
{}
{} 
{}
{} 
{}
{}
{}
"#,
        header,
        location,
        source_header,
        source_line,
        underline_line,
        indicator_line,
        explanation,
        help_line,
        footer
    )
}
