use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WadError {
    #[error("Invalid WAD signature")]
    InvalidSignature,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid lump name")]
    InvalidLumpName,
}

pub struct WadFile {
    pub lumps: Vec<WadLump>,
}

pub struct WadLump {
    pub name: String,
    pub data: Vec<u8>,
}

impl WadFile {
    pub fn load<R: Read + Seek>(mut reader: R) -> Result<Self, WadError> {
        // Read the 4-byte signature ("IWAD" or "PWAD")
        let mut signature = [0u8; 4];
        reader.read_exact(&mut signature)?;

        if &signature != b"IWAD" && &signature != b"PWAD" {
            return Err(WadError::InvalidSignature);
        }

        // Read number of lumps and directory offset
        let num_lumps = reader.read_u32::<LittleEndian>()?;
        let dir_offset = reader.read_u32::<LittleEndian>()?;

        // Seek to directory and read lump entries
        reader.seek(SeekFrom::Start(dir_offset as u64))?;

        let mut lumps = Vec::new();
        for _ in 0..num_lumps {
            let lump_offset = reader.read_u32::<LittleEndian>()?;
            let lump_size = reader.read_u32::<LittleEndian>()?;

            // Read 8-byte null-terminated name
            let mut name_bytes = [0u8; 8];
            reader.read_exact(&mut name_bytes)?;

            // Convert to string, stopping at first null byte
            let name = String::from_utf8_lossy(&name_bytes)
                .trim_end_matches('\0')
                .to_string();

            // Read lump data
            let current_pos = reader.stream_position()?;
            reader.seek(SeekFrom::Start(lump_offset as u64))?;

            let mut data = vec![0u8; lump_size as usize];
            reader.read_exact(&mut data)?;

            // Return to directory position
            reader.seek(SeekFrom::Start(current_pos))?;

            lumps.push(WadLump { name, data });
        }

        Ok(WadFile { lumps })
    }

    pub fn find_lump(&self, name: &str) -> Option<&WadLump> {
        self.lumps.iter().find(|lump| lump.name == name)
    }
}
