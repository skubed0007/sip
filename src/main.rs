use codegen::codegen::cgen;
use colored::*;
use enable_ansi_support::enable_ansi_support;
use errt::d1::DErr;
use help::print_help;
use lexer::lex;
use parsers::parse;
use utils::sipfmt::sipfmt;
use std::env;
use std::fs::{self, File, remove_file};
use std::io::{BufReader, Read, Write, stdout};
use std::process::{Command, exit};

pub mod ast;
pub mod codegen;
pub mod errt;
pub mod help;
pub mod lexer;
pub mod parsers;
pub mod utils;
pub mod tokdefs;

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
    pub use_zig_cc: bool,
    pub zig_optimize: Option<String>,
    pub zig_cpu_features: Option<String>,
    pub zig_link_libc: bool,
}

fn parse_args(args: &[String]) -> (Options, String, String) {
    if args.len() < 2 {
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
        use_zig_cc: false,
        zig_optimize: None,
        zig_cpu_features: None,
        zig_link_libc: true,
    };

    let mut command = None;
    let mut source_file = None;

    let mut iter = args.iter().skip(1).peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "build" | "fmt" | "help" => command = Some(arg.clone()),
            a if a.ends_with(".sip") => source_file = Some(a),
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
            "-D" => options.define_macro = iter.next().cloned(),
            "-I" => options.include_dir = iter.next().cloned(),
            "-L" => options.library_search_path = iter.next().cloned(),
            "-l" => options.library = iter.next().cloned(),
            "--target" => options.target = iter.next().cloned(),
            "--mcpu" => options.cpu = iter.next().cloned(),
            "--march" => options.arch = iter.next().cloned(),
            "--sanitize" => options.sanitize = iter.next().cloned(),
            "--lto" => options.lto = iter.next().cloned(),
            "--cache-dir" => options.cache_dir = iter.next().cloned(),
            "--test" => options.test = true,
            "--trans-zig" => options.trans_zig = true,
            "--out" => options.out = iter.next().cloned(),
            "--retainc" => options.retain_c = true,
            "--zig-cc" => options.use_zig_cc = true,
            "--zig-optimize" => options.zig_optimize = iter.next().cloned(),
            "--zig-cpu-features" => options.zig_cpu_features = iter.next().cloned(),
            "--zig-no-libc" => options.zig_link_libc = false,
            _ => {
                eprintln!("{}: {}", "Unknown option or argument".red(), arg);
                exit(1);
            }
        }
    }

    if command.is_none() || source_file.is_none() {
        eprintln!("{}: missing command or .sip file", "Error".red());
        print_help();
        exit(1);
    }

    (options, command.unwrap(), source_file.unwrap().to_string())
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
    if options.debug{
        println!("tokens:\n{:#?}",&ast);
    }
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

    print!("\r{}", if options.use_zig_cc { "Compiling with Zig CC..." } else { "Compiling with Clang..." }.cyan());
    stdout().flush().unwrap();

    let mut cmd = if options.use_zig_cc {
        Command::new("zig")
    } else {
        Command::new("clang")
    };

    if options.use_zig_cc {
        cmd.arg("cc");
        
        if let Some(opt) = options.zig_optimize.as_ref() {
            cmd.arg(format!("-O{}", opt));
        } else if let Some(opt) = options.opt {
            cmd.arg(format!("-O{}", opt));
        }
        
        if let Some(features) = &options.zig_cpu_features {
            cmd.arg(format!("-mcpu={}", features));
        }
        
        if !options.zig_link_libc {
            cmd.arg("-nostdlib");
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
        
        if let Some(m) = &options.define_macro {
            cmd.arg(format!("-D{}", m));
        }
        
        if let Some(inc) = &options.include_dir {
            cmd.arg(format!("-I{}", inc));
        }
        
        if let Some(lib_path) = &options.library_search_path {
            cmd.arg(format!("-L{}", lib_path));
        }
        
        if let Some(lib) = &options.library {
            cmd.arg(format!("-l{}", lib));
        }
    } else {
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
        if let Some(m) = &options.define_macro {
            cmd.arg(format!("-D{}", m));
        }
        if let Some(inc) = &options.include_dir {
            cmd.arg(format!("-I{}", inc));
        }
        if let Some(lib_path) = &options.library_search_path {
            cmd.arg(format!("-L{}", lib_path));
        }
        if let Some(lib) = &options.library {
            cmd.arg(format!("-l{}", lib));
        }
        if let Some(cpu) = &options.cpu {
            cmd.arg(format!("-mcpu={}", cpu));
        }
        if let Some(arch) = &options.arch {
            cmd.arg(format!("-march={}", arch));
        }
        if let Some(sanitize) = &options.sanitize {
            cmd.arg(format!("-fsanitize={}", sanitize));
        }
        if let Some(lto) = &options.lto {
            cmd.arg(format!("-flto={}", lto));
        }
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
            eprintln!("\n{}: {}", "Failed to execute Clang".red(), e);
        }
    }

    if !options.retain_c {
        let _ = remove_file(&c_file_path);
    }
}

fn main() {
    match enable_ansi_support(){
        Err(e) => {
            eprintln!("unable to enable ansi support!\nerror: {}",e);
            exit(1)
        }
        _ => {}
    }
    let mut args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "fmt" {
        let fmt_args = args.drain(2..).collect::<Vec<_>>();

        if fmt_args.is_empty() {
            eprintln!("{}: no files or directories provided for fmt", "Error".red());
            std::process::exit(1);
        }

        for path in &fmt_args {
            let p = std::path::Path::new(path);
            if p.is_file() {
                if p.extension().and_then(|ext| ext.to_str()) != Some("sip") {
                    eprintln!(
                        "{}: '{}' is not a .sip file , maybe you need to add '.sip' extension?",
                        "Error".red(),
                        path
                    );
                    std::process::exit(1);
                }
            } else if !p.is_dir() {
                eprintln!(
                    "{}: '{}' is neither a directory nor a .sip file , maybe you need to add '.sip' extension?",
                    "Error".red(),
                    path
                );
                std::process::exit(1);
            }
        }

        if let Err(e) = sipfmt(&fmt_args) {
            eprintln!("{}: {}", "Formatting error".red(), e);
            std::process::exit(1);
        }
        return;
    }

    let (options, command, source_file) = parse_args(&args);

    if !source_file.ends_with(".sip") {
        eprintln!(
            "{}: source file '{}' must have a .sip extension",
            "Error".red(),
            source_file
        );
        std::process::exit(1);
    }

    if options.help {
        print_help();
        return;
    }

    match command.as_str() {
        "build" => build(&options, &source_file),
        "help"  => print_help(),
        _       => {
            eprintln!("{}: unknown command '{}'", "Error".red(), command);
            std::process::exit(1);
        }
    }
}

