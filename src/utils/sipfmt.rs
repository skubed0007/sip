use memmap2::Mmap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Format every file in `args`: if it's a file, format it;  
/// if it's a directory, recurse into it and format all `.sip` files found.
pub fn sipfmt(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        print_help();
        std::process::exit(1);
    }

    // Gather all .sip files to format
    let mut to_format = Vec::new();
    for arg in args {
        let path = PathBuf::from(&arg);
        if path.is_file() {
            to_format.push(path);
        } else if path.is_dir() {
            collect_sip_files(&path, &mut to_format)?;
        } else {
            eprintln!(
                "Warning: '{}' is neither file nor directory. Skipping.",
                arg
            );
        }
    }

    if to_format.is_empty() {
        eprintln!("Nothing to format! No .sip files found.");
        std::process::exit(1);
    }

    // Format each file
    for file_path in &to_format {
        format_file(&file_path)?;
    }

    println!("All done! Formatted {} file(s).", to_format.len());
    Ok(())
}

/// Recursively descend `dir` and push any `.sip` file path into `out`.
fn collect_sip_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_sip_files(&path, out)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("sip") {
            out.push(path);
        }
    }
    Ok(())
}

/// The core formatting logic for a single file.
fn format_file(input_path: &Path) -> io::Result<()> {
    println!("Formatting '{}'", input_path.display());

    let file = File::open(input_path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let source = std::str::from_utf8(&mmap)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Non-UTF8 input"))?;

    let mut formatted = String::with_capacity(source.len() + 128);
    for part in source.split(';') {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            formatted.push_str(trimmed);
            formatted.push_str(";\n");
        }
    }

    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(input_path)?;
    let mut writer = BufWriter::with_capacity(8192, output_file);
    writer.write_all(formatted.as_bytes())?;
    writer.flush()?;

    Ok(())
}

fn print_help() {
    println!(
        "\
Welcome to sipfmt — the 'so simple it's suspicious' Sip formatter!

USAGE:
    sipfmt <file_or_dir> [<file_or_dir>...]

This tool will overwrite each given `.sip` file—and will
recurse into any directories you pass to find more `.sip` files.

Perfect for obsessive semicolon whisperers and formatting goblins alike!
"
    );
}
