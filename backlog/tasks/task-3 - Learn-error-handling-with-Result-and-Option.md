---
id: task-3
title: Learn error handling with Result and Option
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn safe error handling in Rust using Result and Option types. This prevents crashes and makes code reliable.

## Acceptance Criteria

- [ ] Uses Result<T E> for functions that can fail
- [ ] Uses Option<T> for values that might not exist
- [ ] Handles errors without using panic!
- [ ] Understands when to use ? operator

## Implementation Plan

## Background

Rust eliminates null pointer exceptions and undefined behavior by using algebraic data types: Result<T, E> for operations that can fail, and Option<T> for values that might not exist. This forces you to handle all error cases explicitly.

### Key Concepts for This Task:
- Result<T, E>: Either Ok(T) for success or Err(E) for failure
- Option<T>: Either Some(T) for a value or None for no value
- Pattern matching: Using match to handle different cases
- The ? operator: Shorthand for early return on errors
- expect() vs unwrap(): Ways to handle errors (avoid in production)

### Error Handling Philosophy:
- Make errors explicit and visible
- Force handling of all error cases
- Prefer returning errors over panicking
- Use types to prevent errors at compile time

### Example Code:
use std::fs::File;
use std::io::{self, Read};

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // ? returns early on error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)  // Wrap success value in Ok
}

fn find_player_by_id(players: &[Player], id: u32) -> Option<&Player> {
    for player in players {
        if player.id == id {
            return Some(player);  // Found it
        }
    }
    None  // Not found
}

fn main() {
    // Handling Result with match
    match read_file_content("game.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Handling Option with if let
    let players = vec![Player { id: 1, name: "Alice".to_string() }];
    if let Some(player) = find_player_by_id(&players, 1) {
        println!("Found player: {}", player.name);
    } else {
        println!("Player not found");
    }
    
    // Using ? operator for chaining
    let result = read_and_process_file("config.txt");
}

fn read_and_process_file(filename: &str) -> Result<(), io::Error> {
    let content = read_file_content(filename)?;  // Propagate error
    println!("Processing: {}", content);
    Ok(())
}

## Step-by-Step Implementation:

1. Create functions that return Result<T, E> for file operations
2. Create functions that return Option<T> for searches
3. Practice pattern matching with match statements
4. Learn the ? operator for error propagation
5. Experiment with map(), and_then(), unwrap_or()
6. Try both expect() and proper error handling
