# Rust SDK Example

Rust offers both performance and safety, making it a great choice for building a 2D rendering SDK using SDL2.

### **1. Install Rust**

First, ensure you have Rust installed on your system. You can install Rust using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation.

### **2. Set Up a New Rust Project**

Create a new Rust project using Cargo, Rust's package manager and build system:

```bash
cargo new my_rust_sdl2_project
cd my_rust_sdl2_project
```

This command creates a new directory called `my_rust_sdl2_project` with a basic Rust project structure.

### **3. Add Dependencies**

Open the `Cargo.toml` file and add the necessary dependencies:

```toml
[dependencies]
sdl2 = { version = "0.35.3", features = ["image"] }
clap = { version = "4.3.10", features = ["derive"] }
```

- **sdl2**: Provides bindings to SDL2.
- **clap**: Simplifies command-line argument parsing.

### **4. Install SDL2 and SDL2_image**

#### **macOS**

Install using Homebrew:

```bash
brew install sdl2 sdl2_image
```

#### **Linux (Ubuntu/Debian)**

```bash
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

#### **Windows**

- Download SDL2 and SDL2_image development libraries from the [SDL website](https://www.libsdl.org/download-2.0.php).
- Ensure the DLLs are in your system's PATH or in the same directory as your executable.

### **5. Write the Rust Code**

Replace the contents of `src/main.rs` with the following code:

```rust
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
    #[arg(short, long, default_value_t = 600)]
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
```

### **6. Explanation of the Code**

- **Command-Line Arguments**: We use `clap` to parse command-line arguments for FPS, window size, and sprite path.
- **SDL2 Initialization**: Initializes SDL2 and the image subsystem for handling PNG and JPG files.
- **Window and Canvas**: Creates a window and a canvas (renderer) for drawing.
- **Event Handling**: Processes events like quitting the application, mouse clicks, and key presses.
- **Rendering**: Loads a texture from the specified sprite path and renders it onto the canvas.
- **Frame Rate Control**: Uses `std::thread::sleep` to control the frame rate based on `max_fps`.

### **7. Running the Project**

Run the application using:

```bash
cargo run -- --max-fps 60 --width 800 --height 600 --sprite-path path/to/your/sprite.png
```

Replace `path/to/your/sprite.png` with the actual path to your sprite image.

### **8. Basic Animation**

To implement basic animation:

- **Load Multiple Frames**: If your animation consists of multiple frames, load them into a vector.

  ```rust
  let frames = vec![
      texture_creator.load_texture("sprite_frame1.png")?,
      texture_creator.load_texture("sprite_frame2.png")?,
      // Add more frames as needed
  ];
  ```

- **Update Frame in the Main Loop**:

  ```rust
  let mut frame_index = 0;
  while is_running {
      // Event handling...

      // Clear the screen
      canvas.set_draw_color(Color::RGB(0, 0, 0));
      canvas.clear();

      // Render the current frame
      canvas.copy(&frames[frame_index], None, Some(target_rect))?;

      // Present the updated canvas
      canvas.present();

      // Update the frame index
      frame_index = (frame_index + 1) % frames.len();

      // Frame rate control...
  }
  ```

### **9. Mouse and Keyboard Events**

SDL2 provides a rich set of events:

- **Mouse Events**:

  ```rust
  match event {
      Event::MouseButtonDown { x, y, mouse_btn, .. } => {
          println!("Mouse button {:?} pressed at ({}, {})", mouse_btn, x, y);
      }
      Event::MouseMotion { x, y, xrel, yrel, .. } => {
          println!("Mouse moved to ({}, {}), relative movement ({}, {})", x, y, xrel, yrel);
      }
      _ => {}
  }
  ```

- **Keyboard Events**:

  ```rust
  match event {
      Event::KeyDown { keycode: Some(keycode), .. } => {
          println!("Key pressed: {:?}", keycode);
      }
      _ => {}
  }
  ```

### **10. Compile to a Binary**

For release builds:

```bash
cargo build --release
```

The binary will be located at `target/release/my_rust_sdl2_project`.

### **11. Cross-Platform Compatibility**

Rust applications compiled on one platform generally need to be recompiled on the target platform. For cross-compilation:

- **macOS to Linux/Windows**: Requires setting up cross-compilation toolchains.
- **Using Docker**: You can use Docker containers to build binaries for different platforms.

### **12. Additional Features**

- **Scaling Sprites**: Adjust the width and height in `target_rect` to scale sprites.
- **FPS Limiting**: Adjust `max_fps` via command-line arguments.
- **Window Resizing**: Handle `Event::Window { win_event, .. }` to respond to window size changes.

### **13. Helpful Tips**

- **Error Handling**: Use `Result<(), String>` in `main` to propagate errors.
- **Logging**: Use the `log` crate for advanced logging capabilities.
- **Organize Code**: As your project grows, consider splitting code into modules and separate files.

### **14. Resources**

- **Rust SDL2 Documentation**: [https://docs.rs/sdl2](https://docs.rs/sdl2)
- **SDL2 API Reference**: [https://wiki.libsdl.org/APIByCategory](https://wiki.libsdl.org/APIByCategory)
- **Clap Documentation**: [https://docs.rs/clap](https://docs.rs/clap)

### **15. Example Directory Structure**

```
my_rust_sdl2_project/
├── Cargo.toml
└── src
    └── main.rs
```

### **16. Running the Application**

Ensure that you have your sprite image in the correct path and run:

```bash
cargo run -- --sprite-path ./sprite.png
```

### **17. Troubleshooting**

- **Missing SDL2 Libraries**: If you get a build error about missing SDL2 libraries, ensure they are installed and accessible.
- **Incorrect Image Path**: Double-check the sprite path provided.
- **Permissions**: Ensure you have the necessary permissions to read the sprite file.

### **18. Conclusion**

You've now set up a basic Rust application using SDL2 that can:

- Create a window and renderer.
- Load and render sprites.
- Handle basic animation.
- Respond to mouse and keyboard events.
- Accept configurations via command-line arguments.

This foundation allows you to expand further, adding more sophisticated features like GUI elements, game logic, or integrating with other libraries.

If you have any questions or need assistance with specific parts of the code, feel free to ask!
