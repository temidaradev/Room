use sdl2::Sdl;

use entity::*;
use map::*;
use math::*;
use player::*;
use renderer::*;
use wad::WadFile;

use std::time::{Duration, Instant};

pub struct Engine {
    sdl_context: Sdl,
    wad: WadFile,
    renderer: Renderer,
    game_state: GameState,
    last_frame_time: Instant,
}

pub struct GameState {
    pub current_map: Option<Map>,
    pub player: Player,
    pub entities: Vec<Entity>,
    pub game_time: Duration,
}
