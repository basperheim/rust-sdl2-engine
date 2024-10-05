use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use serde::Deserialize;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use sdl2::event::Event;
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
            let texture = self.texture_creator.load_texture(path)?;
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
struct SpriteConfig {
    images: Vec<String>,
    location: Point,
}

#[derive(Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct GameState {
    window: WindowConfig,
    sprites: Vec<SpriteConfig>,
    fps: Option<u64>,
}

impl GameState {
    fn fps(&self) -> u64 {
        self.fps.unwrap_or(60)
    }
}

fn parse_game_state(encoded_data: &str) -> Result<GameState, serde_json::Error> {
    let decoded = STANDARD.decode(encoded_data).expect("Failed to decode base64");
    let json_str = String::from_utf8(decoded).expect("Invalid UTF-8 sequence");
    serde_json::from_str::<GameState>(&json_str)
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create a window with default size
    let window = video_subsystem
        .window("Simple 2D Renderer", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a canvas (renderer)
    let mut canvas: Canvas<Window> = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize SDL2_image
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Initialize the texture manager
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);

    // Set up event handling
    let mut event_pump = sdl_context.event_pump()?;

    // Set up channel for non-blocking input
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Spawn a thread to read from stdin
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(input) = line {
                if tx.send(input).is_err() {
                    break;
                }
            }
        }
    });

    let mut game_state: Option<GameState> = None;
    let mut frame_duration = Duration::from_millis(16); // Default to ~60 FPS

    'running: loop {
        // Non-blocking receive
        match rx.try_recv() {
            Ok(input) => {
                match parse_game_state(&input) {
                    Ok(state) => {
                        // Resize window if necessary
                        let (current_width, current_height) = canvas.output_size()?;
                        if state.window.width != current_width || state.window.height != current_height {
                            canvas
                                .window_mut()
                                .set_size(state.window.width, state.window.height)
                                .map_err(|e| e.to_string())?;
                        }

                        // Update frame duration based on FPS
                        frame_duration = Duration::from_millis(1000 / state.fps());

                        // Update game state
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
            canvas.copy(&bg_texture, None, None)?;

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
                canvas.copy(&texture, None, Some(position))?;
            }
        }

        // Update the screen
        canvas.present();

        // Frame rate control
        ::std::thread::sleep(frame_duration);
    }

    Ok(())
}
