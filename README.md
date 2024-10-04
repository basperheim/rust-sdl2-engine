# Rust SDL2 Game Engine

Rust offers both performance and safety, making it a great choice for building a 2D rendering SDK using SDL2.

## Set SDL2 Paths

You'll need to instal SDL2 and set the paths.

For a Homebrew installation of `sdl2` add something like this to your bash profile:

```bash
# Set environment variables for ARM64 Macs using Homebrew
export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig:$PKG_CONFIG_PATH"
```

## Compile and Run

```bash
cargo clean
cargo build
cargo run -- --sprite-path ./images/example-tank.jpeg
```
