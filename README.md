<div align="center">
  <img src="logo.png" alt="Sip Programming Language Logo" width="200" height="200"/>
  
<h1 align="center" style="decoration:bold;font-size::1en">Sip Programming Language</h1>
  <h1>Take a Sip and Relax</h1>
  <p><em>A Refreshing Take on Programming</em></p>
</div>

<div align="center">
  <h2>Why Choose Sip?</h2>
  <p>Sip is a modern programming language designed for simplicity, safety, and seamless interoperability with C. Whether you're building high-performance systems or experimenting with new ideas, Sip makes it easy to write clean, efficient, and maintainable code.</p>
</div>

<div align="left">
  <h2>Core Features</h2>
  <ul>
    <li><strong>Interoperability:</strong> Seamlessly integrate with C for low-level control and performance.</li>
    <li><strong>Memory Management:</strong> Explicit and safe memory handling with constructs like <code>lazypage</code> and <code>zeropage</code>.</li>
    <li><strong>Null Safety:</strong> Built-in <code>nil</code> checks to prevent null pointer errors.</li>
    <li><strong>Modern Syntax:</strong> Intuitive and expressive syntax for faster development.</li>
  </ul>
</div>

---

## ⚠️ Sip Responsibly ⚠️

> **CAUTION**: Sip's syntax is still brewing and may change until version 0.5.0 is released. Code examples shown here might not reflect the final recipe. ~~Check the documentation for the freshest updates.~~

---

## The Serious Business Part (But Still Fun)

### Variables That Flow Naturally
Sip keeps it simple: variables are mutable by default, constants are declared with `const`, and global variables are explicitly marked. No surprises, just smooth coding.

```sip
// Mutable variable (because life changes)
i32 count = 0;

// Immutable constant (set in stone)
const i32 limit = 100;

// Global variable (everyone’s business)
global i32 total = 0;

// Global constant (the universal truth)
global const i32 MAX = 100;
```

### Functions That Pour Out Clarity
Functions in Sip are like a perfectly measured drink—balanced, clear, and satisfying. Explicit return types and parameter declarations make everything easy to follow.

```sip
// A simple addition function
fun add(i32 a, i32 b) -> i32 {
    return a + b;  // Simple math, simple life
}

```

### Error Handling: No Bitter Aftertaste
Sip doesn’t believe in exceptions. Instead, it gives you the tools to handle errors explicitly and gracefully. Defensive programming has never been this smooth.

```sip
// Allocating memory with nil-checking
Page mem = PAGE(4M);
nil (mem == nil) {
    std.print("Oops, we spilled something!");
    exit(1);
}

// Safely using the allocated memory
mem.write(data);
```

### Memory Management: No Spills, Just Control
Sip puts you in control of memory without making you feel like you’re juggling glasses. Explicit allocation and guarded access keep things safe and predictable.

```sip
// Allocating a 4MB page
Page mem = PAGE(4M);

// Checking the allocation result
check mem {
    OK(page) => std.print("Memory: acquired!"),
    ERR(_)   => std.print("Memory: Mission failed!");
}

// Always guard memory access
nil (mem != nil) {
    mem.write(data);
}
```

### C++ Interoperability: A Perfect Blend
Sip integrates seamlessly with C++, allowing you to call C++ functions and use C++ libraries.

```sip
bring "sip-cpp";

// Calling a C++ function
result = sipcpp.call("std::pow", [2, 3]);
std.print("2^3 = " + result);
```

---

<div align="center">
  <h2>Getting Started</h2>
  <p>Write your first Sip program and experience the clarity and simplicity of Sip.</p>
</div>

```sip
@fun test(i32 age);
fun main(){
    lazypage mem = _PAGE(4M);
    nil (mem) {
        @c {
            nprintf("error allocating lazy page!");
        }
    }
    zeropage zmem = _PAGE(4M);
    //shall cause error as `nil` isnt checked for zmem
    i32 age = _ALLOCMEM(zmem,12);
}
fun test(i32 age){
    //code
}
```

---

<div align="center">
  <h2>Conclusion</h2>
  <p>Sip is more than just a programming language—it’s a refreshing philosophy. It’s about writing code that’s honest, clear, and predictable.</p>
  <p><em>Take a Sip and start coding!</em></p>
</div>

<div align="center">
  <h2>Community and Support</h2>
  <p>Join our growing community of developers who are sipping their way to better code. Share your experiences, ask questions, and contribute to the Sip ecosystem.</p>
  <p><strong>Links:</strong></p>
  <ul>
    <li><a href="https://github.com/sip-lang">GitHub</a></li>
    <li><a href="https://sip-lang.org">Official Website</a></li>
    <li><a href="https://discord.gg/sip-lang">Discord</a></li>
  </ul>
</div>

