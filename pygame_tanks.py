import pygame
import sys

class PygameSprite(pygame.sprite.Sprite):
    def __init__(self, sprite_id, image_path, location, size):
        super().__init__()
        self.image = pygame.image.load(image_path)  # Load the image
        self.image = pygame.transform.scale(self.image, (size['width'], size['height']))  # Scale the image
        self.rect = self.image.get_rect(topleft=location)  # Set the initial position

    def move(self, x, y):
        self.rect.topleft = (x, y)

class Background(pygame.sprite.Sprite):
    def __init__(self, image_file, location=(0, 0)):
        super().__init__()  # Call Sprite initializer
        self.image = pygame.image.load(image_file)
        self.image = pygame.transform.scale(self.image, (800, 600))  # Assume screen size is known
        self.rect = self.image.get_rect()
        self.rect.left, self.rect.top = location

def run_game():
    pygame.init()
    clock = pygame.time.Clock()
    fps = 60
    size = width, height = 800, 600
    screen = pygame.display.set_mode(size)
    pygame.display.set_caption("My Pygame Replica")

    # Create the background
    background = Background("images/background.jpeg")

    # Create sprite instances with correct image scaling
    tank1 = PygameSprite("tank1", "images/tank-1.png", (100, 100), {'width': 64, 'height': 64})
    tank2 = PygameSprite("tank2", "images/tank-2.png", (200, 150), {'width': 64, 'height': 64})
    sprites = pygame.sprite.Group(tank1, tank2)

    running = True
    start_time = pygame.time.get_ticks()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        # Update sprites
        if pygame.time.get_ticks() - start_time > 1000:  # After 1 second
            tank1.move(tank1.rect.x + 1, 100)

        if pygame.time.get_ticks() - start_time > 5000:  # After 5 seconds
            tank2.kill()  # Proper way to remove sprite in Pygame

        sprites.update()

        # Drawing
        screen.blit(background.image, background.rect)  # Draw the background
        sprites.draw(screen)
        pygame.display.flip()

        clock.tick(fps)

    pygame.quit()
    sys.exit()

if __name__ == '__main__':
    run_game()
