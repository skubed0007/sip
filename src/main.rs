use std::env;
use std::fs::{self, File, remove_file};
use std::io::{self, stdout, BufReader, Read, Write};
use std::process::{Command, exit};
use codegen::codegen::cgen;
use colored::*;
use errt::d1::DErr;
use help::print_help;
use lexer::lex;
use parsers::parse;

pub mod parsers;
pub mod codegen;
pub mod lexer;
pub mod tokdefs;
pub mod errt;
pub mod ast;
pub mod help;

#[derive(Debug)]
pub struct Options {
    pub opt: Option<i32>,
    pub debug: bool,
    pub verbose: bool,
    pub help: bool,
    pub all_warnings: bool,
    pub warnings_as_errors: bool,
    pub static_link: bool,
    pub pic: bool,
    pub define_macro: Option<String>,
    pub include_dir: Option<String>,
    pub library_search_path: Option<String>,
    pub library: Option<String>,
    pub target: Option<String>,
    pub cpu: Option<String>,
    pub arch: Option<String>,
    pub sanitize: Option<String>,
    pub lto: Option<String>,
    pub cache_dir: Option<String>,
    pub test: bool,
    pub trans_zig: bool,
    pub out: Option<String>,
    pub retain_c: bool,
}

fn parse_args(args: Vec<String>) -> (Options, String, String) {
    if args.len() < 3 {
        print_help();
        eprintln!("{}", "Usage: sip <command> <source_file> [options]".red());
        exit(1);
    }

    let mut options = Options {
        opt: None,
        debug: false,
        verbose: false,
        help: false,
        all_warnings: false,
        warnings_as_errors: false,
        static_link: false,
        pic: false,
        define_macro: None,
        include_dir: None,
        library_search_path: None,
        library: None,
        target: None,
        cpu: None,
        arch: None,
        sanitize: None,
        lto: None,
        cache_dir: None,
        test: false,
        trans_zig: false,
        out: None,
        retain_c: false,
    };

    let command = args[1].clone();
    let source_file = args[2].clone();

    let mut iter = args.into_iter().skip(3);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-O" | "--opt" => {
                if let Some(val) = iter.next() {
                    options.opt = val.parse().ok();
                }
            }
            "-g" | "--debug" => options.debug = true,
            "-h" | "--help" => options.help = true,
            "-Wall" => options.all_warnings = true,
            "-Werror" => options.warnings_as_errors = true,
            "-static" => options.static_link = true,
            "-fPIC" => options.pic = true,
            "-D" => options.define_macro = iter.next(),
            "-I" => options.include_dir = iter.next(),
            "-L" => options.library_search_path = iter.next(),
            "-l" => options.library = iter.next(),
            "--target" => options.target = iter.next(),
            "--mcpu" => options.cpu = iter.next(),
            "--march" => options.arch = iter.next(),
            "--sanitize" => options.sanitize = iter.next(),
            "--lto" => options.lto = iter.next(),
            "--cache-dir" => options.cache_dir = iter.next(),
            "--test" => options.test = true,
            "--trans-zig" => options.trans_zig = true,
            "--out" => options.out = iter.next(),
            "--retainc" => options.retain_c = true,
            _ => {
                eprintln!("{}: {}", "Unknown option".red(), arg);
                exit(1);
            }
        }
    }

    (options, command, source_file)
}

