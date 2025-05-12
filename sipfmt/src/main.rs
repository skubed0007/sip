use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_help();
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        println!("Oh no! File '{}' does not exist!", args[1]);
        std::process::exit(1);
    }

    println!("Hey, formatting file: {}", input_path.display());

    let _file = File::open(input_path)?;
    let source = std::fs::read_to_string(input_path)?;

    let formatted = source
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| format!("{}\n", s))
        .collect::<String>();

    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(input_path)?;

    output.write_all(formatted.as_bytes())?;

    println!("Woohoo! Successfully formatted {}", input_path.display());
    Ok(())
}

fn print_help() {
    let usage = format!("Usage: sipfmt <file.sip>\n");
    let info = format!(
        "Formats `.sip` files by placing each statement ending in `;` on a new line.\n\
         Note: The original file will be overwritten!\n"
    );
    println!("Hey there! Welcome to sipfmt - the tiny Sip formatter!\n{}\n{}", usage, info);
}

