use sdl2::Sdl;

use entity::*;
use input::*;
use map::*;
use math::*;
use player::*;
use renderer::*;
use wad::WadFile;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

pub struct Engine {
    sdl_context: Sdl,
    wad: WadFile,
    renderer: Renderer,
    game_state: GameState,
    input_handler: Input,
    last_frame_time: Instant,
}

pub struct GameState {
    pub current_map: Option<Map>,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub game_time: Duration,
}

impl Engine {
    pub fn new(wad_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let wad = WadFile::load(std::fs::File::open(wad_path)?)?;

        let renderer = Renderer::new(&sdl_context)?;
        let game_state = GameState::new();
        let input_handler = Input::new(&sdl_context)?;

        Ok(Engine {
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

    fn update_game_state(
        &mut self,
        delta_time: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.game_state.game_time += delta_time;

        // Update player position based on input
        self.game_state
            .player
            .update(delta_time, &self.input_handler);

        // Update entities
        for entity in &mut self.game_state.entities {
            entity.update(delta_time);
        }

        Ok(())
    }
}
