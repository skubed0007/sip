use colored::*;

pub fn print_help() {
    // Title
    println!(
        "{} {}",
        "SIP".bold().bright_cyan(),
        "— a blazing-fast, C-like language that doesn’t mess around"
            .italic()
            .bright_black()
    );
    println!();

    // Usage
    println!("{}", "USAGE".bold().bright_blue());
    println!(
        "  {} {} {}",
        "sip".bright_white(),
        "<command>".bold().bright_green(),
        "<source-file> [options]".bright_yellow()
    );
    println!(
        "{}",
        "  (Heads up: always toss me the command first, then the file, *then* the fancy stuff)"
            .italic()
            .bright_black()
    );
    println!();

    // Global Options
    println!("{}", "OPTIONS".bold().bright_blue());

    println!(
        "  {} {} I'll keep the C file around, like a good packrat",
        "›".bold().bright_green(),
        "--retainc".bold().bright_yellow()
    );

    println!(
        "  {} {} Tell me how hard to flex my optimizer muscles",
        "›".bold().bright_green(),
        "-O <n>, --opt <n>".bold().bright_yellow()
    );
    println!(
        "     {} {}   Chill mode — no optimizations, perfect for debugging",
        "•".bright_black(),
        "0".bold().bright_magenta()
    );
    println!(
        "     {} {}   A healthy balance of speed and size — just right",
        "•".bright_black(),
        "1".bold().bright_magenta()
    );
    println!(
        "     {} {}   Full throttle — go fast or go home",
        "•".bright_black(),
        "2".bold().bright_magenta()
    );
    println!(
        "     {} {}   Slim-fit binary mode — tight and tiny",
        "•".bright_black(),
        "3".bold().bright_magenta()
    );

    println!(
        "  {} {} I'll sprinkle in debug symbols — detective mode activated",
        "›".bold().bright_green(),
        "-g, --debug".bold().bright_yellow()
    );
    println!(
        "  {} {} I’ll warn you about *everything*, even your bad life choices (jk)",
        "›".bold().bright_green(),
        "-Wall".bold().bright_yellow()
    );
    println!(
        "  {} {} No mercy — warnings become errors",
        "›".bold().bright_green(),
        "-Werror".bold().bright_yellow()
    );
    println!(
        "  {} {} I'll glue everything into one solid binary",
        "›".bold().bright_green(),
        "-static".bold().bright_yellow()
    );
    println!(
        "  {} {} Position-independent code — I can run from anywhere",
        "›".bold().bright_green(),
        "-fPIC, --pic".bold().bright_yellow()
    );
    println!(
        "  {} {} Need a macro? I'll define it, no questions asked",
        "›".bold().bright_green(),
        "-D<macro>[=<val>]".bold().bright_yellow()
    );
    println!(
        "  {} {} I'll look here for includes — like a lost tourist with a map",
        "›".bold().bright_green(),
        "-I<path>".bold().bright_yellow()
    );
    println!(
        "  {} {} Library hunt? I’ll search here too",
        "›".bold().bright_green(),
        "-L<path>".bold().bright_yellow()
    );
    println!(
        "  {} {} I'll link your code to this fine library right here",
        "›".bold().bright_green(),
        "-l<lib>".bold().bright_yellow()
    );

    println!(
        "  {} {} Pick a platform and I’ll play along",
        "›".bold().bright_green(),
        "--target <triple>".bold().bright_yellow()
    );
    for (triple, desc) in &[
        (
            "x86_64-linux-gnu",
            "Classic Linux 64-bit — like jeans, but for compilers",
        ),
        (
            "aarch64-linux-gnu",
            "ARM64 Linux — for your Raspberry dreams",
        ),
        (
            "x86_64-windows-msvc",
            "Windows x64 (MSVC) — serious business",
        ),
        (
            "aarch64-windows-msvc",
            "Windows ARM64 — the quiet revolution",
        ),
        ("x86_64-macos", "macOS x64 — yes, I speak Apple"),
        ("armv7-linux-gnueabihf", "ARM32 Linux — legacy vibes"),
        ("riscv64-linux-gnu", "RISC-V — cutting edge and proud of it"),
    ] {
        println!(
            "     {} {}   {}",
            "•".bright_black(),
            triple.bold().bright_magenta(),
            desc.bright_black()
        );
    }

    println!(
        "  {} {} Tell me which CPU to impress",
        "›".bold().bright_green(),
        "--mcpu <name>".bold().bright_yellow()
    );
    for cpu in &["native", "haswell", "skylake", "cortex-a53", "cortex-a72"] {
        println!(
            "     {} {}",
            "•".bright_black(),
            cpu.bold().bright_magenta()
        );
    }

    println!(
        "  {} {} Architecture? I speak several dialects",
        "›".bold().bright_green(),
        "--march <arch>".bold().bright_yellow()
    );
    for arch in &["native", "x86-64", "armv8-a", "armv7-a", "riscv64"] {
        println!(
            "     {} {}",
            "•".bright_black(),
            arch.bold().bright_magenta()
        );
    }

    println!(
        "  {} {} Runtime checks? Say no more",
        "›".bold().bright_green(),
        "--sanitize <kind>".bold().bright_yellow()
    );
    for kind in &["address", "undefined", "thread", "memory", "leak"] {
        println!(
            "     {} {}",
            "•".bright_black(),
            kind.bold().bright_magenta()
        );
    }

    println!(
        "  {} {} Want me to think ahead? I'll optimize across files",
        "›".bold().bright_green(),
        "--lto [thin|full]".bold().bright_yellow()
    );
    println!(
        "     {} {}   Sleek and quick",
        "•".bright_black(),
        "thin".bold().bright_magenta()
    );
    println!(
        "     {} {}   Full-on mega brain mode",
        "•".bright_black(),
        "full".bold().bright_magenta()
    );

    println!(
        "  {} {} I’ll keep my Zig mess here",
        "›".bold().bright_green(),
        "--cache-dir <path>".bold().bright_yellow()
    );
    println!(
        "  {} {} I'll inject my secret tests into your build",
        "›".bold().bright_green(),
        "--test".bold().bright_yellow()
    );
    println!(
        "  {} {} Experimental: I’ll emit Zig instead of C (don’t judge)",
        "›".bold().bright_green(),
        "--trans-zig".bold().bright_yellow()
    );
    println!(
        "  {} {} I’ll call the result this fancy name",
        "›".bold().bright_green(),
        "--out <file>".bold().bright_yellow()
    );
    println!(
        "  {} {} That’s me! You're reading it",
        "›".bold().bright_green(),
        "-h, --help".bold().bright_yellow()
    );

    println!();

    // Commands
    println!("{}", "COMMANDS".bold().bright_blue());
    println!(
        "  {} {} Compile your stuff. Straightforward. I got you.",
        "→".bold().bright_magenta(),
        "build".bold().bright_green()
    );
    println!();

    // Examples
    println!("{}", "EXAMPLES".bold().bright_blue());
    println!("{}", "  (Format: command → source file → options. Always. No exceptions. Don't try to be fancy.)".italic().bright_black());

    let examples = vec![
        ("sip build main.sip", "Build it like it’s hot"),
        (
            "sip build foo.sip -O 2 --static",
            "Speedy and self-contained — no strings attached",
        ),
        (
            "sip build foo.sip -O 3 -DDEBUG=1",
            "Tiny binary, big brain debugging",
        ),
        (
            "sip build foo.sip --mcpu native",
            "I'll optimize it for *your* CPU — because I care",
        ),
        (
            "sip build foo.sip --march armv8-a",
            "Cross-build to ARM like a boss",
        ),
        (
            "sip build foo.sip --sanitize address",
            "Got bugs? Let me sniff 'em out",
        ),
        (
            "sip build foo.sip --lto full",
            "Full link-time optimization — I'm seeing the whole picture",
        ),
        (
            "sip build foo.sip --target x86_64-macos",
            "I’m pretending to be a Mac today",
        ),
        (
            "sip build foo.sip --trans-zig",
            "Trying out a Ziggy path — experimental stuff ahead",
        ),
    ];

    for (cmd, note) in examples {
        println!(
            "  {} {} {}",
            "★".bold().bright_yellow(),
            cmd.bright_white(),
            format!("- {}", note).bright_black()
        );
    }

    println!();
}
