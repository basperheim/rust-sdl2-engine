import subprocess
import base64
import json
import time
import os
import sys
import threading

SCREEN = {'width': 800, 'height': 600}

class Sprite:
    def __init__(self, sprite_id, images, location, size, frame_rate=60):
        self.id = sprite_id
        self.images = images
        self.x = location['x']
        self.y = location['y']
        self.size = size
        self.frame_rate = frame_rate

    def as_dict(self) -> dict:
        return {
            'id': self.id,
            'images': self.images,
            'location': {'x': self.x, 'y': self.y},
            'size': self.size,
            'frame_rate': self.frame_rate
        }

    def move(self, x, y):
        self.x = x
        self.y = y

class GameEngine():
    def __init__(self, width, height, title, background, icon, fps=60):
        self.title = title
        self.width = width
        self.height = height
        self.background = background
        self.icon = icon
        self.fps = fps
        self.is_running = True

        # Sprite class instances go in here
        self.sprites = []

        # Other Rust-related I/O attributes
        self.process = self.start_process()
        self.stdout_thread = threading.Thread(target=self.read_stdout, daemon=True)
        self.stderr_thread = threading.Thread(target=self.read_stderr, daemon=True)
        self.stdout_thread.start()
        self.stderr_thread.start()
        self.start_time = time.time()

    def get_json_state(self) -> dict:
        return {
            "default_font": "Orbitron-Black.ttf",
            "window": {
                "width": self.width,
                "height": self.height,
                "title": self.title,
                "background": self.background,
                "icon_path": self.icon
            },
            "sprites": [sprite.as_dict() for sprite in self.sprites if sprite is not None],
            "text": [],
            "fps": self.fps
        }

    def start_process(self):
        binary_path = os.path.join('target', 'release', 'sdl2_rust')
        # binary_path = os.path.join('.', 'sdl2_rust')

        if not os.path.isfile(binary_path):
            print(f"Error: Binary not found at {binary_path}")
            sys.exit(1)

        if not os.access(binary_path, os.X_OK):
            print(f"Making the binary executable: {binary_path}")
            os.chmod(binary_path, 0o755)

        return subprocess.Popen(
            [binary_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

    def update(self):
        # Check if the process is still running
        if self.process.poll() is None:
            try:
                state_str = json.dumps(self.get_json_state())
                encoded_state = base64.b64encode(state_str.encode('utf-8')).decode('utf-8')
                self.process.stdin.write(encoded_state + '\n')
                self.process.stdin.flush()
                time.sleep(1 / self.fps)
            except BrokenPipeError:
                print("Rust process terminated. Unable to send game state.")
                self.is_running = False
            except Exception as e:
                print(f"Failed to send game state: {e}")
                self.is_running = False

    def read_stdout(self):
        try:
            for line in self.process.stdout:
                if line:
                    # Decode the line to a string and strip whitespace
                    line = line.strip()
                    print(f"Rust Output: {line}")

                    # Parse the JSON string
                    try:
                        event = json.loads(line)  # Deserialize the JSON string
                        action = event.get("action")

                        # Handle different actions
                        if action == "quit":
                            print("Received quit event from Rust process.")
                            self.is_running = False
                        elif action == "mouse_motion":
                            x = event.get("x")
                            y = event.get("y")
                            print(f"Mouse moved to: ({x}, {y})")
                        elif action == "key_down":
                            keycode = event.get("keycode")
                            print(f"Key pressed: {keycode}")
                        elif action == "key_up":
                            keycode = event.get("keycode")
                            print(f"Key released: {keycode}")
                        elif action == "mouse_button_down":
                            button = event.get("button")
                            x = event.get("x")
                            y = event.get("y")
                            print(f"Mouse button {button} down at ({x}, {y})")
                        elif action == "mouse_button_up":
                            button = event.get("button")
                            x = event.get("x")
                            y = event.get("y")
                            print(f"Mouse button {button} up at ({x}, {y})")

                    except json.JSONDecodeError:
                        print("Failed to decode JSON:", line)
                else:
                    break
        except Exception as e:
            print(f"Error reading stdout: {e}")

    def read_stderr(self):
        try:
            for line in self.process.stderr:
                if line:
                    print(f"Rust Error: {line.strip()}", file=sys.stderr)
                else:
                    break
        except Exception as e:
            print(f"Error reading stderr: {e}")

    def clean_up(self):
        if self.process.stdin:
            self.process.stdin.close()  # Close stdin safely
        self.process.terminate()  # Terminate the subprocess
        self.process.wait()  # Wait for it to finish
        self.stdout_thread.join()  # Wait for threads to finish
        self.stderr_thread.join()

def main():
    engine = GameEngine(
        SCREEN['width'],
        SCREEN['height'],
        title="My Game",
        background="background.jpeg",
        icon="cute-bunny.png", fps=60
    )

    # Create sprite instances
    tank1 = Sprite("tank1", ["tank-1.png", "tank-2.png"], {"x": 100, "y": 100}, {'width': 64, 'height': 64})
    tank2 = Sprite("tank2", ["tank-1.png", "tank-2.png"], {"x": 200, "y": 150}, {'width': 128, 'height': 128})

    try:
        while engine.is_running:

            # Move the sprite after a few seconds
            if time.time() > engine.start_time + 2:
                tank1.move(tank1.x + 1, 100)
                if tank1.x > SCREEN['width']:
                    tank1.move(10, 100)

            # Destroy tank #2 after a few seconds
            if time.time() > engine.start_time + 5:
                engine.sprites = [tank1]
            else:
                engine.sprites = [tank1, tank2]
            engine.update()

    except Exception as err:
        print(f"Game loop/state error: {err}")
    finally:
        # Ensure clean up on exit
        engine.clean_up()

if __name__ == "__main__":
    main()