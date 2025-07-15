---
id: task-5
title: Create basic WAD file reader
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Build a simple WAD file reader that can list the contents. Learn about file format parsing and data validation.

## Acceptance Criteria

- [ ] Opens a WAD file successfully
- [ ] Reads the header (signature num_lumps directory_offset)
- [ ] Lists all lump names in the file
- [ ] Validates the WAD signature is correct

## Implementation Plan

## Background

WAD (Where's All the Data) files are archive formats used by Doom and other id Software games. They contain game assets like maps, textures, sounds, and sprites. Understanding WAD format is crucial for creating a Doom-like engine.

### WAD File Structure:
1. Header (12 bytes): signature + lump count + directory offset
2. Lumps: Raw data chunks (maps, textures, sounds, etc.)
3. Directory: Array of entries pointing to each lump

### Key Concepts for This Task:
- File format specification: Understanding documented binary formats
- Offset-based file access: Seeking to specific positions
- Data validation: Checking magic numbers and bounds
- String handling: Fixed-width vs null-terminated strings
- Memory layout: How data is organized in files

### WAD Header Format:


### Directory Entry Format:


### Example Code:
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
struct WadHeader {
    signature: String,
    num_lumps: i32,
    directory_offset: i32,
}

#[derive(Debug)]
struct WadLump {
    offset: i32,
    size: i32,
    name: String,
}

fn read_wad_header(file: &mut File) -> Result<WadHeader, io::Error> {
    file.seek(SeekFrom::Start(0))?;
    
    // Read signature (4 bytes)
    let mut sig_bytes = [0; 4];
    file.read_exact(&mut sig_bytes)?;
    let signature = String::from_utf8_lossy(&sig_bytes).to_string();
    
    // Read counts and offsets
    let num_lumps = file.read_i32::<LittleEndian>()?;
    let directory_offset = file.read_i32::<LittleEndian>()?;
    
    Ok(WadHeader {
        signature,
        num_lumps,
        directory_offset,
    })
}

fn read_wad_directory(file: &mut File, header: &WadHeader) -> Result<Vec<WadLump>, io::Error> {
    file.seek(SeekFrom::Start(header.directory_offset as u64))?;
    
    let mut lumps = Vec::new();
    
    for _ in 0..header.num_lumps {
        let offset = file.read_i32::<LittleEndian>()?;
        let size = file.read_i32::<LittleEndian>()?;
        
        // Read name (8 bytes, null-padded)
        let mut name_bytes = [0; 8];
        file.read_exact(&mut name_bytes)?;
        let name = String::from_utf8_lossy(&name_bytes)
            .trim_end_matches('\0')
            .to_string();
        
        lumps.push(WadLump { offset, size, name });
    }
    
    Ok(lumps)
}

fn list_wad_contents(filename: &str) -> Result<(), io::Error> {
    let mut file = File::open(filename)?;
    
    // Read header
    let header = read_wad_header(&mut file)?;
    println\!("WAD Type: {}", header.signature);
    println\!("Lumps: {}", header.num_lumps);
    
    // Validate signature
    if header.signature \!= "IWAD" && header.signature \!= "PWAD" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid WAD signature"));
    }
    
    // Read directory
    let lumps = read_wad_directory(&mut file, &header)?;
    
    println\!("\nLump Directory:");
    for (i, lump) in lumps.iter().enumerate() {
        println\!("{:3}: {:8} (offset: {:8}, size: {:6})", i, lump.name, lump.offset, lump.size);
    }
    
    Ok(())
}

## Step-by-Step Implementation:

1. Create structs for WadHeader and WadLump
2. Implement read_wad_header() function
3. Implement read_wad_directory() function
4. Add signature validation (IWAD vs PWAD)
5. Handle null-terminated strings correctly
6. Create a function to list all lump names
7. Add error handling for corrupted files
8. Test with a real WAD file (shareware DOOM1.WAD)
