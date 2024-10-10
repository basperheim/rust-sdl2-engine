# Understanding Binary Sizes and Making the Binary Portable

https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#bundled-feature

```bash
brew reinstall sdl2_image sdl2_ttf
brew install automake autoconf libtool pkg-config libb64 clion
brew reinstall cmake
```

Export clang paths as needed:

```bash
export CC=/usr/bin/cc
export CXX=/usr/bin/c++
RUST_BACKTRACE=1
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Rebuild Rust app:

```bash
cargo clean
cargo build --release
```

**Issue:**

- **Binary Size Difference:**
  - The release build binary (`sdl2_rust`) is significantly smaller (~766 KB) compared to the debug build (~1.6 MB).
- **Including SDL2 Libraries:**
  - You want the SDL2 libraries to be included in the binary itself to enhance portability, avoiding dependencies on system-installed SDL2 libraries.

**Explanation:**

1. **Binary Size Difference:**

   - **Debug Build (`target/debug`):**
     - Contains additional debug information and symbols.
     - Not optimized for size or speed.
   - **Release Build (`target/release`):**
     - Optimizations are enabled (`-C opt-level=3` by default).
     - Debug symbols are stripped out unless specifically retained.
     - Resulting binary is smaller and faster.

2. **Including SDL2 Libraries in the Binary:**

   By default, the `sdl2` crate links against the system-installed SDL2 libraries dynamically. This means that the compiled binary depends on the SDL2 shared libraries (`.dll`, `.so`, or `.dylib`) present on the system.

**Solution:**

To make your binary as portable as possible by including SDL2 libraries within it, you can:

1. **Use the `bundled` Feature of the `sdl2` Crate:**

   - This feature compiles SDL2 from source and statically links it into your binary.
   - This removes the dependency on system-installed SDL2 libraries.

2. **Modify Your `Cargo.toml`:**

   Update the `sdl2` dependency to include the `bundled` feature:

   ```toml
   [dependencies]
   sdl2 = { version = "0.37.0", features = ["image", "ttf", "bundled"] }
   ```

3. **Considerations When Using `bundled`:**

   - **Build Times:**
     - Compiling SDL2 from source will increase your build times.
   - **Dependencies:**
     - You need to have the necessary build tools and dependencies installed on your system.
     - For example, on Linux, you may need `autoconf`, `automake`, `libtool`, `pkg-config`, and development libraries for dependencies like `libX11`, `libXext`, etc.
   - **Platform-Specific Issues:**
     - The `bundled` feature may have different requirements or limitations depending on the target platform (Linux, macOS, Windows).

4. **Building the Binary:**

   - **Clean Build:**
     - Before building, run `cargo clean` to ensure previous builds don't interfere.
   - **Build Command:**
     - Use `cargo build --release` to build the optimized binary.

5. **Verify Static Linking:**

   - On Linux, you can use `ldd` to check the linked libraries:
     ```
     ldd target/release/sdl2_rust
     ```
     - Ideally, SDL2 libraries should not appear in the output if they are statically linked.
   - On macOS, use `otool -L`:
     ```
     otool -L target/release/sdl2_rust
     ```
   - On Windows, use `dumpbin` or check dependencies using tools like `Dependency Walker`.

6. **Alternative: Distribute SDL2 Libraries Alongside Your Binary**

   - If static linking is problematic, you can distribute the SDL2 shared libraries with your application.
   - Place the SDL2 library files in the same directory as your binary.
   - Ensure your application knows where to find these libraries at runtime.

**Example `Cargo.toml` Configuration:**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.21.0"
clap = { version = "4.3.10", features = ["derive"] }
sdl2 = { version = "0.37.0", features = ["image", "ttf", "bundled"] }  # Added "bundled"
```

**Notes:**

- **Ensure Build Tools Are Installed:**
  - On macOS, you may need to install additional tools via Homebrew:
    ```
    brew install automake autoconf libtool pkg-config
    ```
  - On Linux, use your distribution's package manager to install necessary tools.
- **Potential Issues:**
  - If you encounter build errors related to SDL2 or its dependencies, you may need to resolve those before the build can succeed.

