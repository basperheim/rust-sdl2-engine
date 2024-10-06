import subprocess
import base64
import json
import time
import os
import sys
import threading

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
            'location': { 'x': self.x, 'y': self.y },
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

        # Sprites list attributes
        self.sprites = [] # Sprite class instances go in here

        # Other Rust-related I/O attributes
        self.process = self.start_process()
        self.stdout_thread = threading.Thread(target=self.read_stdout, daemon=True)
        self.stderr_thread = threading.Thread(target=self.read_stderr, daemon=True)
        self.stdout_thread.start()
        self.stderr_thread.start()
        self.start_time = time.time()

    def get_json_state(self) -> dict:
        return {
            "window": {
                "width": self.width,
                "height": self.height,
                "title": self.title,
                "background": self.background,
                "icon_path": self.icon
            },
            "sprites": [sprite.as_dict() for sprite in self.sprites if sprite is not None],
            "fps": self.fps
        }

    def start_process(self):
        binary_path = os.path.join('target', 'debug', 'sdl2_rust')

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
                exc_type, exc_obj, exc_tb = sys.exc_info()
                print(f"Failed to send game state: {e} - type: {exc_type} - line #{exc_tb.tb_lineno}")
                self.is_running = False

    def read_stdout(self):
        try:
            for line in self.process.stdout:
                if line:
                    print(f"Rust Output: {line.strip()}")
                    # Check for a quit event from the Rust output
                    if "Event: Quit" in line.strip():
                        print("Received quit event from Rust process.")
                        self.is_running = False
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
    engine = GameEngine(800, 600, title="My Game", background="images/background.jpeg", icon="images/cute-bunny.png", fps=60)

    # Create sprite instances
    tank1 = Sprite("tank1", ["images/tank-1.png", "images/tank-2.png"], {"x": 100, "y": 100}, {'width': 64, 'height': 64})
    tank2 = Sprite("tank2", ["images/tank-1.png", "images/tank-2.png"], {"x": 200, "y": 150}, {'width': 128, 'height': 128})

    # Start the game loop
    try:
        while engine.is_running:
            # Move the sprite after a few seconds
            if time.time() > engine.start_time + 2:
                tank1.move(tank1.x + 1, 100)

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
