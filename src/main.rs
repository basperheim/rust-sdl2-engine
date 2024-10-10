use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use std::env;
use std::path::Path;
use std::fs;

use serde::{Deserialize, Serialize};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{self, InitFlag, LoadTexture, ImageRWops};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::render::TextureQuery;

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
        // println!("Attempting to load texture: {}", path);  // Debugging line
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
    #[serde(default = "default_title")]
    title: String,
    #[serde(default = "default_icon_path")]
    icon_path: String,
}

fn default_title() -> String {
    "Learn Programming 2D Game Engine".to_string()
}

fn default_icon_path() -> String {
    let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| String::from("images"));
    Path::new(&images_dir).join("learn-programming-logo-128px.png").to_string_lossy().to_string()
}

#[derive(Deserialize, Serialize, Clone)]
struct SpriteSize {
    width: u32,
    height: u32,
}

#[derive(Deserialize, Serialize, Clone)]
struct SpriteConfig {
    id: String,
    images: Vec<String>,
    location: Point,
    size: SpriteSize,
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
struct TextConfig {
    id: String,
    font_family: Option<String>,
    content: String,
    size: u16,
    color: Option<ColorConfig>,
    location: Point,
}

#[derive(Deserialize, Serialize, Clone)]
struct ColorConfig {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 } // Default to black color
    }
}

struct FontManager<'a> {
    fonts: HashMap<String, Font<'a, 'a>>,
    ttf_context: &'a Sdl2TtfContext,
}

impl<'a> FontManager<'a> {
    fn new(ttf_context: &'a Sdl2TtfContext) -> Self {
        Self {
            fonts: HashMap::new(),
            ttf_context,
        }
    }

    fn load_font(&mut self, font_path: &str, size: u16) -> Result<&Font<'a, 'a>, String> {
        let key = format!("{}-{}", font_path, size);
        if !self.fonts.contains_key(&key) {
            let font = self.ttf_context.load_font(font_path, size).map_err(|e| {
                eprintln!("Error loading font '{}': {}", font_path, e);
                e.to_string()
            })?;
            self.fonts.insert(key.clone(), font);
        }
        Ok(self.fonts.get(&key).unwrap())
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct GameState {
    window: WindowConfig,
    sprites: Vec<SpriteConfig>,
    text: Vec<TextConfig>,
    default_font: String,
    fps: Option<u64>,
}

impl GameState {
    fn fps(&self) -> u64 {
        self.fps.unwrap_or(60)
    }
}

fn set_game_icon(canvas: &mut Canvas<Window>, file_path: &str) -> Result<(), String> {
    // Attempt to load the icon from the specified file path
    let icon_surface = sdl2::rwops::RWops::from_file(file_path, "r")
        .and_then(|rwops| rwops.load())
        .or_else(|e| {
            eprintln!("Error loading icon from file '{}': {}", file_path, e);

            // Construct the fallback icon path
            let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| String::from("images"));
            let fallback_icon_path = Path::new(&images_dir).join("learn-programming-logo-128px.png");

            // Read the fallback icon as bytes
            match fs::read(fallback_icon_path.clone()) { // Clone the path before moving it
                Ok(fallback_icon_bytes) => {
                    // Convert the bytes to RWops
                    sdl2::rwops::RWops::from_bytes(&fallback_icon_bytes)
                        .and_then(|rwops| rwops.load())
                        .map_err(|e| {
                            eprintln!("Error loading embedded icon: {}", e);
                            e.to_string()
                        })
                },
                Err(e) => {
                    eprintln!("Error reading fallback icon from path '{}': {}", fallback_icon_path.display(), e);
                    Err(e.to_string())
                }
            }
        })?;

    // Set the icon for the window
    canvas.window_mut().set_icon(icon_surface);
    Ok(())
}

fn parse_game_state(encoded_data: &str) -> Result<GameState, serde_json::Error> {
    let decoded = STANDARD.decode(encoded_data).expect("Failed to decode base64");
    let json_str = String::from_utf8(decoded).expect("Invalid UTF-8 sequence");
    serde_json::from_str::<GameState>(&json_str)
}

fn main() -> Result<(), String> {
    let images_dir: String = env::var("IMAGES_DIR").unwrap_or_else(|_| String::from("images"));
    let fonts_dir: String = env::var("FONTS_DIR").unwrap_or_else(|_| String::from("fonts"));

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize SDL2_ttf
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Initialize the FontManager
    let mut font_manager = FontManager::new(&ttf_context);

    // Create a window with a default title and size
    let window = video_subsystem
        .window(&default_title(), 800, 600)
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
    let mut icon_set = false;

    let mut mouse_x = 0;
    let mut mouse_y = 0;

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
                            existing_state.default_font = new_state.default_font.clone();

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
                            existing_state.text = new_state.text;

                            // Update the window title
                            canvas.window_mut().set_title(&existing_state.window.title).map_err(|e| e.to_string())?;

                            // Set the icon if not already set
                            if !icon_set {
                                set_game_icon(&mut canvas, &existing_state.window.icon_path)?;
                                icon_set = true; // Update the flag
                            }

                        } else {
                            // No existing game_state, so initialize it and set the title and icon
                            for sprite in &mut new_state.sprites {
                                sprite.current_frame = 0;
                                sprite.last_update = 0;
                            }

                            game_state = Some(new_state);

                            // Set the window title and icon for the first time
                            canvas.window_mut().set_title(&game_state.as_ref().unwrap().window.title).map_err(|e| e.to_string())?;

                            if !icon_set {
                                set_game_icon(&mut canvas, &game_state.as_ref().unwrap().window.icon_path)?;
                                icon_set = true; // Update the flag
                            }
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
                    let json_output = r#"{"action": "quit"}"#; // JSON for quit event
                    println!("{}", json_output);
                    io::stdout().flush().unwrap();
                    break 'running;
                }

                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;

