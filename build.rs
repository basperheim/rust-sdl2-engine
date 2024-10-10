// export DYLD_LIBRARY_PATH=~/Downloads/Coding-Projects/rust-sdl2-engine/target/release
// export RUSTFLAGS='-L ~/Downloads/Coding-Projects/rust-sdl2-engine/libs'
// otool -L ./target/release/sdl2_rust

fn main() {
  // Specify the linker path for local libraries first
  println!("cargo:rustc-link-search=native=./libs");

  // Link the SDL2 libraries
  println!("cargo:rustc-link-lib=SDL2");
  println!("cargo:rustc-link-lib=SDL2_image");
  println!("cargo:rustc-link-lib=SDL2_ttf");

  // Set rpath for macOS to locate the libraries at runtime
  #[cfg(target_os = "macos")]
  {
      println!("cargo:rustc-link-arg=-Wl,-rpath,./libs");
  }

  #[cfg(target_os = "linux")]
  {
      println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
  }
}
