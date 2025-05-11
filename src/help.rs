use colored::*;

pub fn print_help() {
    // Title
    println!(
        "{} {}",
        "SIP".bold().bright_cyan(),
        "— a blazing-fast, C-like language that doesn’t mess around".italic().bright_black()
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

    println!("  {} {} {}", "›".bold().bright_green(), "--retainc".bold().bright_yellow(), "I'll keep the C file around, like a good packrat");

    println!("  {} {} {}", "›".bold().bright_green(), "-O <n>, --opt <n>".bold().bright_yellow(), "Tell me how hard to flex my optimizer muscles");
    println!("     {} {}   {}", "•".bright_black(), "0".bold().bright_magenta(), "Chill mode — no optimizations, perfect for debugging");
    println!("     {} {}   {}", "•".bright_black(), "1".bold().bright_magenta(), "A healthy balance of speed and size — just right");
    println!("     {} {}   {}", "•".bright_black(), "2".bold().bright_magenta(), "Full throttle — go fast or go home");
    println!("     {} {}   {}", "•".bright_black(), "3".bold().bright_magenta(), "Slim-fit binary mode — tight and tiny");

    println!("  {} {} {}", "›".bold().bright_green(), "-g, --debug".bold().bright_yellow(), "I'll sprinkle in debug symbols — detective mode activated");
    println!("  {} {} {}", "›".bold().bright_green(), "-Wall".bold().bright_yellow(), "I’ll warn you about *everything*, even your bad life choices (jk)");
    println!("  {} {} {}", "›".bold().bright_green(), "-Werror".bold().bright_yellow(), "No mercy — warnings become errors");
    println!("  {} {} {}", "›".bold().bright_green(), "-static".bold().bright_yellow(), "I'll glue everything into one solid binary");
    println!("  {} {} {}", "›".bold().bright_green(), "-fPIC, --pic".bold().bright_yellow(), "Position-independent code — I can run from anywhere");
    println!("  {} {} {}", "›".bold().bright_green(), "-D<macro>[=<val>]".bold().bright_yellow(), "Need a macro? I'll define it, no questions asked");
    println!("  {} {} {}", "›".bold().bright_green(), "-I<path>".bold().bright_yellow(), "I'll look here for includes — like a lost tourist with a map");
    println!("  {} {} {}", "›".bold().bright_green(), "-L<path>".bold().bright_yellow(), "Library hunt? I’ll search here too");
    println!("  {} {} {}", "›".bold().bright_green(), "-l<lib>".bold().bright_yellow(), "I'll link your code to this fine library right here");

    println!("  {} {} {}", "›".bold().bright_green(), "--target <triple>".bold().bright_yellow(), "Pick a platform and I’ll play along");
    for (triple, desc) in &[
        ("x86_64-linux-gnu", "Classic Linux 64-bit — like jeans, but for compilers"),
        ("aarch64-linux-gnu", "ARM64 Linux — for your Raspberry dreams"),
        ("x86_64-windows-msvc", "Windows x64 (MSVC) — serious business"),
        ("aarch64-windows-msvc", "Windows ARM64 — the quiet revolution"),
        ("x86_64-macos", "macOS x64 — yes, I speak Apple"),
        ("armv7-linux-gnueabihf", "ARM32 Linux — legacy vibes"),
        ("riscv64-linux-gnu", "RISC-V — cutting edge and proud of it"),
    ] {
        println!("     {} {}   {}", "•".bright_black(), triple.bold().bright_magenta(), desc.bright_black());
    }

    println!("  {} {} {}", "›".bold().bright_green(), "--mcpu <name>".bold().bright_yellow(), "Tell me which CPU to impress");
    for cpu in &["native", "haswell", "skylake", "cortex-a53", "cortex-a72"] {
        println!("     {} {}", "•".bright_black(), cpu.bold().bright_magenta());
    }

    println!("  {} {} {}", "›".bold().bright_green(), "--march <arch>".bold().bright_yellow(), "Architecture? I speak several dialects");
    for arch in &["native", "x86-64", "armv8-a", "armv7-a", "riscv64"] {
        println!("     {} {}", "•".bright_black(), arch.bold().bright_magenta());
    }

    println!("  {} {} {}", "›".bold().bright_green(), "--sanitize <kind>".bold().bright_yellow(), "Runtime checks? Say no more");
    for kind in &["address", "undefined", "thread", "memory", "leak"] {
        println!("     {} {}", "•".bright_black(), kind.bold().bright_magenta());
    }

    println!("  {} {} {}", "›".bold().bright_green(), "--lto [thin|full]".bold().bright_yellow(), "Want me to think ahead? I'll optimize across files");
    println!("     {} {}   {}", "•".bright_black(), "thin".bold().bright_magenta(), "Sleek and quick");
    println!("     {} {}   {}", "•".bright_black(), "full".bold().bright_magenta(), "Full-on mega brain mode");

    println!("  {} {} {}", "›".bold().bright_green(), "--cache-dir <path>".bold().bright_yellow(), "I’ll keep my Zig mess here");
    println!("  {} {} {}", "›".bold().bright_green(), "--test".bold().bright_yellow(), "I'll inject my secret tests into your build");
    println!("  {} {} {}", "›".bold().bright_green(), "--trans-zig".bold().bright_yellow(), "Experimental: I’ll emit Zig instead of C (don’t judge)");
    println!("  {} {} {}", "›".bold().bright_green(), "--out <file>".bold().bright_yellow(), "I’ll call the result this fancy name");
    println!("  {} {} {}", "›".bold().bright_green(), "-h, --help".bold().bright_yellow(), "That’s me! You're reading it");

    println!();

    // Commands
    println!("{}", "COMMANDS".bold().bright_blue());
    println!("  {} {} {}", "→".bold().bright_magenta(), "build".bold().bright_green(), "Compile your stuff. Straightforward. I got you.");
    println!();

    // Examples
    println!("{}", "EXAMPLES".bold().bright_blue());
    println!("{}", "  (Format: command → source file → options. Always. No exceptions. Don't try to be fancy.)".italic().bright_black());

    let examples = vec![
        ("sip build main.sip", "Build it like it’s hot"),
        ("sip build foo.sip -O 2 --static", "Speedy and self-contained — no strings attached"),
        ("sip build foo.sip -O 3 -DDEBUG=1", "Tiny binary, big brain debugging"),
        ("sip build foo.sip --mcpu native", "I'll optimize it for *your* CPU — because I care"),
        ("sip build foo.sip --march armv8-a", "Cross-build to ARM like a boss"),
        ("sip build foo.sip --sanitize address", "Got bugs? Let me sniff 'em out"),
        ("sip build foo.sip --lto full", "Full link-time optimization — I'm seeing the whole picture"),
        ("sip build foo.sip --target x86_64-macos", "I’m pretending to be a Mac today"),
        ("sip build foo.sip --trans-zig", "Trying out a Ziggy path — experimental stuff ahead"),
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
