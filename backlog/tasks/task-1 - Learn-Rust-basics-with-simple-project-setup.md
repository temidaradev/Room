---
id: task-1
title: Learn Rust basics with simple project setup
status: Done
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-12'
labels: []
dependencies: []
---

## Description

Start with the most basic Rust concepts by setting up a simple project. Learn about variables, functions, and basic data types.

## Acceptance Criteria

- [ ] Project compiles and runs
- [ ] Shows 'Hello Doom\!' message
- [ ] Understands basic Rust syntax
- [ ] Can explain what main() function does

## Implementation Plan

## Background

Rust is a systems programming language that emphasizes memory safety, performance, and concurrency. Unlike C/C++, Rust prevents common programming errors like null pointer dereferences, buffer overflows, and memory leaks at compile time.

### Key Concepts for This Task:
- Cargo: Rust's package manager and build system
- main(): The entry point of every Rust program
- println\!: A macro for printing to stdout
- Variables: Immutable by default, use mut for mutable
- String literals: Text enclosed in double quotes

### Project Structure:
my_project/
├── Cargo.toml    # Project metadata and dependencies
├── src/
│   └── main.rs   # Main source file

### Example Code:
fn main() {
    // Variables are immutable by default
    let greeting = "Hello";
    let mut target = "Doom";  // mut makes it mutable
    
    // Functions can be called with arguments
    println\!("{} {}\!", greeting, target);
    
    // Basic data types
    let x: i32 = 42;        // 32-bit signed integer
    let pi: f64 = 3.14159;  // 64-bit floating point
    let active: bool = true; // Boolean
}

## Step-by-Step Implementation:

1. Create new project: cargo new room_game && cd room_game
2. Understand Cargo.toml: Contains project metadata and dependencies
3. Edit src/main.rs: Replace default code with our game greeting
4. Build and run: cargo build then cargo run
5. Experiment with basics: Try different variable types and functions
