<img src="logo.svg" alt="Sip" width="200">

# Sip Programming Language
**Sip** is a lightweight, blazingly fast systems programming language.  
Its unique syntax, external/C interop, rich error reporting, and a tiny AST make it perfect for crafting high performance, low level code.

---

## 🚀 Features

- **C‑style syntax**  
- **`extern` / `link` / `clink`** for seamless C interoperability  
- **Powerful error reporting** with `ErrT` kinds (`ExpectName`, `MissingSemicolon`, etc.)  
- **Flexible AST** (`NT::Extern`, `NT::Link`, `NT::fncall`)  
- **Zero‑allocation identifiers** via `smallstr` inline buffers  
- **Simple lexer** supporting comments, numbers, strings, operators, and more  
- **Parser modules** for `extern`, `link`, function calls, etc.  
- **Memory‑mapped input** for blazing‑fast file access  
- **Configurable build** with optional features  
- **Colorful diagnostics** that guide you to fix issues quickly  
- **Extensible design**—add new AST nodes, error kinds, or parse rules easily

---

## 📦 Installation

Follow these steps to get **Sip** up and running on your system:

1. **Prerequisites**  
   - Rust toolchain (Rust 1.60+).  
     Install Rust with [rustup](https://rustup.rs/):  
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```  
   - Ensure you have `git`, `cargo`, and a C compiler if you plan to link against native libraries.

2. **Clone the Repository**  
   ```bash
   git clone https://github.com/your‑username/sip.git
   cd sip
   ```

3. **Build in Debug Mode**  
   ```bash
   cargo build
   ```

4. **Build in Release Mode**  
   ```bash
   cargo build --release
   ```

5. **(Optional) Install to PATH**  
   ```bash
   sudo cp target/release/sip /usr/local/bin/
   ```

6. **Verify Installation**  
   ```bash
   sip --help
   ```

OR

just download the binary file from [here](bin) and place it on your path
---

## ⚙️ Usage

After building, you can run **Sip** directly:

```bash
sip <SOURCE_FILE>
```

### Example

Given a source file `example.sip`:

```c
//link unistd.h
clink "unistd.h";
extern write fd(i32) /*give optional types*/ text len(i32) = "write" fd text len;
//call the function now
write(1,"hello world",12);
```

---

## 📂 Project Structure

```
sip/
├── Cargo.toml               # Project manifest
├── LICENSE                  # MIT License
├── README.md                # ← this file
└── src/
    ├── main.rs              # CLI entrypoint & ANSI setup
    ├── lexer.rs             # Byte‐level tokenizer
    ├── parsers/
    │   ├── mod.rs           # Parser entrypoint
    │   ├── externp.rs       # `extern` declarations
    │   ├── linkp.rs         # `link` / `clink` parsing
    │   ├── fncall.rs        # Function‐call parsing
    │   └── other.rs         # Other parse modules
    ├── ast.rs               # AST node & `Var` enums
    ├── tokdefs.rs           # Token‐type definitions
    ├── errt/
    │   ├── t.rs             # `ErrT` & `SErr`
    │   └── d1.rs            # `DErr` diagnostic renderer
    └── codegen/             # code generation 
```

---

## 📐 Language Specification

### Lexical Elements

1. **Identifiers**  
   - `[a-zA-Z_][a-zA-Z0-9_]*`  
   - Quoted identifiers: `"..."` (double-quoted)  
2. **Literals**  
   - Integer: `0-9+` (decimal)  
   - Floating point: digits with `.`  
3. **Operators**  
   - Arithmetic: `+`, `-`, `*`, `/`  
   - Assignment: `=`  
   - Variadic: `...`  
4. **Delimiters**  
   - Parentheses: `(`, `)`  
   - Braces: `{`, `}`  
   - Brackets: `[`, `]`  
   - Semicolon: `;`  
   - Comma: `,`  

### Grammar (EBNF)

```ebnf
<program>      ::= { <declaration> | <link> | <statement> }
<declaration>  ::= "extern" <sip-ident> "(" [<param-list>] ")" "=" <c-ident> "(" [<ctype-list>] ")" ";"
<link>         ::= "link" <string-literal> { <string-literal> } ";"
<statement>    ::= <function-call> ";"

<param-list>   ::= <typed-var> { "," <typed-var> } 
<ctype-list>   ::= <c-type> { "," <c-type> }
<typed-var>    ::= <sip-ident> "(" <c-type> ")" | "..."
<c-type>       ::= "i32" | "f32" | custom ident

<function-call>::= "call" <sip-ident> "(" [<arg-list>] ")"
<arg-list>     ::= <literal> | <identifier> | <function-call> | <list> { "," ... }
```

---

## 📜 Detailed Components

### Lexer (`lexer.rs`)

- **Whitespace & Comments**: Skips spaces, tabs, newlines, `//`, `/* ... */`  
- **Token Types**:  
  - `TT::IDENT`, `TT::NUM`, `TT::OP`, etc.  
  - Quoted strings: `TT::IDENT(IDT::DQ, ...)`  
- **Error Tokens**: `TT::ErrT` for unexpected characters  

### Parser (`parsers/`) Modules

- **`externp.rs`**  
  - Parses `extern` declarations  
  - States: `ExpectSipName`, `ExpectSipArgOrEq`, `ExpectEq`, `ExpectCName`, `ExpectCArgOrSemicolon`, `Done`  
  - Builds `NT::Extern(sipname, sipfnargs, cname, cfnargs)`  
  - Error cases: missing identifier, missing `=`, unexpected token in arg lists, missing semicolon  
  - Uses `parse_typed_var` to handle typed parameters and variadic `...`

- **`linkp.rs`**  
  - Parses `link` / `clink` directives  
  - Consumes tokens until `;`, collecting double‑quoted identifiers  
  - Builds `NT::Link(Vec<SmallString>)`  
  - Error cases: no libraries before `;`, unexpected tokens, EOF without `;`

- **`fncall.rs`**  
  - Parses function calls via `call <name>(...) ;`  
  - Looks up `extern` declarations in AST to validate signatures  
  - Iterates over expected parameters, consuming commas and arguments  
  - Supports nested calls, numeric literals, lists, variadic arguments  
  - Builds `NT::fncall(SmallString::from(fnname), args)`  
  - Error cases: unexpected token, mismatched types, missing `)`, missing `;`

- **Error Handling**  
  - All parsers accumulate `Vec<SErr>` on failure  
  - Return `Err(errs)` if any parse errors occurred  
  - Top‑level `parse()` returns either `Ok(Vec<NT>)` AST or `Err(Vec<SErr>)`

### Error Types (`errt/t.rs`)

- **`ErrT`** enum  
  - Core variants:  
    - `ExpectName`  
    - `UnexpectTok`  
    - `MissingSemicolon`  
    - `UnterminatedLiteral`  
    - `InvalidNumber`  
    - `UnknownKeyword`  
    - `UnmatchedParen` / `UnmatchedBrace` / `UnmatchedBracket`  
    - `ExpectOperator` / `ExpectExpression`  
    - `Generic(String)`  
  - Each paired with human‑readable message in `d1.rs`

- **`SErr`** struct  
  - Fields:  
    - `t: ErrT` – error kind  
    - `line: i32` – 1‑based line number (or `-1` for missing location)  
    - `start: i32`, `end: i32` – character indices in source line  
    - `file: String` – file path for context  

- **`DErr`** (`errt/d1.rs`)  
  - Renders colorful diagnostics:  
    1. `error[E{line:04}]: {kind}`  
    2. `  --> file:line:col` location bar  
    3. Source frame with `╭─[source]`, code line, caret(s) under the error span  
    4. Detail message under caret(s)  
    5. Optional `[help]` section with suggestions  
    6. Optional `note:` with extra context  

---

***The Sip Programming language and all its source files , source codes and tools present in the repo found here: https://github.com/skubed0007/sip is licensed under MIT License found here: [LICENSE](LICENSE)***
