import subprocess
import base64
import json
import time
import os
import sys
import threading

# Determine the path to the compiled Rust binary
binary_path = os.path.join('target', 'debug', 'sdl2_rust')

# Verify that the binary exists
if not os.path.isfile(binary_path):
    print(f"Error: Binary not found at {binary_path}")
    sys.exit(1)

# Ensure the binary is executable
if not os.access(binary_path, os.X_OK):
    print(f"Making the binary executable: {binary_path}")
    os.chmod(binary_path, 0o755)

# Start the Rust application
process = subprocess.Popen(
    [binary_path],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,  # Capture stderr for debugging
    text=True,               # Use text mode for stdin/stdout
    bufsize=1,               # Line-buffered
)

# Prepare the game state
game_state = {
    "window": {
        "width": 800,
        "height": 600,
        "background": "images/background.jpeg"
    },
    "players": [],
    "sprites": [
        {
            "id": "1",
            "playerId": "1",
            "images": ["images/tank.png"],
            "location": { "x": 100, "y": 100 },
            "health": 100
        },
        {
            "id": "2",
            "playerId": "1",
            "images": ["images/tank.png"],
            "location": { "x": 200, "y": 150 },
            "health": 100
        }
    ]
}

# Encode the game state
json_str = json.dumps(game_state)
encoded = base64.b64encode(json_str.encode('utf-8')).decode('utf-8')

# Function to send the game state to the Rust process
def send_game_state(encoded_state):
    try:
        process.stdin.write(encoded_state + '\n')
        process.stdin.flush()
        print("Game state sent to Rust application.")
    except Exception as e:
        print(f"Failed to send game state: {e}")

# Function to read events from the Rust process's stdout
def read_stdout():
    try:
        for line in process.stdout:
            if line:
                print(f"Rust Output: {line.strip()}")
            else:
                break
    except Exception as e:
        print(f"Error reading stdout: {e}")

# Function to read errors from the Rust process's stderr
def read_stderr():
    try:
        for line in process.stderr:
            if line:
                print(f"Rust Error: {line.strip()}", file=sys.stderr)
            else:
                break
    except Exception as e:
        print(f"Error reading stderr: {e}", file=sys.stderr)

# Send the encoded game state
send_game_state(encoded)

# Start threads to read stdout and stderr
stdout_thread = threading.Thread(target=read_stdout, daemon=True)
stderr_thread = threading.Thread(target=read_stderr, daemon=True)
stdout_thread.start()
stderr_thread.start()

# Keep the main thread alive while the subprocess runs
try:
    while True:
        if process.poll() is not None:
            print("Rust process has terminated.")
            break
        time.sleep(0.1)
except KeyboardInterrupt:
    print("Terminating Rust process.")
    process.terminate()
    stdout_thread.join()
    stderr_thread.join()