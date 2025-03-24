#!/bin/bash

# Had some issues getting the Rust binary to compile on Apple Silicon, so used to this to move missing SDL2 libs into the Rust project's root dir

source_dir=/opt/homebrew/lib
target_dir=~/Downloads/Coding-Projects/rust-sdl2-engine/libs
reg='^libSDL2'

# List all files in the source directory that match the pattern
# otool -L ./target/release/sdl2_rust
for lib in "$source_dir"/*; do
    # Get the base name of the file
    lib_name=$(basename "$lib")

    # Check if the library name matches the regex pattern
    if [[ "$lib_name" =~ $reg ]]; then
        # Copy the matching library to the target directory
        cp "$lib" "$target_dir" 2>/dev/null
        echo "Copied $lib_name to $target_dir"
    fi
done