**Alternative Crate: `rust-sdl2_ttf-static`**

- There is a crate called [`rust-sdl2_ttf-static`](https://crates.io/crates/rust-sdl2_ttf-static) that provides static linking for SDL2_ttf.
- However, integrating this may require additional configuration and is less commonly used.

## How Does the SDL2 Crate "bundled" Option Work

The `"bundled"` argument you see in the `Cargo.toml` file for the `sdl2` package is **not a general feature of Cargo** itself; instead, it is a **specific feature defined by the `sdl2` crate**. Here's a breakdown of how features work in Cargo and how they relate to your question:

### Understanding Features in Cargo

1. **What Are Features?**

   - In Cargo, features are a way to conditionally compile parts of a crate. They allow crate authors to enable or disable certain functionalities based on the needs of the user. Features are defined in the `Cargo.toml` of a crate.

2. **Defining Features:**

   - Features are specified in the `Cargo.toml` under a `[features]` section. For example, the `sdl2` crate may have a definition like this:

     ```toml
     [features]
     ttf = ["sdl2-sys/ttf"]
     image = ["sdl2-sys/image"]
     bundled = ["sdl2-sys/bundled"]
     ```

   - This allows users to enable these features when they include the crate as a dependency.

3. **Using Features:**

   - When you add a dependency to your `Cargo.toml`, you can specify features that you want to enable. For example:

     ```toml
     [dependencies]
     sdl2 = { version = "0.37.0", features = ["image", "ttf", "bundled"] }
     ```

   - Here, you're telling Cargo to enable the `image`, `ttf`, and `bundled` features for the `sdl2` crate.

### The `bundled` Feature

- The `bundled` feature in the `sdl2` crate specifically instructs Cargo to:

  - Download the SDL2 source code.
  - Compile it along with your Rust code and link it statically into your final binary.
  - This makes the binary more portable since it no longer depends on the system-installed SDL2 libraries.

- **Scope of `bundled`:**
  - The `bundled` feature is specific to the `sdl2` crate and is not a general feature that you can apply to all crates. Other crates may have their own features that do something similar, but they will not necessarily use the name `bundled`.

### General Features in Cargo

- While the `"bundled"` feature is specific to `sdl2`, the concept of features in Cargo is widely applicable. Different crates may have different features with various effects, such as:

  - Optional dependencies
  - Backend configurations
  - Enhanced functionalities (like `ttf` support in `sdl2`)

- Always refer to the documentation for the specific crate you are using to understand what features it provides and how they work.

### Conclusion

In summary:

- The `bundled` feature is specific to the `sdl2` crate and controls how that crate links to its dependencies.
- Features in general allow for flexible configurations within crates, but they are not universally applicable to all crates in the same way.
- Always consult the documentation for the crate to understand its features and how to use them effectively.

## Fixing Bundle Compile Errors

The error messages you're encountering indicate that the build system is looking for specific cross-compilation tools (`aarch64-w64-mingw32-clang` and `aarch64-w64-mingw32-clang++`), which are not found. This situation usually arises when you're trying to compile for a target architecture that does not match your host system, or the necessary cross-compilation tools are not installed.

### Steps to Resolve the Issue

1. **Ensure Correct Target Architecture**:

   - It seems that your build is trying to target `aarch64-apple-darwin`, which is for Apple Silicon (M1/M2 Macs). If you're on an Intel-based Mac, you may want to target `x86_64-apple-darwin` instead.
   - To specify the target explicitly, you can set it in your build command:
     ```bash
     cargo build --release --target=x86_64-apple-darwin
     ```

2. **Install Cross-Compilation Tools**:

   - If you actually want to compile for an `aarch64` target, you will need to install the relevant cross-compilation tools. For instance, on macOS, you can use Homebrew to install `llvm`:
     ```bash
     brew install llvm
     ```
   - After installing, you might need to set your `PATH` to include the tools:
     ```bash
     export PATH="/usr/local/opt/llvm/bin:$PATH"
     ```
   - You may also need to specify the toolchain in your Cargo configuration if you're using a cross-compilation tool.

3. **Check Your CMake Installation**:

   - Ensure that you have CMake installed and that it's accessible in your `PATH`. You can install it via Homebrew:
     ```bash
     brew install cmake
     ```

4. **Use Environment Variables for Compiler**:

   - If you need to specify the compiler directly for CMake, you can set the environment variables `CC` and `CXX` to point to your compiler:
     ```bash
     export CC=/path/to/your/clang
     export CXX=/path/to/your/clang++
     ```
   - You can add these lines to your `.bash_profile` or `.zshrc` (depending on your shell) to make them permanent.

5. **Verbose Build Logs**:

   - If you want more detailed output during the build process, you can set the following environment variable:
     ```bash
     export RUST_BACKTRACE=1
     ```
   - This will help you debug further if issues persist.

6. **Fallback to Non-Bundled Version**:

   - If you are still facing issues, consider temporarily removing the `"bundled"` feature to see if the build completes successfully with the system libraries instead:
     ```toml
     sdl2 = { version = "0.37.0", features = ["image", "ttf"] }  # Removed "bundled"
     ```

7. **Check Your SDL2 Installation**:
   - If you plan to use the bundled feature, ensure that your SDL2 development libraries are properly installed. If you're using a package manager, you may need to install SDL2 through it:
     ```bash
     brew install sdl2 sdl2_image sdl2_ttf
     ```

### Final Steps

After making these adjustments, try running the build command again:

```bash
cargo build --release
```

### Summary

- Verify your target architecture and set it explicitly if necessary.
- Ensure you have the necessary cross-compilation tools installed if you're targeting a different architecture.
- Make sure CMake is installed and properly configured.
- You can temporarily disable the `bundled` feature to avoid linking issues, especially if you want to check if the system libraries work first.

Let me know if you continue to experience issues, and I can help troubleshoot further!

## Create macOS DMG

```bash
ls -la target/release
total 6928
drwxr-xr-x   15   staff      480 Oct  9 17:14 .
drwxr-xr-x@   6   staff      192 Oct  9 13:54 ..
-rw-r--r--    1   staff        0 Oct  9 13:54 .cargo-lock
drwxr-xr-x   56   staff     1792 Oct  9 17:06 .fingerprint
drwxr-xr-x   18   staff      576 Oct  9 17:06 build
drwxr-xr-x  117   staff     3744 Oct  9 17:14 deps
drwxr-xr-x    2   staff       64 Oct  9 13:54 examples
drwxr-xr-x    2   staff       64 Oct  9 13:54 incremental
-rwxr-xr-x    1   staff  2475472 Oct  9 17:14 libSDL2-2.0.0.dylib
lrwxr-xr-x    1   staff       19 Oct  9 17:14 libSDL2-2.0.dylib -> libSDL2-2.0.0.dylib
lrwxr-xr-x    1   staff       17 Oct  9 17:14 libSDL2.dylib -> libSDL2-2.0.dylib
-rw-r--r--    1   staff   289536 Oct  9 17:14 libSDL2_test.a
-rw-r--r--    1   staff      744 Oct  9 17:14 libSDL2main.a
-rwxr-xr-x    1   staff   766568 Oct  9 17:14 sdl2_rust
-rw-r--r--    1   staff      246 Oct  9 13:54 sdl2_rust.d
```

The output from your build process indicates that the release binary (`sdl2_rust`) is dependent on the shared libraries (`libSDL2.dylib` and its associated files) located in the `target/release` directory. Here's a breakdown of what this means and how to package your application for distribution:

### 1. **Understanding Binary Dependencies**

- The **release binary** (`sdl2_rust`) is linked against the **shared libraries** you see in the `target/release` directory:

  - `libSDL2-2.0.0.dylib`: The actual dynamic library for SDL2.
  - `libSDL2.dylib`: A symlink pointing to the versioned library.
  - `libSDL2main.a`: The static library for the SDL2 main functionality.
  - `libSDL2_test.a`: The static library for SDL2 testing.

- When your binary runs, it expects these libraries to be available in the library path. If they are not found, you will encounter errors when trying to execute the binary.

### 2. **Creating a Portable Distribution**

To make your application portable, especially for macOS, you can package everything into a **DMG (Disk Image)** file. Here are the steps to do so:

#### Step 1: Create a Directory for Distribution

1. **Create a new directory** to hold your application and its dependencies:

```bash
mkdir MyGameApp
```

2. **Copy the Release Binary and Libraries** into this directory:

```bash
cp target/release/sdl2_rust MyGameApp/
cp target/release/libSDL2* MyGameApp/
```

#### Step 2: Set Up Your DMG

1. **Use a DMG Creation Tool**: You can use tools like `hdiutil` (which is built into macOS) to create a DMG file.

- First, navigate to the directory where you created `MyGameApp`:

```bash
cd MyGameApp
```

- Create the DMG file:

```bash
hdiutil create -volname MyGameApp -srcfolder . -ov -format UDZO MyGameApp.dmg
```

2. **Distribute the DMG**: Once created, you can distribute the `MyGameApp.dmg` file, which users can mount and run your application from.

### 3. **Considerations for Packaging**

- **Application Structure**: You might want to organize your application into a subdirectory. For example, you can create a folder named `MyGameApp.app/Contents/MacOS/` to place your binary, and create a `Resources` folder to hold any other assets (like fonts, images, etc.):

```bash
mkdir -p MyGameApp.app/Contents/MacOS
mkdir -p MyGameApp.app/Contents/Resources
cp target/release/sdl2_rust MyGameApp.app/Contents/MacOS/
cp target/release/libSDL2* MyGameApp.app/Contents/MacOS/
# Copy other resources (like fonts, images) to Resources
cp -R fonts MyGameApp.app/Contents/Resources/
```

- **App Bundles**: Creating an app bundle (`MyGameApp.app`) is the standard way to distribute applications on macOS, as it provides a clean structure and can contain metadata, icons, and more.

### 4. **Testing Your Distribution**

Before distributing your DMG, it's a good idea to test the application by copying it to a different directory or machine to ensure that it works correctly without requiring the original build directory.

### Summary

- Yes, you will need to package your release binary along with the associated libraries to ensure it runs correctly on other machines.
- Creating a DMG file is a great way to distribute your application on macOS.
- Consider organizing your application in a standard app bundle format for a more professional look and ease of use.

If you need further assistance with the steps or any other questions, feel free to ask!

## Bundling Not Working

Using the `sdl2` crate without the **bundled** feature means that your application will dynamically link against the SDL2 libraries that are installed on the system. Here's what this means for creating a standalone app or DMG for macOS, along with its consequences for portability:

### Consequences of Not Using the Bundled Feature

1. **Dynamic Linking**:

- Your application will rely on the system-installed versions of SDL2 and its dependencies (e.g., SDL2_image, SDL2_ttf). If these libraries are not present on the user's system or if they are the wrong version, your application may fail to run.

2. **Library Dependencies**:

- When distributing your application (e.g., as a DMG), you will need to ensure that the necessary libraries are either:
- Already installed on the user's system (which may not always be the case).
- Properly included with your application if you want to ensure it runs independently.

3. **User Experience**:

- If a user tries to run your application and does not have the required libraries, they will likely see error messages about missing dependencies. This can lead to frustration and confusion, especially if the user is not technically inclined.

### Making Your Application More Portable

To make your application more portable, here are some strategies you can consider:

1. **Include Required Libraries**:

- You can include the required SDL2 libraries in your DMG along with your application. This means you'll bundle the necessary `.dylib` files in a subdirectory (like `lib`) within the DMG and ensure your application knows where to find them at runtime.

For example, you might have:

```
MyApp.dmg/
├── MyApp.app/
└── lib/
  ├── libSDL2-2.0.dylib
  ├── libSDL2_image-2.0.dylib
  └── libSDL2_ttf-2.0.dylib
```

2. **Set rpath**:

- When building your application, you can use the rpath setting to tell the executable where to look for the dynamic libraries. This way, even if the libraries are included in a subdirectory, your app can find them.

You might modify your `build.rs` to look something like this:

```rust
fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../lib");
}
```

3. **Use a DMG Creation Tool**:

- When creating the DMG, consider using a tool that can properly set up the DMG with your application and its dependencies. Tools like `appdmg` can help automate this process.

4. **Inform Users**:

- If you decide to keep your application reliant on system-installed libraries, provide documentation or an installer script to guide users through installing the required dependencies. This way, they know what to expect and can easily set things up.

### Summary

- Not using the bundled feature means your application will depend on system-installed libraries, affecting portability.
- To create a standalone app or DMG, consider including the required libraries and setting the appropriate rpath to help your app find those libraries.
- Take the time to document the setup process or provide a friendly user experience to help users get started with your application.

By planning for these aspects, you can significantly improve the portability and user experience of your Rust SDL2 application. If you have any more questions or need further assistance, feel free to ask!

## Use Docker to Cross-Compile

Yes, you can definitely use Docker as a pseudo cross-platform Rust compiler to build applications like those using SDL2. This approach can help encapsulate your build environment, ensuring consistency across different platforms. Here's how you can achieve that, along with considerations for building on different architectures.

### Building a Standalone Rust Binary with SDL2 in Docker

1. **Create a Dockerfile**:

You can create a Dockerfile that installs the necessary libraries (like SDL2 and SDL2_ttf), compiles your Rust application, and then exits. Here's an example Dockerfile:

```dockerfile
# Use the official Rust image as a base
FROM rust:latest

# Install necessary libraries
RUN apt-get update && \
  apt-get install -y \
  libsdl2-dev \
  libsdl2-image-dev \
  libsdl2-ttf-dev \
  libfreetype6-dev \
  && rm -rf /var/lib/apt/lists/*

# Create a new directory for your app
WORKDIR /usr/src/app

# Copy your Cargo.toml and source code to the container
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build your application
RUN cargo build --release

# Specify the command to run when the container is started
CMD ["cargo", "run", "--release"]
```

This Dockerfile sets up a Rust environment, installs the required SDL2 libraries, and builds your Rust application.

2. **Building the Docker Image**:
   To build the Docker image, navigate to the directory containing your Dockerfile and run:

```bash
docker build -t rust-sdl2-app .
```

3. **Running the Container**:
   You can run the container to build the application:

```bash
docker run --rm rust-sdl2-app
```

The `--rm` flag removes the container after it exits.

4. **Extracting the Binary**:

If you want to extract the binary after building, you can run the container and copy the binary to your host:

```bash
docker run --rm -v "$(pwd):/output" rust-sdl2-app cp /usr/src/app/target/release/your_binary /output/
```

Replace `your_binary` with the name of your Rust binary.

### Cross-Platform Builds with Docker

#### On an ARM64 macOS Machine

Using Docker on an ARM64 macOS machine (with Apple Silicon) to build x86 binaries is feasible, especially with the use of "Buildx," which is a Docker CLI plugin for building images for multiple architectures. You can set up Docker to build for different architectures using QEMU emulation. Here's a quick overview:

1. **Enable Experimental Features**:

Make sure you have Docker Desktop set to use experimental features.

2. **Create a Buildx Builder**:

Create a new builder that supports multiple platforms:

```bash
docker buildx create --name mybuilder --use
docker buildx inspect --bootstrap
```

3. **Build for Different Architectures**:

You can specify the target architecture in your build command. For example, to build for `linux/amd64` (x86):

```bash
docker buildx build --platform linux/amd64 -t rust-sdl2-app .
```

4. **Run in Rosetta**:

If you're running an x86 Docker container on an ARM64 architecture (via Rosetta), it should generally work, but you might experience slower performance due to the emulation overhead.

### Limitations

- **Windows Targets**: While you can build Linux binaries, cross-compiling Windows applications directly from a Linux-based Docker container is more complex and typically requires a Windows container or a dedicated cross-compilation setup.
- **Performance**: Building in an emulated environment (like ARM running x86) may result in slower build times.

### Summary

Using Docker as a cross-platform Rust compiler is a powerful way to ensure your SDL2 applications are built consistently across different environments. It's definitely possible to build x86 binaries on an ARM64 macOS machine using Docker with QEMU emulation, though some limitations and performance considerations apply. If you need more specific guidance on any of these steps, feel free to ask!
