use clap::Parser;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

/// Simple 2D Renderer Configurations
#[derive(Parser)]
struct Config {
    /// Maximum Frames Per Second
    #[arg(short, long, default_value_t = 60)]
    max_fps: u32,

    /// Window Width
    #[arg(short, long, default_value_t = 800)]
    width: u32,

    /// Window Height
    #[arg(short = 'H', long, default_value_t = 600)]
    height: u32,

    /// Sprite Image Path
    #[arg(short, long, default_value = "sprite.png")]
    sprite_path: String,
}

fn main() -> Result<(), String> {
    let config = Config::parse();

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize SDL2_image
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    // Create a window
    let window = video_subsystem
        .window("Simple 2D Renderer", config.width, config.height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a canvas (renderer)
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync() // Synchronize with the display's refresh rate
        .build()
        .map_err(|e| e.to_string())?;

    // Set up event handling
    let mut event_pump = sdl_context.event_pump()?;

    // Load the sprite texture
    let texture_creator = canvas.texture_creator();
    let texture_path = Path::new(&config.sprite_path);
    let texture = texture_creator.load_texture(texture_path)?;

    // Main loop variables
    let mut is_running = true;

    while is_running {
        // Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => is_running = false,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse clicked at ({}, {})", x, y);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => is_running = false,
                _ => {}
            }
        }

        // Clear the screen with black color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render the sprite
        let target_rect = sdl2::rect::Rect::new(100, 100, 64, 64); // Adjust as needed
        canvas.copy(&texture, None, Some(target_rect))?;

        // Present the updated canvas
        canvas.present();

        // Control the frame rate
        ::std::thread::sleep(Duration::from_millis(1000 / config.max_fps as u64));
    }

    Ok(())
}
