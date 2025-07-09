---
id: task-4
title: Read binary files safely with byteorder
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn to read binary data from files safely. This is needed for reading WAD files. Learn about bytes, endianness, and safe parsing.

## Acceptance Criteria

- [ ] Reads a simple binary file
- [ ] Understands little-endian vs big-endian
- [ ] Uses byteorder crate correctly
- [ ] Handles file reading errors gracefully

## Implementation Plan

## Background

Binary file parsing is essential for game development as most game assets (textures, models, sounds) are stored in binary formats. WAD files use little-endian byte ordering and contain structured binary data that must be parsed carefully.

### Key Concepts for This Task:
- Binary vs text files: Binary contains raw bytes, not human-readable text
- Endianness: Byte order (little-endian = least significant byte first)
- Structured data: Fixed-size headers, variable-length data
- Cursor-based reading: Reading sequential data with position tracking
- Type safety: Converting raw bytes to typed data safely

### Understanding Endianness:
- Little-endian: 0x12345678 stored as [0x78, 0x56, 0x34, 0x12]
- Big-endian: 0x12345678 stored as [0x12, 0x34, 0x56, 0x78]
- Most modern CPUs use little-endian (x86, ARM)
- Network protocols typically use big-endian

### Example Code:
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Cursor};
use byteorder::{LittleEndian, ReadBytesExt};

fn read_binary_header(filename: &str) -> Result<(u32, u16, u8), io::Error> {
    let mut file = File::open(filename)?;
    
    // Read different types from binary file
    let magic_number = file.read_u32::<LittleEndian>()?;  // 4 bytes
    let version = file.read_u16::<LittleEndian>()?;       // 2 bytes
    let flags = file.read_u8()?;                          // 1 byte
    
    Ok((magic_number, version, flags))
}

fn read_binary_array(filename: &str) -> Result<Vec<i32>, io::Error> {
    let mut file = File::open(filename)?;
    let mut numbers = Vec::new();
    
    // Read the count first
    let count = file.read_u32::<LittleEndian>()? as usize;
    
    // Read that many i32 values
    for _ in 0..count {
        let value = file.read_i32::<LittleEndian>()?;
        numbers.push(value);
    }
    
    Ok(numbers)
}

fn create_test_binary_file() -> Result<(), io::Error> {
    use std::fs::File;
    use std::io::Write;
    use byteorder::{LittleEndian, WriteBytesExt};
    
    let mut file = File::create("test.bin")?;
    
    // Write some test data
    file.write_u32::<LittleEndian>(0x12345678)?;  // Magic number
    file.write_u16::<LittleEndian>(1)?;           // Version
    file.write_u8(0xFF)?;                         // Flags
    
    // Write array
    file.write_u32::<LittleEndian>(3)?;           // Count
    file.write_i32::<LittleEndian>(100)?;
    file.write_i32::<LittleEndian>(-50)?;
    file.write_i32::<LittleEndian>(200)?;
    
    Ok(())
}

## Step-by-Step Implementation:

1. Add byteorder crate to Cargo.toml
2. Create a simple binary file with known data
3. Practice reading different integer types (u8, u16, u32, i32)
4. Learn about cursor-based reading with seek operations
5. Handle endianness conversion properly
6. Practice error handling with file operations
7. Create helper functions for common binary patterns