                    // let json_output = format!(r#"{{"action": "mouse_motion", "x": {}, "y": {}}}"#, x, y);
                    // println!("{}", json_output);
                    // io::stdout().flush().unwrap();
                }

                Event::KeyDown { keycode: Some(keycode), .. } => {
                    let json_output = format!(r#"{{"action": "key_down", "keycode": "{:?}"}}"#, keycode);
                    println!("{}", json_output);
                    io::stdout().flush().unwrap();
                }

                Event::KeyUp { keycode: Some(keycode), .. } => {
                    let json_output = format!(r#"{{"action": "key_up", "keycode": "{:?}"}}"#, keycode);
                    println!("{}", json_output);
                    io::stdout().flush().unwrap();
                }

                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    let json_output = format!(r#"{{"action": "mouse_button_down", "button": "{:?}", "x": {}, "y": {}}}"#, mouse_btn, x, y);
                    println!("{}", json_output);
                    io::stdout().flush().unwrap();
                }

                Event::MouseButtonUp { x, y, mouse_btn, .. } => {
                    let json_output = format!(r#"{{"action": "mouse_button_up", "button": "{:?}", "x": {}, "y": {}}}"#, mouse_btn, x, y);
                    println!("{}", json_output);
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
            // Construct the background image path
            let background_path = Path::new(&images_dir)
                .join(&state.window.background)
                .to_string_lossy()
                .to_string();

            // Render background
            let bg_texture = texture_manager.load_texture(&background_path)?;
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

                // Construct the image path
                let image_path = Path::new(&images_dir)
                    .join(&sprite_config.images[sprite_config.current_frame])
                    .to_string_lossy()
                    .to_string();

                // Load the texture
                let texture = texture_manager.load_texture(&image_path)?;

                let position = Rect::new(
                    sprite_config.location.x,
                    sprite_config.location.y,
                    sprite_config.size.width,
                    sprite_config.size.height
                );

                // Check for hover
                if position.contains_point((mouse_x, mouse_y)) {
                    let json_output = format!(r#"{{"action": "hover", "sprite": "{}"}}"#, sprite_config.id);
                    println!("{}", json_output);
                    io::stdout().flush().unwrap();
                }

                canvas.copy(&texture, None, Some(position))?;
            }

            // Render text
            for text_config in &state.text {
                // Determine the font to use
                let font_family = text_config.font_family.as_ref().unwrap_or(&state.default_font);
                let font_path = Path::new(&fonts_dir).join(font_family).to_string_lossy().to_string();
                let font = font_manager.load_font(&font_path, text_config.size)?;

                // Render the text surface
                let color = text_config.color.clone().unwrap_or_default();
                let sdl_color = Color::RGBA(color.r, color.g, color.b, color.a);

                let surface = font
                    .render(&text_config.content)
                    .blended(sdl_color)
                    .map_err(|e| e.to_string())?;

                // Create a texture from the surface
                let texture_creator = canvas.texture_creator();
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                // Query the texture to get its width and height
                let TextureQuery { width, height, .. } = texture.query();

                // Set the destination rectangle
                let target = Rect::new(text_config.location.x, text_config.location.y, width, height);

                // Copy the texture to the canvas without scaling
                canvas.copy(&texture, None, Some(target))?;
            }
        }

        // Update the screen
        canvas.present();

        // Frame rate control
        ::std::thread::sleep(frame_duration);
    }

    Ok(())
}
