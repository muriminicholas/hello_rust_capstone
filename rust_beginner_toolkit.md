# Getting Started with Rust – A Beginner's Toolkit
### Capstone: Generative AI-Assisted Learning Guide

---

## 1. Title & Objective

**Technology Chosen:** Rust – a systems programming language focused on safety, speed, and concurrency.

**Why Rust?**
Rust is one of the most loved programming languages (ranked #1 on Stack Overflow's Developer Survey for several consecutive years). It offers memory safety *without* a garbage collector — a rare combination that makes it valuable for building reliable, high-performance software. It is also completely distinct from Python, Java, and JavaScript, making it a challenging and impressive capstone choice.

**End Goal:**
By the end of this guide, you will:
- Have Rust installed and configured on your machine.
- Successfully compile and run a minimal working Rust program ("Hello, World!").
- Understand the basics of Rust's project structure using Cargo.
- Be equipped with resources to continue learning independently.

---

## 2. Quick Summary of the Technology

**What is Rust?**
Rust is a compiled, statically-typed, general-purpose systems programming language created by Mozilla Research and first released in 2015. It gives programmers fine-grained control over memory and system resources — similar to C and C++ — while enforcing memory safety at compile time through its unique *ownership* and *borrowing* model. This means Rust programs cannot have dangling pointers, null pointer dereferences, or data races by default.

**Where is it used?**
- **Operating systems** – The Linux kernel has begun accepting Rust code alongside C.
- **WebAssembly** – Rust compiles to Wasm for high-performance browser applications.
- **Game engines** – Bevy is a data-driven game engine built entirely in Rust.
- **CLI tools** – Tools like `ripgrep` (a blazing-fast search tool) are written in Rust.
- **Networking & backends** – Discord rewrote performance-critical services from Go to Rust.
- **Embedded systems** – Rust runs on microcontrollers with very limited resources.

**One Real-World Example:**
**Discord** migrated their "Read States" service (which tracks which messages you've read) from Go to Rust. The result was dramatically lower latency and near-zero CPU spikes, because Rust has no garbage collector that periodically pauses execution. This is a textbook use case showing Rust's advantage in latency-sensitive, high-throughput systems.

---

## 3. System Requirements

| Requirement       | Details                                              |
|-------------------|------------------------------------------------------|
| **OS**            | Linux, macOS, or Windows (all supported)             |
| **RAM**           | 512 MB minimum (2 GB+ recommended for compilation)   |
| **Disk Space**    | ~1 GB for Rust toolchain + Cargo packages            |
| **Terminal**      | Bash, Zsh, PowerShell, or any shell                  |
| **Text Editor**   | VS Code (recommended), Neovim, or any editor         |
| **Internet**      | Required for installation and downloading crates     |

**Recommended VS Code Extension:**
- `rust-analyzer` – provides IntelliSense, error highlighting, and code completion for Rust.

---

## 4. Installation & Setup Instructions

### Step 1 – Install Rust via `rustup`

`rustup` is the official Rust installer and version management tool. It installs everything you need: the Rust compiler (`rustc`), the package manager (`cargo`), and the standard library.

**On Linux or macOS**, open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts. When asked, choose option `1) Proceed with installation (default)`.

**On Windows**, download and run the installer from:
> https://rustup.rs

Then follow the GUI prompts. You may also be prompted to install the Visual Studio C++ Build Tools — accept this, as Rust needs it on Windows.

---

### Step 2 – Reload Your Shell Environment

After installation, reload your terminal session so the `rustc` and `cargo` commands are available:

```bash
source $HOME/.cargo/env
```

Or simply close and reopen your terminal.

---

### Step 3 – Verify the Installation

Run the following to confirm everything is installed correctly:

```bash
rustc --version
cargo --version
```

**Expected output (versions may differ):**

```
rustc 1.78.0 (9b00956e5 2024-04-29)
cargo 1.78.0 (54d8815d0 2024-04-09)
```

---

### Step 4 – Install `rust-analyzer` in VS Code (Recommended)

1. Open VS Code.
2. Press `Ctrl+Shift+X` (or `Cmd+Shift+X` on Mac) to open Extensions.
3. Search for **rust-analyzer**.
4. Click **Install**.

---

## 5. Minimal Working Example

### What This Example Does

This program creates a Rust project using Cargo, defines a `greet` function, and prints a personalised greeting to the terminal. It demonstrates:
- Rust's `main` function as the entry point.
- Defining and calling functions.
- String formatting with `println!`.
- Cargo's project structure.

---

### Step 1 – Create a New Project with Cargo

```bash
cargo new hello_rust
cd hello_rust
```

Cargo creates the following structure:

```
hello_rust/
├── Cargo.toml      # Project metadata and dependencies
└── src/
    └── main.rs     # Your Rust source code
```

---

### Step 2 – Write the Code

Open `src/main.rs` in your editor and replace its contents with:

```rust
// src/main.rs

// The main function is the entry point of every Rust program
fn main() {
    // Call our custom greet function
    greet("Moringa Student");
}

// A function that accepts a string slice (&str) as a parameter
// and prints a formatted greeting to the terminal
fn greet(name: &str) {
    // println! is a macro (not a function) — note the ! 
    // The {} is a placeholder that gets replaced by `name`
    println!("Hello, {}! Welcome to Rust 🦀", name);
    println!("You are now writing systems-level code safely.");
}
```

---

### Step 3 – Run the Program

```bash
cargo run
```

**What happens behind the scenes:**
1. Cargo compiles your code using `rustc`.
2. Places the binary in `target/debug/hello_rust`.
3. Executes it immediately.

---

### Expected Output

```
   Compiling hello_rust v0.1.0 (/path/to/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/hello_rust`
Hello, Moringa Student! Welcome to Rust 🦀
You are now writing systems-level code safely.
```

---

### Bonus: Build Without Running

To compile without executing (generates a binary you can share):

```bash
cargo build
./target/debug/hello_rust
```

For an optimised release build:

```bash
cargo build --release
./target/release/hello_rust
```

---

## 6. AI Prompt Journal

The following prompts were used with an AI assistant (e.g. ai.moringaschool.com / Claude) during the creation of this capstone.

---

### Prompt 1
**Prompt used:**
> *"Give me a beginner-friendly overview of the Rust programming language — what it is, where it's used, and why it matters in 2024."*

**AI Response Summary:**
The AI explained Rust's ownership model as the core differentiator, described use cases across systems programming, WebAssembly, and CLI tools, and highlighted the Discord case study as a compelling real-world example. It also clarified the difference between Rust and other memory-safe languages like Go.

**Evaluation:** Very helpful. Gave me a solid conceptual foundation before writing any code. I used the Discord example directly after verifying it independently.

---

### Prompt 2
**Prompt used:**
> *"What is the difference between rustc and cargo in Rust? Which should a beginner use?"*

**AI Response Summary:**
The AI clarified that `rustc` is the raw compiler while `cargo` is the build system and package manager that wraps `rustc`. It recommended always using `cargo` for real projects and explained the Cargo.toml file.

**Evaluation:** Clarified a common beginner confusion. This shaped the setup section of this guide.

---

### Prompt 3
**Prompt used:**
> *"Write a simple Rust hello world that also demonstrates defining a function and passing a string argument. Add inline comments for a beginner."*

**AI Response Summary:**
The AI produced a clean example using `fn greet(name: &str)` with `println!` and explained the `&str` vs `String` distinction and why `println!` uses a `!` (it's a macro, not a function).

**Evaluation:** Excellent. The inline comments approach worked well for a beginner guide. I used this as the foundation for the working example section.

---

### Prompt 4
**Prompt used:**
> *"What are the most common errors beginners make when first running Rust code, and how do they fix them?"*

**AI Response Summary:**
The AI listed: missing semicolons, misuse of `String` vs `&str`, borrow checker violations, and forgetting to run `source $HOME/.cargo/env` after installation. Each error came with the compiler message and the fix.

**Evaluation:** Directly shaped the Common Issues & Fixes section below.

---

### Prompt 5
**Prompt used:**
> *"What should I include in a README.md for a beginner Rust project on GitHub?"*

**AI Response Summary:**
The AI suggested: project description, prerequisites (Rust + Cargo versions), installation steps, how to run, expected output, and a license. This became the template for the project's README.

**Evaluation:** Saved significant time. I adapted the structure directly.

---

## 7. Common Issues & Fixes

### Issue 1 – `rustc: command not found` after installation

**Symptom:**
```
bash: rustc: command not found
```

**Cause:** The shell environment wasn't reloaded after installation.

**Fix:**
```bash
source $HOME/.cargo/env
```
Or close and reopen your terminal completely.

---

### Issue 2 – `error[E0308]: mismatched types` with strings

**Symptom:**
```
error[E0308]: mismatched types
expected `&str`, found `String`
```

**Cause:** Rust distinguishes between `String` (heap-allocated, owned) and `&str` (a string slice/reference). Passing a `String` where `&str` is expected causes a type error.

**Fix:** Add a `&` before the variable or use `.as_str()`:
```rust
let name = String::from("Alice");
greet(&name);  // ✅ Coerces String → &str
```

---

### Issue 3 – `cannot borrow as mutable` (Borrow Checker)

**Symptom:**
```
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
```

**Cause:** Variables in Rust are **immutable by default**.

**Fix:** Declare the variable with `mut`:
```rust
let mut x = 5;  // ✅ Now mutable
x = 10;
```

---

### Issue 4 – Missing semicolon

**Symptom:**
```
error: expected `;`, found `}`
```

**Fix:** Add a semicolon at the end of the statement. Note: the *last expression* in a function body intentionally omits the semicolon to return a value — but statements always need one.

---

### Issue 5 – Cargo not finding `Cargo.toml`

**Symptom:**
```
error: could not find `Cargo.toml` in `/home/user` or any parent directory
```

**Cause:** You're running `cargo run` from the wrong directory.

**Fix:**
```bash
cd hello_rust   # Navigate into the project folder first
cargo run
```

---

### Issue 6 – Long compile times on first build

**Symptom:** First `cargo run` takes 30–60 seconds.

**Cause:** Rust compiles dependencies from source. This is normal on the first build.

**Fix:** Subsequent builds are incremental and much faster. The `target/` directory caches compiled artefacts.

---

## 8. References

### Official Documentation
- 📘 [The Rust Book (official free textbook)](https://doc.rust-lang.org/book/) – The single best resource for learning Rust from scratch.
- 📦 [Cargo Documentation](https://doc.rust-lang.org/cargo/) – Everything about Cargo, dependencies, and project structure.
- 🔍 [crates.io](https://crates.io) – The official Rust package registry.
- 📖 [Rust Standard Library Docs](https://doc.rust-lang.org/std/)

### Video Tutorials
- 🎥 [Rust Crash Course – Traversy Media (YouTube)](https://www.youtube.com/watch?v=zF34dRivLOw)
- 🎥 [Introduction to Rust – Jon Gjengset (YouTube)](https://www.youtube.com/c/JonGjengset) – Deep dives for when you're ready to go further.
- 🎥 [Rust for Beginners – freeCodeCamp (YouTube)](https://www.youtube.com/watch?v=MsocPEZBd-M)

### Helpful Blog Posts & Communities
- 📝 [Why Discord switched from Go to Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
- 💬 [r/rust – Reddit Community](https://www.reddit.com/r/rust/)
- 🤝 [Rust Users Forum](https://users.rust-lang.org/)
- 🛠️ [Rust Playground (run Rust in browser, no install needed)](https://play.rust-lang.org/)

### Quick Reference
- ⚡ [Rust by Example](https://doc.rust-lang.org/rust-by-example/) – Learn by reading short runnable examples.
- 🧠 [Rustlings](https://github.com/rust-lang/rustlings) – Small exercises to get you used to reading and writing Rust code.

---

## Appendix – Cargo.toml Reference

Your auto-generated `Cargo.toml` will look like this:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add third-party crates here, e.g.:
# serde = "1.0"
```

- `edition = "2021"` means you're using the 2021 edition of Rust (the latest stable edition). Rust releases new *editions* every few years with improvements — existing code keeps working.
- The `[dependencies]` section is where you list external libraries (called *crates*) from crates.io.

---

*Document prepared as part of the Moringa School Generative AI Capstone Project.*
*Technology: Rust | Format: Beginner Toolkit | Language: Markdown*
