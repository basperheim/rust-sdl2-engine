# Rust SDL2 Game Engine

Welcome to the **Rust SDL2 Game Engine**! This project harnesses Rust's performance and safety features to build a 2D rendering engine using SDL2. Whether you're a game developer, a Rust enthusiast, or someone looking to learn systems programming with graphical capabilities, this engine serves as a solid foundation.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Setting Up SDL2 Paths](#setting-up-sdl2-paths)
- [Compilation and Running](#compilation-and-running)
- [Using the Python Wrapper](#using-the-python-wrapper)
- [Adding Assets](#adding-assets)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Features

- **2D Rendering:** Efficiently render sprites and backgrounds.
- **Event Handling:** Capture and respond to user interactions like keyboard and mouse events.
- **Python Integration:** Control the Rust engine using a high-level Python script for testing and automation.
- **Texture Management:** Load and cache textures to optimize performance.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust and Cargo:**

- **Rust:** A systems programming language focused on safety and performance.
- **Cargo:** Rust's package manager and build system.

**Installation:**

Visit the [official Rust website](https://www.rust-lang.org/tools/install) and follow the installation instructions. Alternatively, use the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, ensure Cargo is in your system's PATH:

```bash
source $HOME/.cargo/env
```

2. **SDL2 Library:**

- SDL2 is a cross-platform development library designed to provide low-level access to audio, keyboard, mouse, joystick, and graphics hardware.

**Installation via Homebrew (macOS):**

If you're on an ARM64 Mac (e.g., M1, M2), install SDL2 and SDL2_image using Homebrew:

```bash
brew install sdl2 sdl2_image
```

**Installation on Linux:**

For Debian/Ubuntu-based systems:

```bash
sudo apt-get update
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

**Installation on Windows:**

Follow the instructions in the [Rust-SDL2 guide](https://github.com/Rust-SDL2/rust-sdl2#user-content-windows).

3. **Python (Optional):**

- Required if you intend to use the Python wrapper to interact with the Rust binary.

**Installation:**

Visit the [official Python website](https://www.python.org/downloads/) and follow the installation instructions for your operating system. Ensure you have Python 3.6 or later.

## Installation

1. **Clone the Repository:**

Begin by cloning the project repository to your local machine.

```bash
git clone https://github.com/yourusername/rust-sdl2-game-engine.git
cd rust-sdl2-game-engine
```

2. **Set Up SDL2 Paths (macOS with Homebrew):**

If you installed SDL2 via Homebrew on an ARM64 Mac, you'll need to set environment variables to help Rust locate the SDL2 libraries.

Add the following lines to your shell profile (e.g., `~/.bash_profile`, `~/.zshrc`):

```bash
# Set environment variables for ARM64 Macs using Homebrew
export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH"
```

Apply the changes:

```bash
source ~/.bash_profile
# or
source ~/.zshrc
```

**Note:** Adjust the paths if your Homebrew installation is located elsewhere.

## Compilation and Running

1. **Clean Previous Builds (Optional):**

It's a good practice to clean previous builds to ensure a fresh compilation, especially after making significant changes.

```bash
cargo clean
```

2. **Build the Project:**

To compile the project in debug mode (faster compilation, unoptimized binary):

```bash
cargo build
```

For an optimized release build (slower compilation, optimized binary):

```bash
cargo build --release
```

3. **Run the Project:**

Debug Mode:

```bash
cargo run
```

**Release Mode:**

```bash
cargo run --release
```

**Running the Compiled Binary Directly:**

Debug Binary:

```bash
./target/debug/sdl2_rust
```

Release Binary:

```bash
./target/release/sdl2_rust
```

**Note:** Ensure you execute the binary from the project's root directory so that asset paths are correctly resolved.

## Using the Python Wrapper

A Python script (`sdl2_wrapper.py`) is provided to interact with the Rust binary. This allows you to send game states and receive events programmatically, facilitating testing and automation.

### **Prerequisites:**

- **Python 3.6 or Later:** Ensure Python is installed on your system. You can check your Python version with:

```bash
python3 --version
```

- **Standard Python Libraries:** The script uses only standard libraries (`subprocess`, `base64`, `json`, `time`, `os`, `sys`, `threading`), so no additional installations are necessary.

### **Running the Python Wrapper:**

1. **Ensure Assets are Correctly Placed:**

Make sure the `images` directory contains all necessary image files (`background.jpg`, `sprite1.png`, `sprite2.png`).

**Project Structure Example:**

```
rust-sdl2-game-engine/
├── Cargo.toml
├── Cargo.lock
├── images/
│   ├── background.jpg
│   ├── sprite1.png
│   └── sprite2.png
├── src/
│   └── main.rs
├── game.json
├── sdl2_wrapper.py
└── README.md
```

2. **Edit Game State (Optional):**

Modify `game.json` or `game2.json` with your desired game state configurations. Alternatively, you can adjust the Python script to generate custom game states on the fly.

3. **Run the Python Script:**

Execute the Python wrapper to start the Rust application and send the game state.

```bash
python3 sdl2_wrapper.py
```

**Script Overview:**

- **Starts** the Rust binary as a subprocess.
- **Encodes** a game state in Base64 and sends it to the Rust application via `stdin`.
- **Listens** for events emitted by the Rust process and prints them out.

4. **Example Output:**

```bash
Game state sent to Rust application.
Rust Output: Event: MouseButtonDown Left at (100, 150)
Rust Output: Event: MouseButtonUp Left at (100, 150)
Rust Output: Event: Quit
Rust process has terminated.
```

5. **Terminating the Python Wrapper:**

**Gracefully Terminate:** Press `Ctrl+C` in the terminal to terminate both the Python script and the Rust process.

### Customizing the Python Wrapper

- **Sending Multiple Game States:** Modify the Python script to send additional game states as needed. For example, to update sprite positions or handle game logic over time.

- **Handling Responses:** Enhance the script to process events or implement game logic based on the events received from the Rust application.

## Adding Assets

To enhance your game or application with new visuals, follow these steps to add new sprites or backgrounds:

1. **Place Image Files:**

Add your image files (e.g., PNG or JPG) to the `images/` directory.

2. **Update Game State:**

Modify the game state JSON (e.g., `game.json`) to reference the new images.

```json
{
  "window": {
    "width": 800,
    "height": 600,
    "background": "images/background.jpg"
  },
  "players": [],
  "sprites": [
    {
      "id": "1",
      "playerId": "1",
      "images": ["images/sprite1.png"],
      "location": { "x": 100, "y": 100 },
      "health": 100
    },
    {
      "id": "2",
      "playerId": "1",
      "images": ["images/sprite2.png"],
      "location": { "x": 200, "y": 150 },
      "health": 100
    }
  ]
}
```

3. **Send Updated Game State:**

Use the Python wrapper to send the updated game state to the Rust application.

```bash
python3 sdl2_wrapper.py
```

The Rust application will render the new sprites based on the updated game state.

## Troubleshooting

Encountering issues is a natural part of development. Below are common problems and their solutions to help you get back on track.

### 1. Rust Process Terminates Immediately When Run via Python

**Symptom:**

```bash
Game state sent to Rust application.
Rust process terminated.
```

**Possible Causes & Solutions:**

Error Loading Assets:
    - **Cause:** The Rust application might fail to locate or load the specified image files (`background.jpg`, `sprite1.png`, `sprite2.png`), leading to an immediate termination.
    
**Solution:
    - **Check Asset Paths:** Ensure that the image paths in your game state JSON are correct and relative to the project's root directory.
    - **Verify Asset Existence:** Confirm that all referenced images exist in the `images/` directory.
    - **Inspect Error Messages:** Run the Python script and check for any error messages printed under `Rust Error:`.

- **Subprocess Communication Issues:**

  - **Cause:** Improper handling of `stdin` or `stdout` in the Python script could cause the Rust process to misinterpret the communication, leading to unexpected termination.
  - **Solution:**
    - **Ensure Proper Encoding:** Verify that the game state is correctly encoded in Base64 before sending.
    - **Check for Exceptions:** Ensure that the Python script handles exceptions when sending data or reading outputs.

- **Path and Working Directory Mismatches:**
  - **Cause:** The Rust application might be running with a different working directory when invoked via Python, leading to incorrect relative paths for assets.
  - **Solution:**
    - **Run Python Script from Project Root:** Ensure that you execute the Python script from the project's root directory where the `images/` folder is located.

### 2. SDL2 Window Doesn't Open When Running via Python

Possible Causes & Solutions:
  - **Incorrect Asset Paths:** Double-check that the paths to your images in the game state JSON are correct and that the images exist in those locations.
  - **Missing SDL2 Libraries:** Ensure that SDL2 and SDL2_image are properly installed and that environment variables are correctly set (especially on macOS with Homebrew).
  - **Permissions Issues:** Ensure the Rust binary has execute permissions.

```bash
chmod +x ./target/debug/sdl2_rust
```

### 3. Permission Denied Errors

**Symptom:**

```bash
bash: ./target/debug/sdl2_rust: Permission denied
```

**Solution:**

Add execute permissions to the Rust binary:

```bash
chmod +x ./target/debug/sdl2_rust
```

### 4. SDL2 Libraries Not Found

**Symptom:**

Errors indicating that SDL2 libraries cannot be located during compilation or runtime.

**Solution:**

- **Verify Environment Variables:** Ensure that `LIBRARY_PATH` and `PKG_CONFIG_PATH` are correctly set, especially on macOS with Homebrew.
- **Reinstall SDL2 Libraries:** If issues persist, try reinstalling SDL2 and SDL2_image.

```bash
brew reinstall sdl2 sdl2_image
```

### 5. Unexpected Termination of the Rust Process

Possible Causes & Solutions:
  - **Unhandled Errors in Rust Code:** Add more detailed error logging in your Rust application to identify where it's failing.
  - **Broken Pipes:** Ensure the Python script maintains the `stdin` pipe open and doesn't terminate unexpectedly.
  - **Resource Constraints:** Monitor system resources to ensure the Rust application isn't crashing due to resource exhaustion.
