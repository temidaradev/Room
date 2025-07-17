use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};
use wad::WadFile;

pub struct Map {
    pub vertices: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
    pub sidedefs: Vec<Sidedef>,
    pub sectors: Vec<Sector>,
    pub things: Vec<Thing>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone)]
pub struct Linedef {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub flags: u16,
    pub special_type: u16,
    pub sector_tag: u16,
    pub front_sidedef: u16,
    pub back_sidedef: u16,
}

#[derive(Debug, Clone)]
pub struct Sidedef {
    pub x_offset: i16,
    pub y_offset: i16,
    pub upper_texture: String,
    pub lower_texture: String,
    pub middle_texture: String,
    pub sector: u16,
}

#[derive(Debug, Clone)]
pub struct Sector {
    pub floor_height: i16,
    pub ceiling_height: i16,
    pub floor_texture: String,
    pub ceiling_texture: String,
    pub light_level: i16,
    pub special_type: u16,
    pub tag: u16,
}

impl Map {
    pub fn load_from_wad(
        wad: &WadFile,
        map_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Find the map marker lump
        let map_index = wad
            .lumps
            .iter()
            .position(|lump| lump.name == map_name)
            .ok_or("Map not found")?;

        // Map data follows the marker in a specific order
        let vertices = Self::parse_vertices(&wad.lumps[map_index + 4].data)?;
        let linedefs = Self::parse_linedefs(&wad.lumps[map_index + 2].data)?;
        let sidedefs = Self::parse_sidedefs(&wad.lumps[map_index + 3].data)?;
        let sectors = Self::parse_sectors(&wad.lumps[map_index + 8].data)?;
        let things = Self::parse_things(&wad.lumps[map_index + 1].data)?;

        Ok(Map {
            vertices,
            linedefs,
            sidedefs,
            sectors,
            things,
        })
    }

    fn parse_vertices(data: &[u8]) -> Result<Vec<Vertex>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(data);
        let mut vertices = Vec::new();

        while cursor.position() < data.len() as u64 {
            let x = cursor.read_i16::<LittleEndian>()?;
            let y = cursor.read_i16::<LittleEndian>()?;
            vertices.push(Vertex { x, y });
        }

        Ok(vertices)
    }

    fn parse_linedefs(data: &[u8]) -> Result<Vec<Linedef>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(data);
        let mut linedefs = Vec::new();

        while cursor.position() < data.len() as u64 {
            let start_vertex = cursor.read_u16::<LittleEndian>()?;
            let end_vertex = cursor.read_u16::<LittleEndian>()?;
            let flags = cursor.read_u16::<LittleEndian>()?;
            let special_type = cursor.read_u16::<LittleEndian>()?;
            let sector_tag = cursor.read_u16::<LittleEndian>()?;
            let front_sidedef = cursor.read_u16::<LittleEndian>()?;
            let back_sidedef = cursor.read_u16::<LittleEndian>()?;

            linedefs.push(Linedef {
                start_vertex,
                end_vertex,
                flags,
                special_type,
                sector_tag,
                front_sidedef,
                back_sidedef,
            });
        }

        Ok(linedefs)
    }

    // Similar parsing functions for sidedefs, sectors, and things...

    fn parse_sidedefs(data: &[u8]) -> Result<Vec<Sidedef>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(data);
        let mut sidedefs = Vec::new();

        while cursor.position() < data.len() as u64 {
            let x_offset = cursor.read_i16::<LittleEndian>()?;
            let y_offset = cursor.read_i16::<LittleEndian>()?;

            // Read texture names (8 bytes each)
            let mut upper_texture = [0u8; 8];
            let mut lower_texture = [0u8; 8];
            let mut middle_texture = [0u8; 8];

            cursor.read_exact(&mut upper_texture)?;
            cursor.read_exact(&mut lower_texture)?;
            cursor.read_exact(&mut middle_texture)?;

            let sector = cursor.read_u16::<LittleEndian>()?;

            sidedefs.push(Sidedef {
                x_offset,
                y_offset,
                upper_texture: String::from_utf8_lossy(&upper_texture)
                    .trim_end_matches('\0')
                    .to_string(),
                lower_texture: String::from_utf8_lossy(&lower_texture)
                    .trim_end_matches('\0')
                    .to_string(),
                middle_texture: String::from_utf8_lossy(&middle_texture)
                    .trim_end_matches('\0')
                    .to_string(),
                sector,
            });
        }

        Ok(sidedefs)
    }

    fn parse_sectors(data: &[u8]) -> Result<Vec<Sector>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(data);
        let mut sectors = Vec::new();

        while cursor.position() < data.len() as u64 {
            let floor_height = cursor.read_i16::<LittleEndian>()?;
            let ceiling_height = cursor.read_i16::<LittleEndian>()?;

            let mut floor_texture = [0u8; 8];
            let mut ceiling_texture = [0u8; 8];
            cursor.read_exact(&mut floor_texture)?;
            cursor.read_exact(&mut ceiling_texture)?;

            let light_level = cursor.read_i16::<LittleEndian>()?;
            let special_type = cursor.read_u16::<LittleEndian>()?;
            let tag = cursor.read_u16::<LittleEndian>()?;

            sectors.push(Sector {
                floor_height,
                ceiling_height,
                floor_texture: String::from_utf8_lossy(&floor_texture)
                    .trim_end_matches('\0')
                    .to_string(),
                ceiling_texture: String::from_utf8_lossy(&ceiling_texture)
                    .trim_end_matches('\0')
                    .to_string(),
                light_level,
                special_type,
                tag,
            });
        }

        Ok(sectors)
    }

    fn parse_things(data: &[u8]) -> Result<Vec<Thing>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(data);
        let mut things = Vec::new();

        while cursor.position() < data.len() as u64 {
            let x = cursor.read_i16::<LittleEndian>()?;
            let y = cursor.read_i16::<LittleEndian>()?;
            let angle = cursor.read_u16::<LittleEndian>()?;
            let thing_type = cursor.read_u16::<LittleEndian>()?;
            let flags = cursor.read_u16::<LittleEndian>()?;

            things.push(Thing {
                x,
                y,
                angle,
                thing_type,
                flags,
            });
        }

        Ok(things)
    }
}

#[derive(Debug, Clone)]
pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub angle: u16,
    pub thing_type: u16,
    pub flags: u16,
}
