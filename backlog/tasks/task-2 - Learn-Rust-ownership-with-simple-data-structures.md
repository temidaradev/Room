---
id: task-2
title: Learn Rust ownership with simple data structures
status: Done
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-12'
labels: []
dependencies: []
---

## Description

Understand ownership, borrowing, and lifetimes through creating simple game data structures. This is the most important Rust concept.

## Acceptance Criteria

- [ ] Creates a simple Player struct
- [ ] Understands ownership rules
- [ ] Can pass data between functions safely
- [ ] Explains the difference between move and borrow

## Implementation Plan

## Background

Ownership is Rust's most unique feature that ensures memory safety without garbage collection. It prevents dangling pointers, memory leaks, and data races at compile time through three simple rules:

1. Each value has a single owner
2. When the owner goes out of scope, the value is dropped
3. There can only be one owner at a time

### Key Concepts for This Task:
- Ownership: Who is responsible for cleaning up memory
- Borrowing: Temporary access to data without taking ownership
- References: Pointers that don't own the data (&T for immutable, &mut T for mutable)
- Move semantics: Transferring ownership between variables
- Clone: Creating a deep copy of data

### Stack vs Heap:
- Stack: Fast, fixed-size data (integers, booleans, structs with known size)
- Heap: Dynamic data (String, Vec, Box) that needs allocation

### Example Code:
struct Player {
    x: f32,
    y: f32,
    health: i32,
}

fn main() {
    let mut player = Player { x: 0.0, y: 0.0, health: 100 };
    print_player(&player);
    move_player(&mut player, 10.0, 5.0);
    let other_player = player;
    let cloned_player = other_player.clone();
}

fn print_player(p: &Player) {
    println\!("Player at ({}, {}) with {} health", p.x, p.y, p.health);
}

fn move_player(p: &mut Player, dx: f32, dy: f32) {
    p.x += dx;
    p.y += dy;
}

## Step-by-Step Implementation:

1. Create Player struct with derive(Debug, Clone)
2. Practice creating instances and accessing fields
3. Write functions that take references (&Player)
4. Write functions that take mutable references (&mut Player)
5. Experiment with moves, borrows, and clones
6. Try breaking the borrow checker to understand the rules
