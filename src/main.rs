use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use serde::Deserialize;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use sdl2::event::Event;
// Removed the unused import: `use sdl2::keyboard::Keycode;`
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

const SPRITE_WIDTH: u32 = 64;
const SPRITE_HEIGHT: u32 = 64;

struct TextureManager<'a> {
    textures: HashMap<String, Texture<'a>>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl<'a> TextureManager<'a> {
    fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            textures: HashMap::new(),
            texture_creator,
        }
    }

    fn load_texture(&mut self, path: &str) -> Result<&Texture<'a>, String> {
        if !self.textures.contains_key(path) {
            // Map the error to String using `map_err`
            let texture = self.texture_creator.load_texture(path).map_err(|e| e.to_string())?;
            self.textures.insert(path.to_string(), texture);
        }
        Ok(self.textures.get(path).unwrap())
    }
}

#[derive(Deserialize)]
struct WindowConfig {
    width: u32,
    height: u32,
    background: String,
}

#[derive(Deserialize)]
struct Player {
    id: String,
    owner: String,
    isHuman: bool,
}

#[derive(Deserialize)]
struct SpriteConfig {
    id: String,
    playerId: String,
    images: Vec<String>,
    location: Point,
    health: u32,
}

#[derive(Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct GameState {
    window: WindowConfig,
    players: Vec<Player>,
    sprites: Vec<SpriteConfig>,
}

fn parse_game_state(encoded_data: &str) -> Result<GameState, serde_json::Error> {
    let decoded = STANDARD.decode(encoded_data).expect("Failed to decode base64");
    let json_str = String::from_utf8(decoded).expect("Invalid UTF-8 sequence");
    serde_json::from_str::<GameState>(&json_str)
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    // Create a window with default size
    let window = video_subsystem
        .window("Simple 2D Renderer", 800, 600) // Default size
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a canvas (renderer)
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .present_vsync() // Synchronize with the display's refresh rate
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize SDL2_image
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).map_err(|e| e.to_string())?;

    // Initialize the texture manager
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    // Set up event handling
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    // Set up channel for non-blocking input
    let (tx, rx) = mpsc::channel();

    // Spawn a thread to read from stdin
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(input) = line {
                // It's good practice to handle potential send errors
                if tx.send(input).is_err() {
                    break;
                }
            }
        }
    });

    let mut game_state: Option<GameState> = None;

    'running: loop {
        // Non-blocking receive
        match rx.try_recv() {
            Ok(input) => {
                match parse_game_state(&input) {
                    Ok(state) => {
                        // **Use `state` before moving it into `game_state`**
                        // Resize window if necessary
                        let (current_width, current_height) = canvas.output_size().map_err(|e| e.to_string())?;
                        if state.window.width != current_width || state.window.height != current_height {
                            canvas
                                .window_mut()
                                .set_size(state.window.width, state.window.height)
                                .map_err(|e| e.to_string())?;
                        }

                        // Now move `state` into `game_state`
                        game_state = Some(state);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse game state: {}", e);
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => break 'running,
        }

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Event: Quit");
                    io::stdout().flush().unwrap();
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    println!("Event: KeyDown {:?}", keycode);
                    io::stdout().flush().unwrap();
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    println!("Event: KeyUp {:?}", keycode);
                    io::stdout().flush().unwrap();
                }
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    println!("Event: MouseButtonDown {:?} at ({}, {})", mouse_btn, x, y);
                    io::stdout().flush().unwrap();
                }
                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    println!("Event: MouseButtonUp {:?} at ({}, {})", mouse_btn, x, y);
                    io::stdout().flush().unwrap();
                }
                _ => {}
            }
        }

        // Clear the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if let Some(ref state) = game_state {
            // Render background
            let bg_texture = texture_manager.load_texture(&state.window.background)?;
            canvas.copy(&bg_texture, None, None).map_err(|e| e.to_string())?;

            // Render sprites
            for sprite_config in &state.sprites {
                if sprite_config.images.is_empty() {
                    continue; // Skip if there are no images
                }
                let texture = texture_manager.load_texture(&sprite_config.images[0])?;
                let position = Rect::new(
                    sprite_config.location.x,
                    sprite_config.location.y,
                    SPRITE_WIDTH,
                    SPRITE_HEIGHT,
                );
                canvas.copy(&texture, None, Some(position)).map_err(|e| e.to_string())?;
            }
        }

        // Update the screen
        canvas.present();

        // Frame rate control
        ::std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
