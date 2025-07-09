---
id: task-7
title: Parse map vertices from WAD file
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn to parse structured binary data by reading map vertices. This introduces parsing patterns and data validation.

## Acceptance Criteria

- [ ] Reads VERTICES lump from WAD file
- [ ] Parses x y coordinates as i16 values
- [ ] Stores vertices in a Vec<Vertex>
- [ ] Validates data length is correct

## Implementation Plan

## Background

Binary data parsing is a core skill in game development. Maps in Doom WAD files store geometry as arrays of structured data. Vertices are the simplest example - just x,y coordinates stored as signed 16-bit integers.

### Key Concepts for This Task:
- Structured binary data: Fixed-size records in sequence
- Data validation: Checking sizes and bounds
- Type conversion: From raw bytes to meaningful data
- Error handling: Graceful handling of malformed data
- Memory efficiency: Reading large datasets efficiently

### VERTICES Lump Format:
Each vertex is 4 bytes:
- 2 bytes: x coordinate (signed 16-bit integer, little-endian)
- 2 bytes: y coordinate (signed 16-bit integer, little-endian)

### Doom Coordinate System:
- Origin (0,0) is at map center
- X increases to the right
- Y increases upward
- Units are "map units" (roughly 1 unit = 1 pixel at 100% zoom)
- Typical map ranges: -32768 to +32767 in both directions

### Example Code:
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: i16,
    y: i16,
}

impl Vertex {
    fn new(x: i16, y: i16) -> Self {
        Vertex { x, y }
    }
    
    fn distance_to(&self, other: &Vertex) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

fn find_vertices_lump(lumps: &[WadLump]) -> Option<&WadLump> {
    lumps.iter().find(|lump| lump.name == "VERTEXES")
}

fn parse_vertices(data: &[u8]) -> Result<Vec<Vertex>, io::Error> {
    // Each vertex is 4 bytes (2 i16 values)
    if data.len() % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Vertices data length must be multiple of 4"
        ));
    }
    
    let vertex_count = data.len() / 4;
    let mut vertices = Vec::with_capacity(vertex_count);
    
    let mut cursor = std::io::Cursor::new(data);
    
    for _ in 0..vertex_count {
        let x = cursor.read_i16::<LittleEndian>()?;
        let y = cursor.read_i16::<LittleEndian>()?;
        vertices.push(Vertex::new(x, y));
    }
    
    Ok(vertices)
}

fn read_vertices_from_wad(filename: &str) -> Result<Vec<Vertex>, io::Error> {
    let mut file = File::open(filename)?;
    
    // Read WAD header and directory (from previous task)
    let header = read_wad_header(&mut file)?;
    let lumps = read_wad_directory(&mut file, &header)?;
    
    // Find the VERTEXES lump
    let vertices_lump = find_vertices_lump(&lumps)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "VERTEXES lump not found"))?;
    
    // Read the lump data
    file.seek(SeekFrom::Start(vertices_lump.offset as u64))?;
    let mut data = vec![0u8; vertices_lump.size as usize];
    file.read_exact(&mut data)?;
    
    // Parse vertices
    parse_vertices(&data)
}

fn analyze_vertices(vertices: &[Vertex]) {
    if vertices.is_empty() {
        println!("No vertices found");
        return;
    }
    
    println!("Found {} vertices", vertices.len());
    
    // Find bounding box
    let min_x = vertices.iter().map(|v| v.x).min().unwrap();
    let max_x = vertices.iter().map(|v| v.x).max().unwrap();
    let min_y = vertices.iter().map(|v| v.y).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y).max().unwrap();
    
    println!("Bounding box: ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);
    println!("Map dimensions: {} x {}", max_x - min_x, max_y - min_y);
    
    // Show first few vertices
    for (i, vertex) in vertices.iter().take(5).enumerate() {
        println!("Vertex {}: ({}, {})", i, vertex.x, vertex.y);
    }
}

fn main() -> Result<(), io::Error> {
    let vertices = read_vertices_from_wad("doom.wad")?;
    analyze_vertices(&vertices);
    Ok(())
}

## Step-by-Step Implementation:

1. Create a Vertex struct with x, y fields (i16)
2. Add methods for distance calculation and display
3. Write a function to parse vertices from binary data
4. Add data validation (check size is multiple of 4)
5. Find the VERTEXES lump in the WAD directory
6. Read the lump data and parse all vertices
7. Create analysis functions (bounding box, statistics)
8. Handle errors gracefully (missing lump, invalid data)
9. Test with different WAD files to verify parsing
