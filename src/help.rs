use colored::*;

pub fn print_help() {
    // Title
    println!(
        "{} {}",
        "SIP".bold().bright_cyan(),
        "— a blazing-fast, C-like language that doesn't mess around"
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
    let opts = vec![
        ("--retainc", "I'll keep the C file around, like a good packrat"),
        ("fmt | -f <file/folder>", "I am good at cleaning , ama sip up your code"),
        ("-O <n>, --opt <n>", "Tell me how hard to flex my optimizer muscles"),
        ("-g, --debug", "I'll sprinkle in debug symbols — detective mode activated"),
        ("-Wall", "I'll warn you about *everything*, even your bad life choices (jk)"),
        ("-Werror", "No mercy — warnings become errors"),
        ("-static", "I'll glue everything into one solid binary"),
        ("-fPIC, --pic", "Position-independent code — I can run from anywhere"),
        ("-D<macro>[=<val>]", "Need a macro? I'll define it, no questions asked"),
        ("-I<path>", "I'll look here for includes — like a lost tourist with a map"),
        ("-L<path>", "Library hunt? I'll search here too"),
        ("-l<lib>", "I'll link your code to this fine library right here"),
        ("--target <triple>", "Pick a platform and I'll play along"),
        ("--mcpu <name>", "Tell me which CPU to impress"),
        ("--march <arch>", "Architecture? I speak several dialects"),
        ("--sanitize <kind>", "Runtime checks? Say no more"),
        ("--lto [thin|full]", "Want me to think ahead? I'll optimize across files"),
        ("--cache-dir <path>", "I'll keep my Zig mess here"),
        ("--test", "I'll inject my secret tests into your build"),
        ("--trans-zig", "Experimental: I'll emit Zig instead of C (don't judge)"),
        ("--out <file>", "I'll call the result this fancy name"),
        ("--zig-cc", "Use Zig's C compiler instead of Clang — for that extra Ziggy flavor"),
        ("--zig-optimize <level>", "Zig-specific optimization level (0-3)"),
        ("--zig-cpu-features <features>", "Enable specific CPU features for Zig compilation"),
        ("--zig-no-libc", "Don't link libc when using Zig CC — for bare metal fun"),
        ("-h, --help", "That's me! You're reading it"),
    ];

    for (opt, desc) in opts {
        println!(
            "  {} {} {}",
            "›".bold().bright_green(),
            opt.bold().bright_yellow(),
            desc
        );
    }

    // Optimization Levels
    println!("\n{}", "  Optimization Levels:".italic().bright_black());
    let opt_levels = vec![
        ("0", "Chill mode — no optimizations, perfect for debugging"),
        ("1", "A healthy balance of speed and size — just right"),
        ("2", "Full throttle — go fast or go home"),
        ("3", "Slim-fit binary mode — tight and tiny"),
    ];
    for (level, note) in opt_levels {
        println!(
            "     {} {}   {}",
            "•".bright_black(),
            level.bold().bright_magenta(),
            note
        );
    }

    // Targets
    println!("\n{}", "  Target Triples:".italic().bright_black());
    let targets = vec![
    // x86 (Intel/AMD)
    ("x86_64-linux-gnu", "Linux x86-64 (glibc) — standard desktop/server"),
    ("i686-linux-gnu", "Linux x86-32 (glibc) — legacy systems"),
    ("x86_64-linux-musl", "Linux x86-64 (musl libc) — static binaries, Alpine"),
    ("i686-linux-musl", "Linux x86-32 (musl libc) — lightweight systems"),
    ("x86_64-windows-msvc", "Windows x86-64 (MSVC) — native Windows apps"),
    ("x86_64-windows-gnu", "Windows x86-64 (MinGW) — GNU toolchain"),
    ("x86_64-apple-darwin", "macOS x86-64 — Intel Macs"),
    ("x86_64-unknown-freebsd", "FreeBSD x86-64 — server/workstation BSD"),
    ("x86_64-unknown-linux-android", "Android x86-64 — emulator, x86 devices"),

    // ARM64 (AArch64)
    ("aarch64-linux-gnu", "Linux ARM64 — modern servers, Raspberry Pi 4+"),
    ("aarch64-linux-musl", "Linux ARM64 (musl libc) — static binaries"),
    ("aarch64-apple-darwin", "macOS ARM64 — Apple Silicon Macs"),
    ("aarch64-apple-ios", "iOS ARM64 — iPhones/iPads"),
    ("aarch64-windows-msvc", "Windows ARM64 — Surface Pro X"),
    ("aarch64-unknown-freebsd", "FreeBSD ARM64 — embedded/servers"),
    ("aarch64-unknown-linux-android", "Android ARM64 — modern devices"),

    // ARMv7 (32-bit)
    ("armv7-linux-gnueabihf", "Linux ARMv7 (hard-float) — older SBCs like Pi 3"),
    ("armv7-linux-androideabi", "Android ARMv7 — legacy devices"),
    ("armv7-unknown-linux-gnueabihf", "Linux ARMv7 (generic)"),

    // RISC-V
    ("riscv64gc-linux-gnu", "Linux RISC-V 64-bit — open ISA for hardware"),
    ("riscv64-unknown-freebsd", "FreeBSD RISC-V 64-bit — servers/embedded"),

    // MIPS
    ("mips64-linux-gnu", "Linux MIPS64 — legacy systems"),
    ("mips64-unknown-linux-gnu", "Linux MIPS64 (generic)"),

    // PowerPC
    ("ppc64-linux-gnu", "Linux PowerPC 64-bit — IBM POWER systems"),
    ("ppc64-unknown-linux-gnu", "Linux PowerPC 64-bit (generic)"),

    // SPARC
    ("sparc64-linux-gnu", "Linux SPARC 64-bit — legacy Oracle/Sun systems"),

    // SystemZ (s390x)
    ("s390x-linux-gnu", "Linux s390x — IBM Z mainframes"),

    // LoongArch
    ("loongarch64-linux-gnu", "Linux LoongArch 64-bit — Loongson processors"),

    // WebAssembly (WASM)
    ("wasm32-unknown-unknown", "WebAssembly 32-bit — browser/desktop plugins"),
    ("wasm64-unknown-unknown", "WebAssembly 64-bit — large memory support"),

    // Embedded Systems & Microcontrollers
    ("thumbv7m-none-eabi", "ARM Cortex-M3 — bare-metal microcontrollers"),
    ("thumbv7em-none-eabi", "ARM Cortex-M4 — DSP-enabled microcontrollers"),
    ("thumbv8m.base-none-eabi", "ARMv8-M Baseline — Cortex-M55"),
    ("msp430-unknown-none", "TI MSP430 — ultra-low-power MCUs"),
    ("hexagon-unknown-linux-gnueabi", "Qualcomm Hexagon DSP — mobile/heterogeneous computing"),

    // Specialized Targets
    ("nvptx64-nvidia-cuda", "NVIDIA PTX 64-bit — CUDA GPU programming"),
    ("spir64-unknown-unknown", "SPIR 64-bit — OpenCL/SPIR-V"),
    ("bpf-linux", "eBPF — Linux kernel tracing/monitoring"),
];
    for (triple, desc) in targets {
        println!(
            "     {} {}   {}",
            "•".bright_black(),
            triple.bold().bright_magenta(),
            desc.bright_black()
        );
    }

    // CPUs
    println!("\n{}", "  CPUs for --mcpu:".italic().bright_black());
    for cpu in &[
        "native",
        "haswell",
        "skylake",
        "cortex-a53",
        "cortex-a72",
    ] {
        println!(
            "     {} {}",
            "•".bright_black(),
            (*cpu).bold().bright_magenta()
        );
    }

    // Architectures
    println!("\n{}", "  Architectures for --march:".italic().bright_black());
    for arch in &["native", "x86-64", "armv8-a", "armv7-a", "riscv64"] {
        println!(
            "     {} {}",
            "•".bright_black(),
            arch.bold().bright_magenta()
        );
    }

    // Sanitizers
    println!("\n{}", "  Sanitizers for --sanitize:".italic().bright_black());
    for kind in &["address", "undefined", "thread", "memory", "leak"] {
        println!(
            "     {} {}",
            "•".bright_black(),
            kind.bold().bright_magenta()
        );
    }

    // LTO
    println!("\n{}", "  Link-Time Optimization:".italic().bright_black());
    for kind in &["thin", "full"] {
        println!(
            "     {} {}   {}",
            "•".bright_black(),
            kind.bold().bright_magenta(),
            if *kind == "thin" {
                "Sleek and quick"
            } else {
                "Full-on mega brain mode"
            }
        );
    }

    // Commands
    println!("\n{}", "COMMANDS".bold().bright_blue());
    println!(
        "  {} {} {}",
        "→".bold().bright_magenta(),
        "build".bold().bright_green(),
        "Compile your stuff. Straightforward. I got you."
    );

    // Examples
    println!("\n{}", "EXAMPLES".bold().bright_blue());
    println!(
        "{}",
        "  (Format: command → source file → options. Always. No exceptions. Don't try to be fancy.)"
            .italic()
            .bright_black()
    );

    let examples = vec![
        ("sip build main.sip", "Build it like it's hot"),
        ("sip build foo.sip -O 2 --static", "Speedy and self-contained — no strings attached"),
        ("sip build foo.sip -O 3 -DDEBUG=1", "Tiny binary, big brain debugging"),
        ("sip build foo.sip --mcpu native", "I'll optimize it for *your* CPU — because I care"),
        ("sip build foo.sip --march armv8-a", "Cross-build to ARM like a boss"),
        ("sip build foo.sip --sanitize address", "Got bugs? Let me sniff 'em out"),
        ("sip build foo.sip --lto full", "Full link-time optimization — I'm seeing the whole picture"),
        ("sip build foo.sip --target x86_64-macos", "I'm pretending to be a Mac today"),
        ("sip build foo.sip --trans-zig", "Trying out a Ziggy path — experimental stuff ahead"),
        ("sip build foo.sip --zig-cc --zig-optimize 3", "Zig CC with maximum optimization — zoom zoom!"),
        ("sip build foo.sip --zig-cc --zig-cpu-features +avx2,+fma", "Zig CC with specific CPU features enabled"),
        ("sip build foo.sip --zig-cc --zig-no-libc", "Bare metal compilation with Zig CC — no libc crutches"),
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