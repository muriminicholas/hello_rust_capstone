// src/main.rs
// Moringa School Capstone – Getting Started with Rust
// A minimal working example demonstrating functions, string slices, and macros.

fn main() {
    // Call our custom greet function with a string literal (&str)
    greet("Moringa Student");

    // Demonstrating variables and basic arithmetic
    let x: i32 = 10;
    let y: i32 = 32;
    println!("Bonus: {} + {} = {}", x, y, add(x, y));
}

/// Accepts a name as a string slice and prints a formatted greeting.
/// &str is a reference to a string — it does not take ownership of the data.
fn greet(name: &str) {
    // println! is a macro (not a function) — notice the ! suffix
    // {} is a placeholder that gets replaced with the value of `name`
    println!("Hello, {}! Welcome to Rust 🦀", name);
    println!("You are now writing systems-level code safely.");
    println!("-------------------------------------------");
}

/// Adds two 32-bit signed integers and returns the result.
/// The last expression in a function body is returned automatically (no `return` keyword needed).
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon here — this is the return expression
}
