# 🦀 Prompt-Powered Kickstart: A Beginner's Toolkit for Rust

> **Moringa School — Generative AI Capstone Project**
> *Built using AI-assisted learning via Claude (Anthropic)*

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-CE422B?style=flat&logo=rust)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Cargo-1.70%2B-orange?style=flat)](https://doc.rust-lang.org/cargo/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![GenAI Assisted](https://img.shields.io/badge/Built%20with-GenAI-blueviolet)](https://claude.ai)

---

## 📍 Overview

This project is a **beginner-friendly Rust toolkit** created as part of the Moringa School Generative AI Capstone. The goal is to use generative AI (Claude) to learn a new technology — Rust — and document the process clearly enough that any other beginner can replicate it from scratch.

**Technology chosen:** Rust — a systems programming language focused on safety, speed, and concurrency.

**Why Rust?**
- Voted most loved language on Stack Overflow for 8 consecutive years
- Used in production by Microsoft, Google, AWS, Cloudflare, and Mozilla
- Teaches real memory management concepts that apply to every language
- Prevents entire classes of bugs (null pointers, data races) at compile time — not at runtime

**End goal:** Scaffold, compile, and run a minimal Rust program using Cargo that demonstrates functions, basic types, and the `println!` macro — and document every step so others can follow along.

---

## 🧠 Quick Summary of Rust

**What is it?**
Rust is a statically typed, compiled systems programming language created by Mozilla Research (first stable release: 2015). It gives you low-level control like C or C++ while enforcing memory safety through its **ownership system** — without needing a garbage collector.

**Where is it used?**
- Operating systems (parts of the Linux kernel are now written in Rust)
- WebAssembly and high-performance web backends
- Command-line tools (`ripgrep`, `bat`, `fd` are all Rust)
- Embedded systems and IoT
- Blockchain platforms (Solana is built in Rust)
- Game engines and graphics

**Real-world example:**
Cloudflare uses Rust to handle millions of HTTP requests per second in their networking infrastructure. Its zero-cost abstractions allow high-level code that compiles to performance equivalent to hand-optimised C — with far fewer security vulnerabilities.

> 💡 **Key concept — Ownership:** Every value in Rust has a single owner. When the owner goes out of scope, the value is freed. This is how Rust guarantees memory safety at compile time, with no garbage collector.

---

## 📌 What This Project Does

- Prints a personalised greeting to the terminal
- Demonstrates defining and calling functions in Rust
- Shows Rust's basic types (`i32`, `&str`) and the `println!` macro
- Uses **Cargo** — Rust's official build tool and package manager
- Serves as a documented, replicable starting point for any Rust beginner

---

## ⚙️ System Requirements

| Tool | Version | Check Command |
|------|---------|---------------|
| Rust | ≥ 1.70.0 | `rustc --version` |
| Cargo | ≥ 1.70.0 | `cargo --version` |
| VS Code | Any recent | (recommended editor) |
| rust-analyzer | Latest | VS Code extension |

**Supported Operating Systems:**
- Linux (Ubuntu 20.04+, Arch, Fedora, etc.)
- macOS (10.15 Catalina or later)
- Windows 10 / 11 (requires Visual Studio C++ Build Tools)

---

## 🛠️ Installation & Setup

### Step 1 — Install Rust via `rustup`

`rustup` is the official Rust installer and version manager.

**Linux / macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download the installer from [https://rustup.rs](https://rustup.rs)

> On Windows you will also need the **Visual Studio C++ Build Tools**. The rustup installer will prompt you for this.

After installation, reload your shell so `cargo` and `rustc` are available:
```bash
source $HOME/.cargo/env
```

### Step 2 — Verify Installation

```bash
rustc --version   # Expected: rustc 1.7x.x (... date)
cargo --version   # Expected: cargo 1.7x.x (... date)
```

### Step 3 — Clone This Repository

```bash
git clone https://github.com/muriminicholas/hello_rust_capstone.git
cd hello_rust_capstone
```

### Step 4 — (Optional) Install VS Code Extension

Open VS Code → Extensions (`Ctrl+Shift+X`) → search **rust-analyzer** → Install.

This gives you autocomplete, inline errors, and documentation on hover.

---

## 🚀 How to Run

```bash
cargo run
```

That's it. Cargo compiles and runs the program in one step.

### Expected Output

```
   Compiling hello_rust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/hello_rust`
Hello, Moringa Student! Welcome to Rust 🦀
You are now writing systems-level code safely.
-------------------------------------------
Bonus: 10 + 32 = 42
```

---

## 💻 Minimal Working Example

**File:** `src/main.rs`

```rust
// main.rs — Entry point of the Rust program
// The main() function is where execution begins

fn main() {
    // Call our custom greeting function
    greet("Moringa Student");

    // Demonstrate Rust's basic integer type
    let x: i32 = 10;
    let y: i32 = 32;

    // println! is a macro (note the '!') — not a regular function
    println!("-------------------------------------------");
    println!("Bonus: {} + {} = {}", x, y, x + y);
}

// Define a function that takes a string slice (&str) as a parameter
// &str is Rust's immutable string reference type
fn greet(name: &str) {
    println!("Hello, {}! Welcome to Rust 🦀", name);
    println!("You are now writing systems-level code safely.");
}
```

**What each part does:**
| Element | Explanation |
|---------|-------------|
| `fn main()` | Program entry point — runs first |
| `fn greet(name: &str)` | A reusable function taking a string reference |
| `let x: i32` | Declares an integer variable with an explicit type |
| `println!` | A macro (not a function) for formatted terminal output |
| `{}` | Placeholder in `println!` for injecting variable values |

---

## 🏗️ Project Structure

```
hello_rust_capstone/
├── Cargo.toml          # Project metadata and dependency manifest
├── README.md           # You are here
└── src/
    └── main.rs         # Entry point — all source code lives here
```

**About `Cargo.toml`:**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# (none for this minimal project)
```

---

## 🛠️ Other Useful Cargo Commands

| Command | Description |
|---------|-------------|
| `cargo run` | Compile and run in one step (debug mode) |
| `cargo build` | Compile only, output to `target/debug/` |
| `cargo build --release` | Optimised build, output to `target/release/` |
| `cargo check` | Fast type-check without producing a binary |
| `cargo fmt` | Auto-format your code |
| `cargo clippy` | Lint your code for common mistakes |

> 💡 **Tip:** Use `cargo check` while writing code — it catches errors much faster than `cargo build` because it skips generating the binary.

---

## 🤖 AI Prompt Journal

This section documents the AI prompts used during the project, the responses they generated, and honest evaluations of their helpfulness — as required by the Moringa GenAI Capstone rubric.

---

### Prompt #1 — Learning the Basics

**Prompt used:**
> *"I'm a beginner. Help me learn Rust from scratch — what is it, why should I care, and what do I need to get started?"*

**AI response summary:**
Claude gave a concise overview of Rust's core selling points — memory safety, no garbage collector, and the ownership model. It explained why Rust is valuable even if you already know Python or JavaScript, outlined `rustup` as the installation path, and introduced Cargo as the all-in-one build and dependency tool.

**Evaluation:**
Very helpful as a starting point. The explanation of ownership as Rust's central idea helped me frame the learning ahead. The comparison to languages I already knew made the "why" land quickly. ★★★★★

---

### Prompt #2 — Installation Help

**Prompt used:**
> *"Give me step-by-step instructions to install Rust on Linux and verify that it worked."*

**AI response summary:**
Claude provided the full `curl` command for `rustup`, explained what the installer does step by step, showed how to reload the shell with `source $HOME/.cargo/env`, and gave the exact verification commands. It also flagged what to do if `cargo` is not found after install (shell not reloaded).

**Evaluation:**
Extremely clear and practical. Followed the steps exactly and had Rust running in under 5 minutes. The note about reloading the shell saved real time — without it I would have been confused when `cargo` was not found. ★★★★★

---

### Prompt #3 — Understanding Project Structure

**Prompt used:**
> *"How do I create a new Rust project with Cargo, and what does the project structure look like?"*

**AI response summary:**
Claude explained the `cargo new` command, described `Cargo.toml` and why it matters, and broke down `src/main.rs` as the entry point. It also explained how `cargo run` handles compilation and execution in one step, and drew a comparison to `npm` for JavaScript developers.

**Evaluation:**
Great explanation. The `npm` analogy made Cargo immediately understandable. Understanding the project structure upfront saved time when navigating the codebase. ★★★★☆

---

### Prompt #4 — Shaping the Toolkit

**Prompt used:**
> *"Help me build a beginner-friendly Rust toolkit that other learners can use to get started. What should it include and what working example should I build?"*

**AI response summary:**
Claude suggested building a minimal Hello World that goes slightly beyond printing — demonstrating functions, types, and the `println!` macro. It recommended documenting common errors, providing a Cargo command reference, and pointing learners toward The Rust Book and Rustlings for continued learning.

**Evaluation:**
The most productive prompt of the project. It shaped the entire direction of the capstone and gave a clear, achievable scope. The suggestion to document common errors and useful commands significantly improved the quality of the final deliverable. ★★★★★

---

**💬 Reflection on AI-assisted learning:**
Using Claude as a learning companion fundamentally changed how fast I could get up and running. Rather than spending hours reading documentation before writing a single line of code, I could ask targeted questions and get working examples immediately — then verify my understanding by reading the official docs. The AI was most useful for *initial orientation* and *troubleshooting*; the deeper understanding came from actually writing and running the code myself.

---

## 🧪 Testing & Iteration

### Self-Testing Checklist

| Test | Result | Notes |
|------|--------|-------|
| `cargo new` creates project without errors | ✅ PASS | Tested on Ubuntu 22.04 |
| `cargo run` compiles and prints expected output | ✅ PASS | Output matches exactly |
| `cargo check` catches type errors before run | ✅ PASS | Tested by intentionally passing wrong type |
| `cargo build --release` produces optimised binary | ✅ PASS | Binary appears in `target/release/` |
| `cargo fmt` reformats code consistently | ✅ PASS | No diff on already-clean code |
| `cargo clippy` reports no warnings | ✅ PASS | Zero warnings on submission version |

### Peer Testing

The toolkit was shared with a fellow Moringa student (Python background, no prior Rust experience, tested on Windows 11) who followed the guide from scratch without assistance.

**Feedback received & actions taken:**

| Issue Reported | Action Taken |
|----------------|--------------|
| Windows install step didn't mention restarting the terminal after `rustup` | Added explicit restart note to setup steps |
| Difference between `cargo run` and `cargo build` wasn't immediately obvious | Added a one-line explanation of when to use each |
| Ownership concept box felt too brief | Expanded explanation with a scope note |
| `error: linker not found` error hit on Windows — common errors table was useful | No change — confirmed the table is valuable |
| Missing semicolon caused a confusing error message | Added the "unexpected token" row to the errors table |

### Key Learning from Iteration

> Writing for beginners means removing every assumption — even ones that feel too basic to mention. The most valuable feedback came from watching someone follow the guide without any help.

---



| Error / Problem | Likely Cause | Fix |
|----------------|--------------|-----|
| `cargo: command not found` | Shell not reloaded after install | Run: `source $HOME/.cargo/env` |
| `error[E0425]: cannot find value` | Variable used before declaration | Declare with `let` before use |
| `error[E0308]: mismatched types` | Wrong type passed to function | Check types — Rust won't auto-convert |
| `warning: unused variable` | Variable declared but never used | Prefix with underscore: `_my_var` |
| First compile is very slow | Cargo building full dependency tree | Normal — subsequent runs are fast |
| `error: linker cc not found` (Windows) | Missing C++ Build Tools | Install Visual Studio Build Tools |
| `^` pointing at unexpected token | Missing semicolon or closing brace | Check the line *above* the error marker |

> **When reading Rust errors:** Always check the `error[EXXXX]` code — visit `https://doc.rust-lang.org/error_codes/EXXXX.html` for a full explanation with examples. Rust has some of the most helpful compiler errors of any language.

---

## 📚 References

**Official Documentation**
- 📘 [The Rust Book](https://doc.rust-lang.org/book/) — Free official textbook, start here
- ⚡ [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — Learn through annotated examples
- 📦 [Cargo Documentation](https://doc.rust-lang.org/cargo/) — Everything about the build tool
- 🔖 [Standard Library Reference](https://doc.rust-lang.org/std/) — Full API docs

**Interactive Learning**
- 🧠 [Rustlings](https://github.com/rust-lang/rustlings) — Hands-on exercises that fix broken Rust code
- 🌐 [Rust Playground](https://play.rust-lang.org/) — Run Rust in the browser, no install needed
- 🎯 [Exercism — Rust Track](https://exercism.org/tracks/rust) — Mentor-reviewed practice problems

**Community & Help**
- 💬 [Rust Users Forum](https://users.rust-lang.org/) — Friendly, beginner-welcoming community
- 🗨️ [r/rust](https://www.reddit.com/r/rust/) — Active subreddit
- 🔍 [Stack Overflow — rust tag](https://stackoverflow.com/questions/tagged/rust) — Q&A

---

## 📄 License

MIT — free to use and adapt for learning purposes.

---

*Built with ❤️ and AI assistance as part of the Moringa School Generative AI Capstone, April 2026.*
