import subprocess
import base64
import json
import time
import os
import sys
import threading

def main():
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
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
    )

    # Prepare the game state
    game_state = {
        "window": {
            "width": 800,
            "height": 600,
            "title": "My Game",
            "background": "images/background.jpeg",
            "icon_path": "images/cute-bunny.png"
        },
        "sprites": [
            {
                "id": "tank1",
                "images": ["images/tank-1.png", "images/tank-2.png"],
                "size": { 'width': 128, 'height': 128 },
                "location": { "x": 100, "y": 100 },
                "frame_rate": 3000
            },
            {
                "id": "tank2",
                "images": ["images/tank-1.png", "images/tank-2.png"],
                "size": { 'width': 64, 'height': 64 },
                "location": { "x": 200, "y": 150 },
                "frame_rate": 100
            }
        ],
        "fps": 30
    }

    # Function to send the game state to the Rust process
    def send_game_state(encoded_state):
        try:
            process.stdin.write(encoded_state + '\n')
            process.stdin.flush()
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

    # Send the initial game state
    json_str = json.dumps(game_state)
    encoded = base64.b64encode(json_str.encode('utf-8')).decode('utf-8')
    send_game_state(encoded)

    # Start threads to read stdout and stderr
    stdout_thread = threading.Thread(target=read_stdout, daemon=True)
    stderr_thread = threading.Thread(target=read_stderr, daemon=True)
    stdout_thread.start()
    stderr_thread.start()

    # Keep the main thread alive while the subprocess runs
    try:

        # Create a new mutable copy for updates
        updated_game_state = json.loads(json.dumps(game_state))

        while process.poll() is None:
            # Update sprite positions
            tank1_x = updated_game_state['sprites'][0]['location']['x']
            if tank1_x < updated_game_state['window']['width']:
                updated_game_state['sprites'][0]['location']['x'] += 1
            else:
                updated_game_state['sprites'][0]['location']['x'] = 0

            # Encode the updated game state
            json_str = json.dumps(updated_game_state)
            encoded = base64.b64encode(json_str.encode('utf-8')).decode('utf-8')
            send_game_state(encoded)
            fps = int(game_state.get('fps', 100))
            time.sleep(1 / fps)
    except KeyboardInterrupt:
        print("Terminating Rust process.")
        process.terminate()
    finally:
        process.stdin.close()
        process.wait()
        stdout_thread.join()
        stderr_thread.join()

if __name__ == "__main__":
    main()
