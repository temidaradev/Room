---
id: task-10
title: Set up basic SDL2 window
status: To Do
assignee: []
created_date: '2025-07-09'
updated_date: '2025-07-09'
labels: []
dependencies: []
---

## Description

Learn to use external crates (dependencies) by setting up a basic graphics window. This introduces dependency management and FFI concepts.

## Acceptance Criteria

- [ ] Adds SDL2 to Cargo.toml dependencies
- [ ] Creates a window that opens and closes
- [ ] Handles window events properly
- [ ] Understands the concept of external crates

## Implementation Plan

## Background

SDL2 (Simple DirectMedia Layer) is a cross-platform development library for multimedia applications, games, and emulators. It provides low-level access to audio, keyboard, mouse, joystick, and graphics hardware. SDL2 is widely used in game development and is the foundation for many game engines.

### Key Concepts for This Task:
- External dependencies: Using third-party libraries
- FFI (Foreign Function Interface): Rust calling C libraries
- Event-driven programming: Handling user input and system events
- Graphics contexts: Creating and managing render targets
- Resource management: Proper cleanup of system resources

### SDL2 Architecture:
- Initialization: Setting up SDL2 subsystems
- Window creation: Creating a render target
- Event loop: Processing user input and system events
- Rendering: Drawing to the screen
- Cleanup: Properly releasing resources

### Event-Driven Programming:
Instead of polling for input, SDL2 uses an event queue:
- Events are pushed to a queue by the system
- Your program pulls events from the queue each frame
- Each event represents something that happened (keypress, mouse click, etc.)

### Example Code:
// Add to Cargo.toml:
// [dependencies]
// sdl2 = "0.35"

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    // Create window
    let window = video_subsystem
        .window("Room Game", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    // Create renderer
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    
    // Set up event pump
    let mut event_pump = sdl_context.event_pump()?;
    
    // Game state
    let mut player_x = 400;
    let mut player_y = 300;
    let player_speed = 200; // pixels per second
    
    // Timing
    let mut last_time = std::time::Instant::now();
    
    // Main game loop
    'running: loop {
        // Calculate delta time
        let now = std::time::Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up | Keycode::W => player_y -= (player_speed as f32 * dt) as i32,
                        Keycode::Down | Keycode::S => player_y += (player_speed as f32 * dt) as i32,
                        Keycode::Left | Keycode::A => player_x -= (player_speed as f32 * dt) as i32,
                        Keycode::Right | Keycode::D => player_x += (player_speed as f32 * dt) as i32,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // Keep player on screen
        player_x = player_x.max(0).min(800 - 20);
        player_y = player_y.max(0).min(600 - 20);
        
        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw player
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(player_x, player_y, 20, 20))?;
        
        // Present to screen
        canvas.present();
        
        // Maintain roughly 60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
    
    Ok(())
}

// More advanced example with better structure
struct GameState {
    player_x: f32,
    player_y: f32,
    player_speed: f32,
    running: bool,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_x: 400.0,
            player_y: 300.0,
            player_speed: 200.0,
            running: true,
        }
    }
    
    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Quit { .. } => self.running = false,
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => self.running = false,
            _ => {}
        }
    }
    
    fn update(&mut self, dt: f32, keys: &std::collections::HashSet<Keycode>) {
        // Handle continuous input
        if keys.contains(&Keycode::Up) || keys.contains(&Keycode::W) {
            self.player_y -= self.player_speed * dt;
        }
        if keys.contains(&Keycode::Down) || keys.contains(&Keycode::S) {
            self.player_y += self.player_speed * dt;
        }
        if keys.contains(&Keycode::Left) || keys.contains(&Keycode::A) {
            self.player_x -= self.player_speed * dt;
        }
        if keys.contains(&Keycode::Right) || keys.contains(&Keycode::D) {
            self.player_x += self.player_speed * dt;
        }
        
        // Keep player on screen
        self.player_x = self.player_x.max(0.0).min(800.0 - 20.0);
        self.player_y = self.player_y.max(0.0).min(600.0 - 20.0);
    }
    
    fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        // Clear screen
        canvas.set_draw_color(Color::RGB(32, 32, 32));
        canvas.clear();
        
        // Draw player
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        canvas.fill_rect(Rect::new(
            self.player_x as i32,
            self.player_y as i32,
            20,
            20,
        ))?;
        
        // Draw border
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(0, 0, 800, 600))?;
        
        canvas.present();
        Ok(())
    }
}

fn better_main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem
        .window("Room Game", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut game_state = GameState::new();
    
    let mut keys = std::collections::HashSet::new();
    let mut last_time = std::time::Instant::now();
    
    while game_state.running {
        let now = std::time::Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;
        
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    keys.insert(keycode);
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    keys.remove(&keycode);
                }
                _ => {}
            }
            game_state.handle_event(&event);
        }
        
        // Update game state
        game_state.update(dt, &keys);
        
        // Render
        game_state.render(&mut canvas)?;
    }
    
    Ok(())
}

## Step-by-Step Implementation:

1. Add SDL2 to Cargo.toml dependencies
2. Initialize SDL2 and create a window
3. Set up the event loop structure
4. Handle window close events
5. Add keyboard input handling
6. Create a simple moving rectangle (player)
7. Add proper timing (delta time)
8. Implement screen boundaries
9. Add better structure with GameState
10. Learn about different SDL2 features (sprites, audio, etc.)

### SDL2 Installation Notes:
- On macOS: brew install sdl2
- On Ubuntu: apt-get install libsdl2-dev
- On Windows: SDL2 libraries are usually bundled with the Rust crate
