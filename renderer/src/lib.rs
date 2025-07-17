use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::f64::consts::PI;

pub struct Renderer {
    canvas: Canvas<Window>,
    screen_width: u32,
    screen_height: u32,
}

pub struct Sprite {
    pub texture: Texture,
    pub x: f64,
    pub y: f64,
    pub scale: f64,
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

    fn render_floor_ceiling(&mut self, map: &Map, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let half_height = self.screen_height as f64 / 2.0;

        for y in 0..self.screen_height {
            if y < half_height as u32 {
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

            let color = self.sample_floor_texture(world_x, world_y, is_ceiling);
            self.canvas.set_draw_color(color);
            self.canvas.draw_point((x as i32, screen_y as i32))?;
        }

        Ok(())
    }

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

    fn render_3d_view(&mut self, map: &Map, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let fov = PI / 3.0;
        let half_fov = fov / 2.0;

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

