---
id: task-6
title: Learn collections with HashMap and Vec
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn Rust's collection types by storing and organizing WAD data. Collections are essential for game data management.

## Acceptance Criteria

- [ ] Uses Vec<T> to store lists of data
- [ ] Uses HashMap<K V> for key-value lookups
- [ ] Understands when to use each collection type
- [ ] Can iterate over collections safely

## Implementation Plan

## Background

Collections are fundamental data structures that store multiple values. Rust's standard library provides several collection types, each optimized for different use cases. Understanding when to use each type is crucial for performance and code clarity.

### Key Concepts for This Task:
- Vec<T>: Dynamic arrays that can grow and shrink
- HashMap<K, V>: Hash tables for fast key-value lookups
- Ownership with collections: How borrowing works with containers
- Iteration: Different ways to loop through collections
- Memory management: How collections handle allocation

### Collection Types Overview:
- Vec<T>: Ordered, indexed access, dynamic size
- HashMap<K, V>: Unordered, key-based access, unique keys
- HashSet<T>: Unordered, unique values only
- VecDeque<T>: Double-ended queue, efficient front/back operations
- BTreeMap<K, V>: Ordered map, sorted keys

### Memory Layout:
- Vec: Contiguous memory block on heap
- HashMap: Hash table with buckets and collision handling
- Both use capacity vs length (capacity >= length)

### Example Code:
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct WadLump {
    name: String,
    offset: i32,
    size: i32,
    data: Vec<u8>,
}

fn demonstrate_collections() {
    // Vec: Dynamic arrays
    let mut lumps = Vec::new();
    lumps.push(WadLump {
        name: "E1M1".to_string(),
        offset: 1000,
        size: 512,
        data: vec\![0; 512],
    });
    
    // Vec with capacity (performance optimization)
    let mut vertices = Vec::with_capacity(100);
    vertices.push((10, 20));
    vertices.push((30, 40));
    
    // HashMap: Key-value storage
    let mut lump_index = HashMap::new();
    lump_index.insert("E1M1".to_string(), 0);
    lump_index.insert("THINGS".to_string(), 1);
    
    // Accessing collections
    if let Some(lump) = lumps.get(0) {
        println\!("First lump: {:?}", lump.name);
    }
    
    if let Some(&index) = lump_index.get("E1M1") {
        println\!("E1M1 is at index {}", index);
    }
    
    // Iteration patterns
    for (i, lump) in lumps.iter().enumerate() {
        println\!("Lump {}: {}", i, lump.name);
    }
    
    for (name, index) in &lump_index {
        println\!("Lump {} at index {}", name, index);
    }
    
    // Mutable iteration
    for lump in lumps.iter_mut() {
        lump.offset += 100;  // Adjust all offsets
    }
    
    // Functional programming style
    let large_lumps: Vec<_> = lumps
        .iter()
        .filter(|lump| lump.size > 256)
        .collect();
    
    println\!("Found {} large lumps", large_lumps.len());
}

fn wad_lump_manager() {
    let mut lumps = Vec::new();
    let mut name_to_index = HashMap::new();
    
    // Adding lumps
    let lump = WadLump {
        name: "PLAYPAL".to_string(),
        offset: 2000,
        size: 768,
        data: vec\![0; 768],
    };
    
    let index = lumps.len();
    name_to_index.insert(lump.name.clone(), index);
    lumps.push(lump);
    
    // Fast lookup by name
    if let Some(&index) = name_to_index.get("PLAYPAL") {
        let lump = &lumps[index];
        println\!("Found PLAYPAL at offset {}", lump.offset);
    }
}

## Step-by-Step Implementation:

1. Create a Vec to store WAD lumps
2. Use HashMap to create a name-to-index mapping
3. Practice different ways to add items (push, insert, extend)
4. Learn different iteration patterns (for, while, iterators)
5. Understand borrowing rules with collections
6. Use functional programming methods (map, filter, fold)
7. Handle common errors (index out of bounds, key not found)
8. Compare performance characteristics of different approaches
