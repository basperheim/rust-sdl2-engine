use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
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
        println!("Attempting to load texture: {}", path);  // Debugging line
        if !self.textures.contains_key(path) {
            let texture = self.texture_creator.load_texture(path).map_err(|e| {
                eprintln!("Error loading texture '{}': {}", path, e);
                e.to_string()
            })?;
            self.textures.insert(path.to_string(), texture);
        }
        Ok(self.textures.get(path).unwrap())
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct WindowConfig {
    width: u32,
    height: u32,
    background: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct SpriteConfig {
    id: String,
    images: Vec<String>,
    location: Point,
    #[serde(default = "default_frame_delay")]
    frame_delay: u64, // in milliseconds
    #[serde(skip)]
    current_frame: usize,
    #[serde(skip)]
    last_update: u128, // Using u128 to store milliseconds since UNIX epoch
}

fn default_frame_delay() -> u64 {
    100 // Default to 100ms
}

#[derive(Deserialize, Serialize, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Serialize, Clone)]
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
    let mut last_frame_time = Instant::now();

    'running: loop {
        // Non-blocking receive
        match rx.try_recv() {
            Ok(input) => {
                match parse_game_state(&input) {
                    Ok(mut new_state) => {
                        // Resize window if necessary
                        let (current_width, current_height) = canvas.output_size()?;
                        if new_state.window.width != current_width || new_state.window.height != current_height {
                            canvas
                                .window_mut()
                                .set_size(new_state.window.width, new_state.window.height)
                                .map_err(|e| e.to_string())?;
                        }

                        // Update frame duration based on FPS
                        frame_duration = Duration::from_millis(1000 / new_state.fps());

                        if let Some(existing_state) = &mut game_state {
                            // Update window config
                            existing_state.window = new_state.window;

                            // Take ownership of existing_state.sprites and create a HashMap
                            let mut existing_sprites_map: HashMap<String, SpriteConfig> = existing_state.sprites
                                .drain(..)
                                .map(|sprite| (sprite.id.clone(), sprite))
                                .collect();

                            // Prepare a new vector for updated sprites
                            let mut updated_sprites = Vec::new();

                            // Process each sprite in new_state.sprites
                            for mut new_sprite in new_state.sprites {
                                if let Some(mut existing_sprite) = existing_sprites_map.remove(&new_sprite.id) {
                                    // Update fields while preserving animation state
                                    existing_sprite.images = new_sprite.images;
                                    existing_sprite.location = new_sprite.location;
                                    existing_sprite.frame_delay = new_sprite.frame_delay;
                                    updated_sprites.push(existing_sprite);
                                } else {
                                    // New sprite, initialize animation fields
                                    new_sprite.current_frame = 0;
                                    new_sprite.last_update = 0;
                                    updated_sprites.push(new_sprite);
                                }
                            }

                            // Update existing_state.sprites with the updated sprites
                            existing_state.sprites = updated_sprites;
                        } else {
                            // No existing game_state, so initialize it
                            for sprite in &mut new_state.sprites {
                                sprite.current_frame = 0;
                                sprite.last_update = 0;
                            }
                            game_state = Some(new_state);
                        }
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

        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_millis();
        last_frame_time = now;

        // Clear the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if let Some(ref mut state) = game_state {
            // Render background
            let bg_texture = texture_manager.load_texture(&state.window.background)?;
            canvas.copy(&bg_texture, None, None)?;

            // Render sprites
            for sprite_config in &mut state.sprites {
                if sprite_config.images.is_empty() {
                    continue; // Skip if there are no images
                }

                // Update animation frame
                sprite_config.frame_delay = sprite_config.frame_delay.max(1); // Ensure frame_delay is at least 1ms
                sprite_config.last_update += delta_time;

                if sprite_config.last_update >= sprite_config.frame_delay as u128 {
                    sprite_config.current_frame = (sprite_config.current_frame + 1) % sprite_config.images.len();
                    sprite_config.last_update = 0;
                }

                let texture = texture_manager.load_texture(&sprite_config.images[sprite_config.current_frame])?;
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
