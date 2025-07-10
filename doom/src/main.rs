use wad::*;
use std::fs::File;
use std::io::BufReader;



fn main() {
    match read_wad("./game/Doom1.WAD") {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_wad(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let wad = WadFile::load(reader)?;

    for lump in wad.lumps {
        println!("Lump: {} ({} bytes)", lump.name, lump.data.len());
    }

    Ok(())
}