fn check_target_exists(target: &str) -> bool {
    let output = Command::new("zig")
        .arg("targets")
        .output()
        .expect("Failed to get Zig targets");

    if !output.status.success() {
        eprintln!("{}: {}", "Failed to check targets".red(), String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    String::from_utf8_lossy(&output.stdout).contains(target)
}

fn build(options: &Options, source_file: &str) {
    print!("\r{}", "Reading source file...".cyan());
    stdout().flush().unwrap();

    let file = match File::open(source_file) {
        Ok(f) => {
            print!("\r{}", "[OK] Reading source file".green());
            stdout().flush().unwrap();
            f
        }
        Err(e) => {
            eprintln!("\n{}: {}", "Failed to open source file".red(), e);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    let mut code = Vec::new();

    if let Err(e) = reader.read_to_end(&mut code) {
        eprintln!("\n{}: {}", "Failed to read file into memory".red(), e);
        return;
    }

    print!("\r{}", "Tokenizing...".cyan());
    stdout().flush().unwrap();

    let tokens = lex(&code);

    print!("\r{}", "Parsing...".cyan());
    stdout().flush().unwrap();

    let ast = match parse(tokens, &source_file.to_string()) {
        Ok(ast) => {
            print!("\r{}", "[OK] Parsing completed".green());
            stdout().flush().unwrap();
            ast
        }
        Err(errs) => {
            eprintln!("\n{}: Parsing failed", "Error".red());
            for err in errs {
                DErr(&err, source_file);
            }
            exit(1);
        }
    };

    print!("\r{}", "Generating C code...".cyan());
    stdout().flush().unwrap();

    let ccode = cgen(&ast);
    let c_file_path = format!("{}.tmp.c", source_file);

    print!("\r{}", "Writing C code to file...".cyan());
    stdout().flush().unwrap();

    if let Err(e) = fs::write(&c_file_path, ccode) {
        eprintln!("\n{}: {}", "Failed to write C file".red(), e);
        return;
    }

    if let Some(target) = &options.target {
        if !check_target_exists(target) {
            eprintln!("{}: Target '{}' not found in Zig", "Error".red(), target);
            return;
        }
    }

    print!("\r{}", "Compiling with Zig...".cyan());
    stdout().flush().unwrap();

    let mut cmd = Command::new("zig");
    cmd.arg("cc");

    if let Some(opt) = options.opt {
        cmd.arg(format!("-O{}", opt));
    }
    if options.debug {
        cmd.arg("-g");
    }
    if options.all_warnings {
        cmd.arg("-Wall");
    }
    if options.warnings_as_errors {
        cmd.arg("-Werror");
    }
    if options.pic {
        cmd.arg("-fPIC");
    }
    if options.static_link {
        cmd.arg("-static");
    }
    if let Some(t) = &options.target {
        cmd.arg(format!("--target={}", t));
    }
    for m in &options.define_macro {
        cmd.arg(format!("-D{}", m));
    }
    for inc in &options.include_dir {
        cmd.arg(format!("-I{}", inc));
    }
    for lib_path in &options.library_search_path {
        cmd.arg(format!("-L{}", lib_path));
    }
    for lib in &options.library {
        cmd.arg(format!("-l{}", lib));
    }
    if let Some(cpu) = &options.cpu {
        cmd.arg(format!("--mcpu={}", cpu));
    }
    if let Some(arch) = &options.arch {
        cmd.arg(format!("--march={}", arch));
    }
    if let Some(sanitize) = &options.sanitize {
        cmd.arg(format!("--sanitize={}", sanitize));
    }
    if let Some(lto) = &options.lto {
        cmd.arg(format!("--lto={}", lto));
    }
    if let Some(cache_dir) = &options.cache_dir {
        cmd.arg(format!("--cache-dir={}", cache_dir));
    }
    if options.test {
        cmd.arg("--test");
    }
    if options.trans_zig {
        cmd.arg("--trans-zig");
    }

    cmd.arg(&c_file_path);

    if let Some(out) = &options.out {
        cmd.arg("-o").arg(out);
    }

    match cmd.status() {
        Ok(status) if status.success() => {
            print!("\r{}", "[OK] Build completed successfully!".green());
            println!();
        }
        Ok(status) => {
            eprintln!("\n{}: {}", "Build failed with status".red(), status);
        }
        Err(e) => {
            eprintln!("\n{}: {}", "Failed to execute Zig".red(), e);
        }
    }

    if !options.retain_c {
        let _ = remove_file(&c_file_path);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (options, command, source_file) = parse_args(args);

    if options.help {
        print_help();
        return;
    }

    match command.as_str() {
        "build" => build(&options, &source_file),
        "help" => print_help(),
        _ => {
            eprintln!("{}: {}", "Unknown command".red(), command);
            exit(1);
        }
    }
}
