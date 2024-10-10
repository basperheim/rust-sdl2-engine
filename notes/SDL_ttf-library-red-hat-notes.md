Here's a detailed step-by-step guide on how you compiled and installed the SDL2_ttf library on your x86 Red Hat Linux system, including the necessary commands:

### Prerequisites
1. **Update System and Install Required Packages**:
   ```bash
   sudo dnf update
   sudo dnf install cmake wget gcc gcc-c++ make
   ```

2. **Install FreeType Development Library**:
   ```bash
   sudo dnf install freetype-devel
   ```

### Download and Extract SDL2_ttf
3. **Download the SDL2_ttf Source Code**:
   ```bash
   wget -O SDL2_ttf.tar.gz "https://github.com/libsdl-org/SDL_ttf/releases/download/release-2.22.0/SDL2_ttf-2.22.0.tar.gz"
   ```

4. **Extract the Archive**:
   ```bash
   tar -xvzf SDL2_ttf.tar.gz
   ```

5. **Change into the Extracted Directory**:
   ```bash
   cd SDL2_ttf-2.22.0/
   ```

### Compile and Install SDL2_ttf
6. **Compile the SDL2_ttf Library**:
   ```bash
   cmake -S . -B build
   cmake --build build
   cmake --install build
   ```

   If you encounter permissions errors during the installation step, you can either:
   - Use `sudo`:
     ```bash
     cmake -S . -B build && cmake --build build && sudo cmake --install build
     ```

   - Or switch to superuser:
     ```bash
     cmake -S . -B build && cmake --build build
     su -c 'cmake --install build'
     ```

### Copy Compiled Files Into Local LD Dir

```bash
cp libSDL2_ttf-2.0.so ~/.local/lib64/
cp libSDL2_ttf-2.0.so.0 ~/.local/lib64/
cp libSDL2_ttf-2.0.so.0.2200.0 ~/.local/lib64/
cp libSDL2_ttf.so ~/.local/lib64/
```

### Fixing Permissions (If Needed)
7. **If you installed the library locally and needed to fix permissions**:
   ```bash
   sudo chown -R user:user ~/.local/lib64/
   sudo chmod -R 755 ~/.local/lib64/
   ```

### Set Library Path
8. **Set the Library Path**:
   Export the library path to ensure that your application can find the SDL2_ttf library:
   ```bash
   export LD_LIBRARY_PATH=~/.local/lib64
   ```

   To make this change permanent, add it to your `~/.profile` file:
   ```bash
   vim ~/.profile
   ```
   Add the following line:
   ```bash
   export LD_LIBRARY_PATH=~/.local/lib64
   ```

   Then, source the profile:
   ```bash
   source ~/.profile
   ```

### Verification
9. **Check Installed Libraries**:
   Verify that the library is in place and accessible:
   ```bash
   ls -la ~/.local/lib64/
   ```

10. **Run Your Application**:
   Finally, test your application to ensure that it can locate and use the SDL2_ttf library:
   ```bash
   python3 oop_wrapper_test.py
   ```

### Summary
This guide outlines the entire process of compiling and installing the SDL2_ttf library, along with the steps to ensure that the library is accessible for your applications. You can document this process as a reference for future installations or for sharing with others. If you need any more details or adjustments, feel free to ask!
