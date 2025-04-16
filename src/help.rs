use colored::*;

pub fn help() {
    println!("{}", "╭─[Sip Programming Language]".bold().bright_magenta());
    println!("{}", "│   ├─ build   - Build a Sip source file/folder".green());
    println!("{}", "│   └─ help    - Display this help message".green());
    println!("{}", "│─ [USAGE]".bold().cyan());
    println!("{}", "│   ├─ sip <command> <file/folder(not supported yet)> <options>".yellow());
    println!("{}", "│─ [EXAMPLES]".bold().cyan());
    println!("{}", "│   ├─ sip build my_program.sip".green());
    println!("{}", "│   └─ sip run my_program.sip".green());
    println!("{}", "╰────────────────────────────────────────────────────".blue().bold());
}
