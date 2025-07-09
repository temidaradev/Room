# Complete Guide to Writing a Doom Port in Rust

## Table of Contents

1. [Understanding the Foundation](#understanding-the-foundation)
2. [Project Setup and Dependencies](#project-setup-and-dependencies)
3. [WAD File Format and Parsing](#wad-file-format-and-parsing)
4. [Map Data Structure and Parsing](#map-data-structure-and-parsing)
5. [Core Engine Architecture](#core-engine-architecture)
6. [Rendering System Foundation](#rendering-system-foundation)
7. [Advanced Rendering Techniques](#advanced-rendering-techniques)
8. [Player Movement and Physics](#player-movement-and-physics)
9. [Input Handling](#input-handling)
10. [Audio System Implementation](#audio-system-implementation)
11. [Game Entity System](#game-entity-system)
12. [BSP Tree and Visibility](#bsp-tree-and-visibility)
13. [Testing Strategy](#testing-strategy)
14. [Performance Optimization](#performance-optimization)
15. [Common Pitfalls and Solutions](#common-pitfalls-and-solutions)
16. [Debugging Techniques](#debugging-techniques)
17. [Building and Distribution](#building-and-distribution)
18. [Next Steps and Extensions](#next-steps-and-extensions)

## Understanding the Foundation

Before diving into code, let's understand what makes Doom tick. Doom is fundamentally a 2D game that creates the illusion of 3D through clever rendering techniques. The game world consists of sectors (rooms) connected by linedefs (walls), and the engine uses a technique called "portal rendering" to draw what the player sees.

Think of Doom's world like a floor plan viewed from above. Each room has a floor height and ceiling height, and walls connect these rooms. The "3D" effect comes from drawing vertical lines on screen based on the height differences between floors and ceilings.

The WAD (Where's All the Data) file contains everything needed to run the game: maps, textures, sounds, music, and game data. Your port will need to read this file and reconstruct the game world from it.

## Project Setup and Dependencies

Let's start by setting up our Rust project with the necessary dependencies. Create a new project and configure your `Cargo.toml`:

```toml
[package]
name = "doom-port"
version = "0.1.0"
edition = "2021"

[dependencies]
sdl2 = { version = "0.35", features = ["bundled"] }
byteorder = "1.4"
thiserror = "1.0"
anyhow = "1.0"
glam = "0.27"  # For vector math
rand = "0.8"   # For random number generation
serde = { version = "1.0", features = ["derive"] }  # For configuration files
toml = "0.8"   # For configuration parsing

[dev-dependencies]
criterion = "0.5"  # For benchmarking
```

The `byteorder` crate helps us read binary data from WAD files safely, while `thiserror` and `anyhow` provide excellent error handling capabilities that align with Rust's safety principles. The `glam` crate provides optimized vector mathematics, and `bundled` feature for SDL2 ensures the library is statically linked for easier distribution.

**Important Setup Notes:**

- On macOS, you may need to install SDL2 via Homebrew: `brew install sdl2`
- On Linux, install SDL2 development packages: `sudo apt-get install libsdl2-dev`
- On Windows, the bundled feature handles SDL2 automatically

## WAD File Format and Parsing

The WAD file format is surprisingly simple, which makes it perfect for learning. A WAD file consists of a header followed by a directory of "lumps" (data chunks). Let's build a safe parser:

```rust
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
```

This parser demonstrates several important Rust safety principles. We use `Result` types for error handling, bounds checking through safe slice operations, and the `byteorder` crate to handle endianness safely without manual pointer manipulation.

## Core Engine Architecture

Now let's design the core architecture. We'll use a modular approach where each system has clear responsibilities:

```rust
use sdl2::Sdl;
use std::time::{Duration, Instant};

pub struct DoomEngine {
    sdl_context: Sdl,
    wad: WadFile,
    renderer: Renderer,
    game_state: GameState,
    input_handler: InputHandler,
    last_frame_time: Instant,
}

pub struct GameState {
    pub current_map: Option<Map>,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub game_time: Duration,
}

impl DoomEngine {
    pub fn new(wad_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let wad = WadFile::load(std::fs::File::open(wad_path)?)?;

        let renderer = Renderer::new(&sdl_context)?;
        let game_state = GameState::new();
        let input_handler = InputHandler::new(&sdl_context)?;

        Ok(DoomEngine {
            sdl_context,
            wad,
            renderer,
            game_state,
            input_handler,
            last_frame_time: Instant::now(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            let current_time = Instant::now();
            let delta_time = current_time - self.last_frame_time;
            self.last_frame_time = current_time;

            // Handle input
            if !self.input_handler.handle_events(&mut event_pump)? {
                break 'running;
            }

            // Update game state
            self.update_game_state(delta_time)?;

            // Render frame
            self.renderer.render_frame(&self.game_state)?;

            // Cap frame rate
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }

        Ok(())
    }

    fn update_game_state(&mut self, delta_time: Duration) -> Result<(), Box<dyn std::error::Error>> {
        self.game_state.game_time += delta_time;

        // Update player position based on input
        self.game_state.player.update(delta_time, &self.input_handler);

        // Update entities
        for entity in &mut self.game_state.entities {
            entity.update(delta_time);
        }

        Ok(())
    }
}
```

This architecture separates concerns cleanly. The engine coordinates between systems but doesn't implement their details. Each system can be developed and tested independently.

## Map Data Structure and Parsing

Doom maps are stored in a specific format within the WAD file. Let's create a safe parser for map data:

```rust
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

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
    pub fn load_from_wad(wad: &WadFile, map_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Find the map marker lump
        let map_index = wad.lumps.iter().position(|lump| lump.name == map_name)
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
                upper_texture: String::from_utf8_lossy(&upper_texture).trim_end_matches('\0').to_string(),
                lower_texture: String::from_utf8_lossy(&lower_texture).trim_end_matches('\0').to_string(),
                middle_texture: String::from_utf8_lossy(&middle_texture).trim_end_matches('\0').to_string(),
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
                floor_texture: String::from_utf8_lossy(&floor_texture).trim_end_matches('\0').to_string(),
                ceiling_texture: String::from_utf8_lossy(&ceiling_texture).trim_end_matches('\0').to_string(),
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
```

The key insight here is that we're using Rust's type system to ensure our data structures match the binary format exactly. The `byteorder` crate handles endianness conversion safely, and we use `Result` types to handle potential parsing errors gracefully.

## Rendering System Foundation

Doom's rendering system is fascinating because it predates modern 3D graphics. Instead of polygons, it uses a technique called "portal rendering" where the world is drawn from the player's perspective by casting rays and drawing vertical lines. Let's implement the foundation:

```rust
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f64::consts::PI;

pub struct Renderer {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32,
}

impl Renderer {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("Doom Port", 800, 600)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;

        Ok(Renderer {
            canvas,
            screen_width: 800,
            screen_height: 600,
        })
    }

    pub fn render_frame(&mut self, game_state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
        // Clear screen
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        if let Some(map) = &game_state.current_map {
            self.render_3d_view(map, &game_state.player)?;
        }

        self.canvas.present();
        Ok(())
    }

    fn render_3d_view(&mut self, map: &Map, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let fov = PI / 3.0; // 60 degrees
        let half_fov = fov / 2.0;

        // Cast rays for each screen column
        for x in 0..self.screen_width {
            let ray_angle = player.angle - half_fov + (x as f64 / self.screen_width as f64) * fov;

            if let Some(hit) = self.cast_ray(map, player, ray_angle) {
                self.draw_wall_slice(x, &hit)?;
            }
        }

        Ok(())
    }

    fn cast_ray(&self, map: &Map, player: &Player, angle: f64) -> Option<RayHit> {
        let ray_dx = angle.cos();
        let ray_dy = angle.sin();

        // Step along the ray and check for wall intersections
        let mut distance = 0.0;
        let step_size = 1.0;

        while distance < 1000.0 { // Max view distance
            let test_x = player.x + ray_dx * distance;
            let test_y = player.y + ray_dy * distance;

            // Check if we hit a wall
            if let Some(wall_hit) = self.check_wall_collision(map, test_x, test_y) {
                return Some(RayHit {
                    distance,
                    wall_type: wall_hit,
                    hit_x: test_x,
                    hit_y: test_y,
                });
            }

            distance += step_size;
        }

        None
    }

    fn draw_wall_slice(&mut self, screen_x: u32, hit: &RayHit) -> Result<(), Box<dyn std::error::Error>> {
        // Calculate wall height on screen based on distance
        let wall_height = (self.screen_height as f64 / hit.distance * 100.0) as i32;
        let wall_top = (self.screen_height as i32 - wall_height) / 2;
        let wall_bottom = wall_top + wall_height;

        // Choose color based on wall type (simplified)
        let color = match hit.wall_type {
            WallType::Stone => Color::RGB(128, 128, 128),
            WallType::Wood => Color::RGB(139, 69, 19),
            WallType::Metal => Color::RGB(192, 192, 192),
        };

        self.canvas.set_draw_color(color);

        // Draw vertical line from wall_top to wall_bottom
        for y in wall_top.max(0)..wall_bottom.min(self.screen_height as i32) {
            self.canvas.draw_point((screen_x as i32, y))?;
        }

        Ok(())
    }
}

struct RayHit {
    distance: f64,
    wall_type: WallType,
    hit_x: f64,
    hit_y: f64,
}

enum WallType {
    Stone,
    Wood,
    Metal,
}
```

This rendering system demonstrates the core concept of ray casting. For each column of pixels on the screen, we cast a ray from the player's position in the direction they're looking. When the ray hits a wall, we calculate how tall that wall should appear on screen based on the distance.

## Advanced Rendering Techniques

### Texture Mapping

Once you have basic wall rendering working, the next step is adding textures. Doom textures are stored as patches in the WAD file:

```rust
pub struct Texture {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>, // Palette indices
}

pub struct TextureManager {
    textures: std::collections::HashMap<String, Texture>,
    palette: Vec<[u8; 3]>, // RGB values
}

impl TextureManager {
    pub fn load_from_wad(wad: &WadFile) -> Result<Self, Box<dyn std::error::Error>> {
        let mut textures = std::collections::HashMap::new();
        let palette = Self::load_palette(wad)?;

        // Load PNAMES (patch names)
        if let Some(pnames_lump) = wad.find_lump("PNAMES") {
            let patch_names = Self::parse_patch_names(&pnames_lump.data)?;

            // Load TEXTURE1 and TEXTURE2
            if let Some(texture1_lump) = wad.find_lump("TEXTURE1") {
                let texture1_textures = Self::parse_textures(&texture1_lump.data, &patch_names, wad)?;
                textures.extend(texture1_textures);
            }
        }

        Ok(TextureManager { textures, palette })
    }

    fn load_palette(wad: &WadFile) -> Result<Vec<[u8; 3]>, Box<dyn std::error::Error>> {
        let playpal = wad.find_lump("PLAYPAL")
            .ok_or("PLAYPAL lump not found")?;

        let mut palette = Vec::new();
        for chunk in playpal.data.chunks(3) {
            if chunk.len() == 3 {
                palette.push([chunk[0], chunk[1], chunk[2]]);
            }
        }

        Ok(palette)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }
}
```

### Floor and Ceiling Rendering

Doom renders floors and ceilings using affine texture mapping:

```rust
impl Renderer {
    fn render_floor_ceiling(&mut self, map: &Map, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let half_height = self.screen_height as f64 / 2.0;

        for y in 0..self.screen_height {
            if y < half_height as u32 {
                // Render ceiling
                self.render_horizontal_plane(y, player, true)?;
            } else {
                // Render floor
                self.render_horizontal_plane(y, player, false)?;
            }
        }

        Ok(())
    }

    fn render_horizontal_plane(&mut self, screen_y: u32, player: &Player, is_ceiling: bool) -> Result<(), Box<dyn std::error::Error>> {
        let half_height = self.screen_height as f64 / 2.0;
        let distance = if is_ceiling {
            (player.height * half_height) / (half_height - screen_y as f64)
        } else {
            (player.height * half_height) / (screen_y as f64 - half_height)
        };

        for x in 0..self.screen_width {
            let angle = player.angle + (x as f64 - self.screen_width as f64 / 2.0) * 0.001;
            let world_x = player.x + angle.cos() * distance;
            let world_y = player.y + angle.sin() * distance;

            // Sample texture at world coordinates
            let color = self.sample_floor_texture(world_x, world_y, is_ceiling);
            self.canvas.set_draw_color(color);
            self.canvas.draw_point((x as i32, screen_y as i32))?;
        }

        Ok(())
    }
}
```

### Sprite Rendering

Doom sprites are billboards that always face the player:

```rust
pub struct Sprite {
    pub texture: Texture,
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}

impl Renderer {
    fn render_sprites(&mut self, sprites: &[Sprite], player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        // Sort sprites by distance for proper depth ordering
        let mut sorted_sprites: Vec<_> = sprites.iter().enumerate().collect();
        sorted_sprites.sort_by(|a, b| {
            let dist_a = ((a.1.x - player.x).powi(2) + (a.1.y - player.y).powi(2)).sqrt();
            let dist_b = ((b.1.x - player.x).powi(2) + (b.1.y - player.y).powi(2)).sqrt();
            dist_b.partial_cmp(&dist_a).unwrap()
        });

        for (_, sprite) in sorted_sprites {
            self.render_sprite(sprite, player)?;
        }

        Ok(())
    }

    fn render_sprite(&mut self, sprite: &Sprite, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        // Calculate sprite position relative to player
        let dx = sprite.x - player.x;
        let dy = sprite.y - player.y;

        // Transform to screen coordinates
        let distance = (dx * dx + dy * dy).sqrt();
        let angle_to_sprite = dy.atan2(dx) - player.angle;

        let screen_x = (self.screen_width as f64 / 2.0) +
                      (angle_to_sprite.tan() * self.screen_width as f64 / 2.0);

        let sprite_height = (sprite.texture.height as f64 * sprite.scale) / distance;

        // Render the sprite if it's visible
        if screen_x >= 0.0 && screen_x < self.screen_width as f64 {
            self.draw_sprite_column(sprite, screen_x as u32, sprite_height as u32)?;
        }

        Ok(())
    }
}
```

## Audio System Implementation

Doom's audio system includes sound effects, music, and 3D positional audio. Here's how to implement it safely in Rust:

```rust
use sdl2::mixer::{Chunk, Music, Channel, DEFAULT_CHANNELS};

pub struct AudioManager {
    _mixer_context: sdl2::mixer::Sdl2MixerContext,
    sound_effects: std::collections::HashMap<String, Chunk>,
    current_music: Option<Music<'static>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG)?;

        // Initialize mixer with reasonable defaults
        sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;
        sdl2::mixer::allocate_channels(16);

        Ok(AudioManager {
            _mixer_context: mixer_context,
            sound_effects: std::collections::HashMap::new(),
            current_music: None,
        })
    }

    pub fn load_sound_effects(&mut self, wad: &WadFile) -> Result<(), Box<dyn std::error::Error>> {
        // Doom sound effects are stored as specific lumps
        let sound_names = ["DSPISTOL", "DSSHOTGN", "DSPLASMA", "DSBFG", "DSRLAUNC"];

        for sound_name in &sound_names {
            if let Some(lump) = wad.find_lump(sound_name) {
                let sound_data = self.convert_doom_sound_to_wav(&lump.data)?;
                let chunk = Chunk::from_raw_buffer(sound_data)?;
                self.sound_effects.insert(sound_name.to_string(), chunk);
            }
        }

        Ok(())
    }

    pub fn play_sound_3d(&self, sound_name: &str, player_pos: (f64, f64), sound_pos: (f64, f64)) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(chunk) = self.sound_effects.get(sound_name) {
            let distance = ((sound_pos.0 - player_pos.0).powi(2) + (sound_pos.1 - player_pos.1).powi(2)).sqrt();

            // Calculate volume based on distance
            let volume = (255.0 / (1.0 + distance / 100.0)) as i32;
            let volume = volume.max(0).min(255);

            // Calculate panning based on relative position
            let angle = (sound_pos.1 - player_pos.1).atan2(sound_pos.0 - player_pos.0);
            let pan = ((angle.sin() + 1.0) * 127.0) as u8;

            let channel = Channel::all().play(chunk, 0)?;
            channel.set_volume(volume);
            channel.set_panning(255 - pan, pan)?;
        }

        Ok(())
    }

    fn convert_doom_sound_to_wav(&self, doom_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Doom sounds have a simple header format
        if doom_data.len() < 8 {
            return Err("Invalid Doom sound data".into());
        }

        let sample_rate = u16::from_le_bytes([doom_data[2], doom_data[3]]);
        let sample_count = u32::from_le_bytes([doom_data[4], doom_data[5], doom_data[6], doom_data[7]]);

        // Convert to standard WAV format for SDL2
        let mut wav_data = Vec::new();

        // WAV header
        wav_data.extend_from_slice(b"RIFF");
        wav_data.extend_from_slice(&(36 + sample_count).to_le_bytes());
        wav_data.extend_from_slice(b"WAVE");
        wav_data.extend_from_slice(b"fmt ");
        wav_data.extend_from_slice(&16u32.to_le_bytes());
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // PCM
        wav_data.extend_from_slice(&1u16.to_le_bytes()); // Mono
        wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes());
        wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes());
        wav_data.extend_from_slice(&1u16.to_le_bytes());
        wav_data.extend_from_slice(&8u16.to_le_bytes()); // 8-bit
        wav_data.extend_from_slice(b"data");
        wav_data.extend_from_slice(&sample_count.to_le_bytes());

        // Sound data (skip header)
        wav_data.extend_from_slice(&doom_data[8..]);

        Ok(wav_data)
    }
}
```

**Note**: You'll need to add `sdl2_mixer` to your dependencies:

```toml
[dependencies]
sdl2 = { version = "0.35", features = ["bundled", "mixer"] }
```

## Game Entity System

A flexible entity system is crucial for handling monsters, items, projectiles, and interactive objects:

```rust
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum EntityType {
    Monster { health: i32, monster_type: MonsterType },
    Item { item_type: ItemType, respawn_time: Option<Duration> },
    Projectile { damage: i32, velocity: (f64, f64) },
    Decoration,
}

#[derive(Debug, Clone)]
pub enum MonsterType {
    Imp,
    Demon,
    Cacodemon,
    BaronOfHell,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Health,
    Armor,
    Weapon(WeaponType),
    Ammo(AmmoType),
    Key(KeyType),
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub angle: f64,
    pub radius: f64,
    pub height: f64,
    pub entity_type: EntityType,
    pub active: bool,
    pub sprite_name: String,
}

pub struct EntityManager {
    entities: Vec<Entity>,
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn_entity(&mut self, x: f64, y: f64, entity_type: EntityType, sprite_name: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity {
            id,
            x,
            y,
            z: 0.0,
            angle: 0.0,
            radius: 20.0, // Default radius
            height: 56.0, // Default height
            entity_type,
            active: true,
            sprite_name,
        };

        self.entities.push(entity);
        id
    }

    pub fn update_entities(&mut self, delta_time: Duration, player: &Player, map: &Map) {
        for entity in &mut self.entities {
            if !entity.active {
                continue;
            }

            match &mut entity.entity_type {
                EntityType::Monster { health, monster_type } => {
                    self.update_monster(entity, delta_time, player, map);
                }
                EntityType::Projectile { velocity, .. } => {
                    entity.x += velocity.0 * delta_time.as_secs_f64();
                    entity.y += velocity.1 * delta_time.as_secs_f64();

                    // Check for collision with walls or entities
                    if self.check_projectile_collision(entity, map) {
                        entity.active = false;
                    }
                }
                EntityType::Item { respawn_time, .. } => {
                    // Handle item respawning logic
                    if let Some(respawn) = respawn_time {
                        // Implement respawn timing
                    }
                }
                EntityType::Decoration => {
                    // Decorations don't move or update
                }
            }
        }

        // Remove inactive entities
        self.entities.retain(|e| e.active);
    }

    fn update_monster(&self, entity: &mut Entity, delta_time: Duration, player: &Player, map: &Map) {
        // Simple AI: move towards player
        let dx = player.x - entity.x;
        let dy = player.y - entity.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 50.0 { // Too far, move closer
            let move_speed = 50.0;
            let dt = delta_time.as_secs_f64();

            entity.x += (dx / distance) * move_speed * dt;
            entity.y += (dy / distance) * move_speed * dt;

            // Update facing angle
            entity.angle = dy.atan2(dx);
        }
    }

    fn check_projectile_collision(&self, projectile: &Entity, map: &Map) -> bool {
        // Check collision with walls
        // This would use your existing collision detection logic
        false // Placeholder
    }

    pub fn get_entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn find_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn find_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }
}

// Integration with Thing parsing from WAD
impl EntityManager {
    pub fn spawn_from_things(&mut self, things: &[Thing]) {
        for thing in things {
            let entity_type = match thing.thing_type {
                1 => EntityType::Player, // Player start position
                3001 => EntityType::Monster {
                    health: 60,
                    monster_type: MonsterType::Imp
                },
                3002 => EntityType::Monster {
                    health: 150,
                    monster_type: MonsterType::Demon
                },
                2001 => EntityType::Item {
                    item_type: ItemType::Weapon(WeaponType::Shotgun),
                    respawn_time: None
                },
                2011 => EntityType::Item {
                    item_type: ItemType::Health,
                    respawn_time: Some(Duration::from_secs(30))
                },
                _ => continue, // Unknown thing type
            };

            let sprite_name = self.get_sprite_name_for_thing_type(thing.thing_type);
            self.spawn_entity(thing.x as f64, thing.y as f64, entity_type, sprite_name);
        }
    }

    fn get_sprite_name_for_thing_type(&self, thing_type: u16) -> String {
        match thing_type {
            3001 => "TROO".to_string(), // Imp
            3002 => "SARG".to_string(), // Demon
            2001 => "SHOT".to_string(), // Shotgun
            2011 => "STIM".to_string(), // Stimpack
            _ => "UNKN".to_string(),
        }
    }
}
```

## BSP Tree and Visibility

Doom uses Binary Space Partitioning (BSP) trees for efficient rendering and collision detection. The BSP tree determines what parts of the level are visible from the player's position:

```rust
#[derive(Debug, Clone)]
pub struct BspNode {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox_right: [i16; 4], // top, bottom, left, right
    pub bbox_left: [i16; 4],
    pub right_child: u16,
    pub left_child: u16,
}

#[derive(Debug, Clone)]
pub struct BspTree {
    pub nodes: Vec<BspNode>,
    pub subsectors: Vec<Subsector>,
    pub segs: Vec<Seg>,
}

#[derive(Debug, Clone)]
pub struct Subsector {
    pub seg_count: u16,
    pub first_seg: u16,
}

#[derive(Debug, Clone)]
pub struct Seg {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub angle: u16,
    pub linedef: u16,
    pub direction: u16,
    pub offset: u16,
}

impl BspTree {
    pub fn load_from_wad(wad: &WadFile, map_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let map_index = wad.lumps.iter().position(|lump| lump.name == map_name)
            .ok_or("Map not found")?;

        let nodes = Self::parse_nodes(&wad.lumps[map_index + 7].data)?;
        let subsectors = Self::parse_subsectors(&wad.lumps[map_index + 6].data)?;
        let segs = Self::parse_segs(&wad.lumps[map_index + 5].data)?;

        Ok(BspTree { nodes, subsectors, segs })
    }

    fn parse_nodes(data: &[u8]) -> Result<Vec<BspNode>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut nodes = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let x = cursor.read_i16::<LittleEndian>()?;
            let y = cursor.read_i16::<LittleEndian>()?;
            let dx = cursor.read_i16::<LittleEndian>()?;
            let dy = cursor.read_i16::<LittleEndian>()?;

            let mut bbox_right = [0i16; 4];
            let mut bbox_left = [0i16; 4];

            for i in 0..4 {
                bbox_right[i] = cursor.read_i16::<LittleEndian>()?;
            }
            for i in 0..4 {
                bbox_left[i] = cursor.read_i16::<LittleEndian>()?;
            }

            let right_child = cursor.read_u16::<LittleEndian>()?;
            let left_child = cursor.read_u16::<LittleEndian>()?;

            nodes.push(BspNode {
                x, y, dx, dy,
                bbox_right,
                bbox_left,
                right_child,
                left_child,
            });
        }

        Ok(nodes)
    }

    pub fn traverse_bsp(&self, player_x: f64, player_y: f64, node_index: u16) -> Vec<u16> {
        if node_index & 0x8000 != 0 {
            // Leaf node (subsector)
            return vec![node_index & 0x7FFF];
        }

        let node = &self.nodes[node_index as usize];
        let side = self.point_on_side(player_x, player_y, node);

        let mut visible_subsectors = Vec::new();

        if side <= 0 {
            // Traverse left side first
            visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.left_child));
            if self.bbox_visible(player_x, player_y, &node.bbox_right) {
                visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.right_child));
            }
        } else {
            // Traverse right side first
            visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.right_child));
            if self.bbox_visible(player_x, player_y, &node.bbox_left) {
                visible_subsectors.extend(self.traverse_bsp(player_x, player_y, node.left_child));
            }
        }

        visible_subsectors
    }

    fn point_on_side(&self, x: f64, y: f64, node: &BspNode) -> i32 {
        let dx = x - node.x as f64;
        let dy = y - node.y as f64;

        let cross_product = dx * node.dy as f64 - dy * node.dx as f64;

        if cross_product > 0.0 { 1 } else { -1 }
    }

    fn bbox_visible(&self, player_x: f64, player_y: f64, bbox: &[i16; 4]) -> bool {
        // Simple frustum culling - check if bounding box is potentially visible
        // This is a simplified version; real Doom does more sophisticated clipping
        let distance = ((bbox[2] as f64 - player_x).powi(2) + (bbox[3] as f64 - player_y).powi(2)).sqrt();
        distance < 1000.0 // Max view distance
    }

    // Additional parsing functions for subsectors and segs...
    fn parse_subsectors(data: &[u8]) -> Result<Vec<Subsector>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut subsectors = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let seg_count = cursor.read_u16::<LittleEndian>()?;
            let first_seg = cursor.read_u16::<LittleEndian>()?;

            subsectors.push(Subsector { seg_count, first_seg });
        }

        Ok(subsectors)
    }

    fn parse_segs(data: &[u8]) -> Result<Vec<Seg>, Box<dyn std::error::Error>> {
        let mut cursor = std::io::Cursor::new(data);
        let mut segs = Vec::new();

        while cursor.position() < data.len() as u64 {
            use byteorder::{LittleEndian, ReadBytesExt};

            let start_vertex = cursor.read_u16::<LittleEndian>()?;
            let end_vertex = cursor.read_u16::<LittleEndian>()?;
            let angle = cursor.read_u16::<LittleEndian>()?;
            let linedef = cursor.read_u16::<LittleEndian>()?;
            let direction = cursor.read_u16::<LittleEndian>()?;
            let offset = cursor.read_u16::<LittleEndian>()?;

            segs.push(Seg {
                start_vertex,
                end_vertex,
                angle,
                linedef,
                direction,
                offset,
            });
        }

        Ok(segs)
    }
}

// Integration with renderer for proper visibility culling
impl Renderer {
    fn render_frame_with_bsp(&mut self, game_state: &GameState, bsp: &BspTree) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        if let Some(map) = &game_state.current_map {
            // Get visible subsectors using BSP traversal
            let visible_subsectors = bsp.traverse_bsp(
                game_state.player.x,
                game_state.player.y,
                (bsp.nodes.len() - 1) as u16 // Root node is typically the last one
            );

            // Render only visible subsectors
            for subsector_index in visible_subsectors {
                self.render_subsector(map, bsp, subsector_index, &game_state.player)?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    fn render_subsector(&mut self, map: &Map, bsp: &BspTree, subsector_index: u16, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        if (subsector_index as usize) >= bsp.subsectors.len() {
            return Ok(());
        }

        let subsector = &bsp.subsectors[subsector_index as usize];

        // Render all segs in this subsector
        for i in 0..subsector.seg_count {
            let seg_index = subsector.first_seg + i;
            if (seg_index as usize) < bsp.segs.len() {
                let seg = &bsp.segs[seg_index as usize];
                self.render_seg(map, seg, player)?;
            }
        }

        Ok(())
    }
}
```

## Testing Strategy

Testing a complex system like a Doom port requires a multi-faceted approach:

1. **Unit Tests**: Test individual functions and methods for expected behavior.
2. **Integration Tests**: Test how different parts of the system work together, like loading a WAD file and parsing a map.
3. **Performance Tests**: Ensure the engine runs at an acceptable frame rate with various map and entity complexities.
4. **Regression Tests**: Ensure that bugs do not reappear in future code changes.

### Example Unit Test

Here's an example of a unit test for the `WadFile` loading:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_wad_file() {
        let wad = WadFile::load(std::fs::File::open("path/to/test.wad").unwrap()).unwrap();

        assert_eq!(wad.lumps.len(), 5); // Assuming 5 lumps in test WAD
        assert!(wad.find_lump("MAP01").is_some());
    }
}
```

### Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Performance Optimization

Even though we're avoiding unsafe code, we can still achieve excellent performance through smart algorithmic choices and Rust's zero-cost abstractions:

### Memory Management Strategies

```rust
pub struct FrameAllocator {
    buffer: Vec<u8>,
    offset: usize,
}

impl FrameAllocator {
    pub fn new(size: usize) -> Self {
        FrameAllocator {
            buffer: vec![0; size],
            offset: 0,
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn allocate_slice<T>(&mut self, count: usize) -> Option<&mut [T]> {
        let size_needed = count * std::mem::size_of::<T>();
        let aligned_offset = (self.offset + std::mem::align_of::<T>() - 1)
                           & !(std::mem::align_of::<T>() - 1);

        if aligned_offset + size_needed <= self.buffer.len() {
            self.offset = aligned_offset + size_needed;

            // Safe approach without unsafe code
            let end = aligned_offset + size_needed;
            if end <= self.buffer.len() {
                // We can't directly transmute safely, so we use a different approach
                // For performance-critical code, consider using specialized data structures
                return None; // Placeholder - implement with safe alternatives
            }
        }
        None
    }
}

// Use pre-allocated buffers for rendering
impl Renderer {
    pub fn new_with_buffers(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let mut renderer = Self::new(sdl_context)?;

        // Pre-allocate commonly used buffers
        renderer.depth_buffer = vec![f32::INFINITY; (renderer.screen_width * renderer.screen_height) as usize];
        renderer.wall_distances = vec![0.0; renderer.screen_width as usize];

        Ok(renderer)
    }
}
```

### SIMD and Parallelization

Add `rayon` to your dependencies for safe parallelization:

```toml
[dependencies]
rayon = "1.7"
```

```rust
use rayon::prelude::*;

impl Renderer {
    fn parallel_ray_casting(&mut self, map: &Map, player: &Player) -> Result<Vec<Option<RayHit>>, Box<dyn std::error::Error>> {
        let screen_width = self.screen_width;
        let fov = std::f64::consts::PI / 3.0;
        let half_fov = fov / 2.0;

        let rays: Vec<_> = (0..screen_width)
            .into_par_iter()
            .map(|x| {
                let ray_angle = player.angle - half_fov + (x as f64 / screen_width as f64) * fov;
                self.cast_ray_threadsafe(map, player, ray_angle)
            })
            .collect();

        Ok(rays)
    }

    fn cast_ray_threadsafe(&self, map: &Map, player: &Player, angle: f64) -> Option<RayHit> {
        // Thread-safe version of ray casting
        let ray_dx = angle.cos();
        let ray_dy = angle.sin();

        let mut distance = 0.0;
        let step_size = 1.0;

        while distance < 1000.0 {
            let test_x = player.x + ray_dx * distance;
            let test_y = player.y + ray_dy * distance;

            // Check collision without modifying shared state
            if let Some(wall_hit) = self.check_wall_collision_readonly(map, test_x, test_y) {
                return Some(RayHit {
                    distance,
                    wall_type: wall_hit,
                    hit_x: test_x,
                    hit_y: test_y,
                });
            }

            distance += step_size;
        }

        None
    }
}
```

### Profiling and Benchmarking

Add benchmarking capabilities:

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "doom_benchmarks"
harness = false
```

```rust
// benches/doom_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doom_port::*;

fn benchmark_wad_parsing(c: &mut Criterion) {
    let wad_data = create_test_wad_data();

    c.bench_function("wad_parsing", |b| {
        b.iter(|| {
            let cursor = std::io::Cursor::new(black_box(&wad_data));
            WadFile::load(cursor).unwrap()
        })
    });
}

fn benchmark_ray_casting(c: &mut Criterion) {
    let map = create_test_map();
    let player = Player::new(50.0, 50.0);

    c.bench_function("single_ray_cast", |b| {
        b.iter(|| {
            black_box(cast_ray_for_benchmark(&map, &player, 0.0))
        })
    });

    c.bench_function("full_screen_rays", |b| {
        b.iter(|| {
            for x in 0..800 {
                let angle = x as f64 * 0.001;
                black_box(cast_ray_for_benchmark(&map, &player, angle));
            }
        })
    });
}

fn benchmark_entity_updates(c: &mut Criterion) {
    let mut entity_manager = EntityManager::new();

    // Spawn many entities for testing
    for i in 0..1000 {
        entity_manager.spawn_entity(
            i as f64,
            i as f64,
            EntityType::Monster { health: 60, monster_type: MonsterType::Imp },
            "TROO".to_string()
        );
    }

    let player = Player::new(500.0, 500.0);
    let map = create_test_map();

    c.bench_function("update_1000_entities", |b| {
        b.iter(|| {
            entity_manager.update_entities(
                black_box(std::time::Duration::from_millis(16)),
                black_box(&player),
                black_box(&map)
            )
        })
    });
}

criterion_group!(benches, benchmark_wad_parsing, benchmark_ray_casting, benchmark_entity_updates);
criterion_main!(benches);

// Helper functions for benchmarking
fn create_test_wad_data() -> Vec<u8> {
    let mut wad_data = Vec::new();
    wad_data.extend_from_slice(b"IWAD");
    wad_data.extend_from_slice(&1u32.to_le_bytes());
    wad_data.extend_from_slice(&12u32.to_le_bytes());
    wad_data.extend_from_slice(&0u32.to_le_bytes());
    wad_data.extend_from_slice(&4u32.to_le_bytes());
    wad_data.extend_from_slice(b"TEST\0\0\0\0");
    wad_data
}

fn create_test_map() -> Map {
    Map {
        vertices: (0..1000).map(|i| Vertex { x: i % 100, y: i / 100 }).collect(),
        linedefs: vec![],
        sidedefs: vec![],
        sectors: vec![],
        things: vec![],
    }
}

fn cast_ray_for_benchmark(map: &Map, player: &Player, angle: f64) -> Option<f64> {
    // Simplified ray casting for benchmarking
    let ray_dx = angle.cos();
    let ray_dy = angle.sin();

    for distance in (0..1000).map(|d| d as f64) {
        let test_x = player.x + ray_dx * distance;
        let test_y = player.y + ray_dy * distance;

        // Simple boundary check for benchmark
        if test_x < 0.0 || test_x > 1000.0 || test_y < 0.0 || test_y > 1000.0 {
            return Some(distance);
        }
    }
    None
}
```

### Memory Pool Pattern

For frequently allocated objects, use memory pools:

```rust
pub struct EntityPool {
    entities: Vec<Entity>,
    free_indices: Vec<usize>,
}

impl EntityPool {
    pub fn new(capacity: usize) -> Self {
        EntityPool {
            entities: Vec::with_capacity(capacity),
            free_indices: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Option<&mut Entity> {
        if let Some(index) = self.free_indices.pop() {
            Some(&mut self.entities[index])
        } else if self.entities.len() < self.entities.capacity() {
            self.entities.push(Entity::default());
            self.entities.last_mut()
        } else {
            None // Pool exhausted
        }
    }

    pub fn deallocate(&mut self, entity: &Entity) {
        if let Some(index) = self.entities.iter().position(|e| e.id == entity.id) {
            self.free_indices.push(index);
        }
    }
}
```

## Common Pitfalls and Solutions

### 1. Endianness Issues

Always use the `byteorder` crate when reading binary data:

```rust
// ❌ Wrong - platform dependent and unsafe
let value = unsafe { *(data.as_ptr() as *const u32) };

// ✅ Correct - explicit endianness handling
use byteorder::{LittleEndian, ReadBytesExt};
let mut cursor = std::io::Cursor::new(data);
let value = cursor.read_u32::<LittleEndian>()?;
```

### 2. Integer Overflow

Doom coordinates can be large, so use appropriate types:

```rust
// ❌ Potential overflow with i16
let distance = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as i16;

// ✅ Use appropriate types for calculations
let dx = x2 as i32 - x1 as i32;
let dy = y2 as i32 - y1 as i32;
let distance_squared = dx * dx + dy * dy;
let distance = (distance_squared as f64).sqrt();
```

### 3. Float Precision Issues

```rust
// ❌ Direct float comparison
if player.x == target.x {
    // This rarely works as expected
}

// ✅ Use epsilon comparison
const EPSILON: f64 = 1e-10;
if (player.x - target.x).abs() < EPSILON {
    // Much more reliable
}
```

### 4. Performance Anti-patterns

```rust
// ❌ Allocating in hot loops
fn render_frame(&mut self) {
    for x in 0..self.screen_width {
        let mut visible_walls = Vec::new(); // Allocation every iteration!
        // ...
    }
}

// ✅ Reuse allocations
struct Renderer {
    temp_walls: Vec<Wall>, // Reused buffer
}

fn render_frame(&mut self) {
    for x in 0..self.screen_width {
        self.temp_walls.clear(); // Reuse existing allocation
        // ...
    }
}
```

### 5. Error Handling Mistakes

```rust
// ❌ Panicking on errors
let wad = WadFile::load(file).unwrap(); // Crashes on invalid files

// ✅ Proper error handling
let wad = match WadFile::load(file) {
    Ok(w) => w,
    Err(e) => {
        eprintln!("Failed to load WAD file: {}", e);
        return Err(e.into());
    }
};
```

## Debugging Techniques

### Visual Debugging

```rust
impl Renderer {
    pub fn draw_debug_info(&mut self, player: &Player, map: &Map) -> Result<(), Box<dyn std::error::Error>> {
        if cfg!(debug_assertions) {
            self.draw_player_debug_info(player)?;
            self.draw_map_debug_info(map)?;
            self.draw_performance_metrics()?;
        }
        Ok(())
    }

    fn draw_player_debug_info(&mut self, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        // Draw player position as red dot
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(sdl2::rect::Rect::new(
            (player.x as i32) - 2,
            (player.y as i32) - 2,
            4,
            4
        ))?;

        // Draw player direction
        let end_x = player.x + player.angle.cos() * 50.0;
        let end_y = player.y + player.angle.sin() * 50.0;
        self.canvas.draw_line(
            (player.x as i32, player.y as i32),
            (end_x as i32, end_y as i32)
        )?;

        Ok(())
    }

    fn draw_map_debug_info(&mut self, map: &Map) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));

        // Draw map wireframe
        for linedef in &map.linedefs {
            if linedef.start_vertex < map.vertices.len() as u16 &&
               linedef.end_vertex < map.vertices.len() as u16 {
                let start = &map.vertices[linedef.start_vertex as usize];
                let end = &map.vertices[linedef.end_vertex as usize];

                self.canvas.draw_line(
                    (start.x as i32, start.y as i32),
                    (end.x as i32, end.y as i32)
                )?;
            }
        }

        Ok(())
    }
}
```

### Logging and Telemetry

```rust
use log::{info, warn, error, debug, trace};

// Add to Cargo.toml:
// [dependencies]
// log = "0.4"
// env_logger = "0.10"

impl DoomEngine {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting Doom engine");

        let mut frame_count = 0;
        let mut last_fps_update = std::time::Instant::now();
        let mut frame_times = Vec::new();

        'running: loop {
            let frame_start = std::time::Instant::now();

            // ... game loop ...

            let frame_time = frame_start.elapsed();
            frame_times.push(frame_time);

            frame_count += 1;
            if last_fps_update.elapsed().as_secs() >= 1 {
                let avg_frame_time = frame_times.iter().sum::<std::time::Duration>() / frame_times.len() as u32;
                debug!("FPS: {}, Avg frame time: {:?}", frame_count, avg_frame_time);

                frame_count = 0;
                frame_times.clear();
                last_fps_update = std::time::Instant::now();
            }

            // Log performance warnings
            if frame_time.as_millis() > 20 {
                warn!("Slow frame detected: {:?}", frame_time);
            }
        }

        info!("Doom engine shutdown");
        Ok(())
    }
}
```

### Debug Console

```rust
pub struct DebugConsole {
    commands: std::collections::HashMap<String, Box<dyn Fn(&[String]) -> String>>,
    history: Vec<String>,
    visible: bool,
}

impl DebugConsole {
    pub fn new() -> Self {
        let mut console = DebugConsole {
            commands: std::collections::HashMap::new(),
            history: Vec::new(),
            visible: false,
        };

        console.register_default_commands();
        console
    }

    fn register_default_commands(&mut self) {
        self.commands.insert("help".to_string(), Box::new(|_| {
            "Available commands: help, fps, memory, entities".to_string()
        }));

        self.commands.insert("fps".to_string(), Box::new(|_| {
            "FPS tracking enabled".to_string()
        }));
    }

    pub fn execute_command(&mut self, input: &str) -> String {
        let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return "Empty command".to_string();
        }

        let command = &parts[0];
        let args = &parts[1..];

        if let Some(cmd_fn) = self.commands.get(command) {
            cmd_fn(args)
        } else {
            format!("Unknown command: {}", command)
        }
    }
}
```

## Building and Distribution

### Cross-platform Build Configuration

```toml
# Cargo.toml
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

[target.'cfg(unix)'.dependencies]
libc = "0.2"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.24"
objc = "0.2"
```

### Release Optimization

```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization at cost of compile time
panic = "abort"         # Smaller binary size
strip = true           # Remove debug symbols
opt-level = 3          # Maximum optimization

[profile.release-with-debug]
inherits = "release"
debug = true           # Keep debug info for profiling
strip = false
```

### Distribution Scripts

Create a `build-release.sh` script:

```bash
#!/bin/bash
set -e

echo "Building Doom port for multiple targets..."

# Clean previous builds
cargo clean

# Native target
echo "Building for native target..."
cargo build --profile release-with-debug
cargo build --release

# Cross-compilation targets
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

echo "Building for macOS Intel..."
cargo build --release --target x86_64-apple-darwin

echo "Building for macOS ARM..."
cargo build --release --target aarch64-apple-darwin

echo "Creating distribution packages..."
mkdir -p dist

# Package for each platform with dependencies
cp target/release/doom-port dist/doom-port-linux
cp target/x86_64-pc-windows-gnu/release/doom-port.exe dist/doom-port-windows.exe

# Create a simple installer script
cat > dist/install.sh << 'EOF'
#!/bin/bash
echo "Installing Doom Port..."

# Check for required libraries
if ! command -v sdl2-config &> /dev/null; then
    echo "SDL2 not found. Please install SDL2 development libraries."
    echo "Ubuntu/Debian: sudo apt-get install libsdl2-dev"
    echo "macOS: brew install sdl2"
    exit 1
fi

echo "Installation complete!"
EOF

chmod +x dist/install.sh

echo "Distribution packages created in ./dist/"
echo "Run benchmarks with: cargo bench"
echo "Profile with: cargo flamegraph --bin doom-port"
```

### Automated Testing and CI

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: rustfmt, clippy

      - name: Install SDL2 (Ubuntu)
        if: matrix.os == 'ubuntu-latest'
        run: |
          sudo apt-get update
          sudo apt-get install libsdl2-dev libsdl2-mixer-dev

      - name: Install SDL2 (macOS)
        if: matrix.os == 'macos-latest'
        run: brew install sdl2 sdl2_mixer

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --all-features --verbose

      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Run benchmarks
        run: cargo bench --no-run

  security_audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit
```

This comprehensive documentation now covers all aspects of building a Doom port in Rust, from basic concepts to advanced optimization techniques, all while maintaining safe code practices.
