# 🦀 Hello Rust — Moringa School GenAI Capstone

A beginner-friendly Rust project built as part of the **Moringa School Generative AI Capstone**.
This project demonstrates how to scaffold, compile, and run a minimal Rust program using Cargo.

---

## 📌 What This Project Does

- Prints a personalised greeting to the terminal.
- Demonstrates defining and calling functions in Rust.
- Shows Rust's basic types (`i32`, `&str`) and the `println!` macro.
- Uses Cargo — Rust's official build tool and package manager.

---

## ⚙️ Prerequisites

Make sure you have the following installed before running this project:

| Tool    | Version  | Check Command      |
|---------|----------|--------------------|
| Rust    | ≥ 1.70.0 | `rustc --version`  |
| Cargo   | ≥ 1.70.0 | `cargo --version`  |

**Install Rust (includes Cargo):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then reload your shell:

```bash
source $HOME/.cargo/env
```

> On **Windows**, download the installer from https://rustup.rs

---

## 🚀 How to Run

### 1. Clone the repository

```bash
git clone https://github.com/your-username/hello_rust.git
cd hello_rust
```

### 2. Run with Cargo

```bash
cargo run
```

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

## 🏗️ Project Structure

```
hello_rust/
├── Cargo.toml       # Project metadata and dependency manifest
├── README.md        # You are here
└── src/
    └── main.rs      # Entry point — all source code lives here
```

---

## 🛠️ Other Useful Commands

| Command               | Description                                      |
|-----------------------|--------------------------------------------------|
| `cargo run`           | Compile and run in one step (debug mode)         |
| `cargo build`         | Compile only, output to `target/debug/`          |
| `cargo build --release` | Optimised build, output to `target/release/`  |
| `cargo check`         | Fast type-check without producing a binary       |
| `cargo fmt`           | Auto-format your code                            |
| `cargo clippy`        | Lint your code for common mistakes               |

---

## 📚 Learning Resources

- 📘 [The Rust Book](https://doc.rust-lang.org/book/) — Official free textbook
- ⚡ [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🧠 [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- 🌐 [Rust Playground](https://play.rust-lang.org/) — Run Rust in the browser

---

## 📄 License

MIT — free to use and adapt for learning purposes.

---

*Built with ❤️ and AI assistance as part of the Moringa School Generative AI Capstone.*
