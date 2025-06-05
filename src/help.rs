use colored::*;

pub fn print_help() {
    // Header banner with technical aesthetic
    println!(
        "{} {}",
        "SIP COMPILER".bold().bright_cyan(),
        "— statically-typed systems language targeting modern architectures"
            .italic()
            .bright_black()
    );
    println!("{}", "=".repeat(76).bright_black());

    // Usage syntax
    println!("{}", "USAGE".bold().bright_blue());
    println!(
        "  {} {} {}",
        "sip".bright_white(),
        "[command]".bold().bright_green(),
        "[source-file] [options]".bright_yellow()
    );

    // Global options section
    println!("\n{}", "COMPILATION OPTIONS".bold().bright_blue());
    println!("{}", "-".repeat(76).bright_black());

    let options = vec![
        ("--retainc", "Preserve intermediate C translation unit"),
        (
            "-f, fmt <file>",
            "Format source code according to style guide",
        ),
        ("-O<n>, --opt <n>", "Optimization level specification (0-3)"),
        ("-g", "Generate debug information"),
        ("-Wall", "Enable all diagnostic warnings"),
        ("-Werror", "Treat warnings as compilation errors"),
        ("-static", "Produce statically linked executable"),
        ("-fPIC", "Generate position-independent code"),
        ("-D<macro>[=<val>]", "Define preprocessing macro"),
        ("-I<path>", "Include search path specification"),
        ("-L<path>", "Library search path specification"),
        ("-l<lib>", "Link against specified library"),
        (
            "--target <triple>",
            "Cross-compilation target specification",
        ),
        ("--mcpu <name>", "CPU-specific optimization target"),
        ("--march <arch>", "Architecture feature set selection"),
        ("--sanitize <kind>", "Enable runtime error detection"),
        ("--lto [thin|full]", "Link-time optimization strategy"),
        ("--cache-dir <path>", "Compiler cache directory override"),
        ("--test", "Include unit test harness"),
        ("--trans-zig", "Use Zig transpiler backend (experimental)"),
        ("--out <file>", "Custom output binary path"),
        ("--zig-cc", "Use Zig's C-compatible compiler driver"),
        (
            "--zig-optimize <level>",
            "Zig backend optimization level (0-3)",
        ),
        ("--zig-cpu-features <ftrs>", "Target CPU feature modifiers"),
        ("--zig-no-libc", "Disable implicit libc linking"),
        ("-h, --help", "Display this help message"),
    ];

    for (opt, desc) in options {
        println!(
            "  {} {}   {}",
            "→".bright_blue(),
            opt.bold().bright_yellow(),
            desc.dimmed()
        );
    }

    // Optimization level reference
    println!("\n{}", "OPTIMIZATION PROFILES".bold().bright_blue());
    println!("{}", "-".repeat(76).bright_black());

    let optimizations = vec![
        ("0", "No optimization - prioritize debuggability"),
        (
            "1",
            "Basic optimizations - balanced compilation time and performance",
        ),
        ("2", "Aggressive optimizations - maximum performance focus"),
        ("3", "Size optimizations - minimize binary footprint"),
    ];

    for (level, desc) in optimizations {
        println!(
            "  {} {}   {}",
            "O".bright_green(),
            format!(" -O{}", level).bold().bright_magenta(),
            desc.dimmed()
        );
    }

    // Target architecture matrix
    println!("\n{}", "TARGET ARCHITECTURE TRIPLES".bold().bright_blue());
    println!("{}", "-".repeat(76).bright_black());

    let targets = vec![
        // x86 (Intel/AMD)
        (
            "x86_64-linux-gnu",
            "Linux x86-64 (glibc) — standard desktop/server",
        ),
        ("i686-linux-gnu", "Linux x86-32 (glibc) — legacy systems"),
        (
            "x86_64-linux-musl",
            "Linux x86-64 (musl libc) — static binaries, Alpine",
        ),
        (
            "i686-linux-musl",
            "Linux x86-32 (musl libc) — lightweight systems",
        ),
        (
            "x86_64-windows-msvc",
            "Windows x86-64 (MSVC) — native Windows apps",
        ),
        (
            "x86_64-windows-gnu",
            "Windows x86-64 (MinGW) — GNU toolchain",
        ),
        ("x86_64-apple-darwin", "macOS x86-64 — Intel Macs"),
        (
            "x86_64-unknown-freebsd",
            "FreeBSD x86-64 — server/workstation BSD",
        ),
        (
            "x86_64-unknown-linux-android",
            "Android x86-64 — emulator, x86 devices",
        ),
        // ARM64 (AArch64)
        (
            "aarch64-linux-gnu",
            "Linux ARM64 — modern servers, Raspberry Pi 4+",
        ),
        (
            "aarch64-linux-musl",
            "Linux ARM64 (musl libc) — static binaries",
        ),
        ("aarch64-apple-darwin", "macOS ARM64 — Apple Silicon Macs"),
        ("aarch64-apple-ios", "iOS ARM64 — iPhones/iPads"),
        ("aarch64-windows-msvc", "Windows ARM64 — Surface Pro X"),
        (
            "aarch64-unknown-freebsd",
            "FreeBSD ARM64 — embedded/servers",
        ),
        (
            "aarch64-unknown-linux-android",
            "Android ARM64 — modern devices",
        ),
        // ARMv7 (32-bit)
        (
            "armv7-linux-gnueabihf",
            "Linux ARMv7 (hard-float) — older SBCs like Pi 3",
        ),
        ("armv7-linux-androideabi", "Android ARMv7 — legacy devices"),
        ("armv7-unknown-linux-gnueabihf", "Linux ARMv7 (generic)"),
        // RISC-V
        (
            "riscv64gc-linux-gnu",
            "Linux RISC-V 64-bit — open ISA for hardware",
        ),
        (
            "riscv64-unknown-freebsd",
            "FreeBSD RISC-V 64-bit — servers/embedded",
        ),
        // MIPS
        ("mips64-linux-gnu", "Linux MIPS64 — legacy systems"),
        ("mips64-unknown-linux-gnu", "Linux MIPS64 (generic)"),
        // PowerPC
        (
            "ppc64-linux-gnu",
            "Linux PowerPC 64-bit — IBM POWER systems",
        ),
        ("ppc64-unknown-linux-gnu", "Linux PowerPC 64-bit (generic)"),
        // SPARC
        (
            "sparc64-linux-gnu",
            "Linux SPARC 64-bit — legacy Oracle/Sun systems",
        ),
        // SystemZ (s390x)
        ("s390x-linux-gnu", "Linux s390x — IBM Z mainframes"),
        // LoongArch
        (
            "loongarch64-linux-gnu",
            "Linux LoongArch 64-bit — Loongson processors",
        ),
        // WebAssembly (WASM)
        (
            "wasm32-unknown-unknown",
            "WebAssembly 32-bit — browser/desktop plugins",
        ),
        (
            "wasm64-unknown-unknown",
            "WebAssembly 64-bit — large memory support",
        ),
        // Embedded Systems & Microcontrollers
        (
            "thumbv7m-none-eabi",
            "ARM Cortex-M3 — bare-metal microcontrollers",
        ),
        (
            "thumbv7em-none-eabi",
            "ARM Cortex-M4 — DSP-enabled microcontrollers",
        ),
        ("thumbv8m.base-none-eabi", "ARMv8-M Baseline — Cortex-M55"),
        ("msp430-unknown-none", "TI MSP430 — ultra-low-power MCUs"),
        (
            "hexagon-unknown-linux-gnueabi",
            "Qualcomm Hexagon DSP — mobile/heterogeneous computing",
        ),
        // Specialized Targets
        (
            "nvptx64-nvidia-cuda",
            "NVIDIA PTX 64-bit — CUDA GPU programming",
        ),
        ("spir64-unknown-unknown", "SPIR 64-bit — OpenCL/SPIR-V"),
        ("bpf-linux", "eBPF — Linux kernel tracing/monitoring"),
    ];
    for (triple, desc) in targets {
        println!(
            "  {} {}   {}",
            "T".bright_green(),
            triple.bold().bright_magenta(),
            desc.dimmed()
        );
    }

    // Command syntax
    println!("\n{}", "COMMANDS".bold().bright_blue());
    println!("{}", "-".repeat(76).bright_black());

    let commands = vec![("build", "Compile source to native binary")];

    for (cmd, desc) in commands {
        println!(
            "  {} {}   {}",
            "C".bright_blue(),
            cmd.bold().bright_green(),
            desc.dimmed()
        );
    }

    // Usage examples
    println!("\n{}", "USAGE EXAMPLES".bold().bright_blue());
    println!("{}", "-".repeat(76).bright_black());

    let examples = vec![
        ("sip build main.sip", "Basic compilation"),
        ("sip build app.sip -O 2 --static", "Optimized static build"),
        (
            "sip build module.sip --march armv8-a",
            "Cross-compile for ARMv8",
        ),
        (
            "sip build test.sip --sanitize address",
            "Enable address checking",
        ),
    ];

    for (cmd, desc) in examples {
        println!(
            "  {} {}   {}",
            "»".bright_yellow(),
            cmd.bright_white(),
            desc.dimmed()
        );
    }

    println!("{}", "=".repeat(76).bright_black());
}
