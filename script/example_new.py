import pygame
import os
import random
import sys
from multiprocessing import shared_memory

class TestObject:
    def __init__(self):
        self.bool1 = True
        self.array = ['string', True, 100]
        self.changing_attr = 0.0

class Main:
    def __init__(self, fullscreen=False) -> None:
        # Display setup
        self.width, self.height = 1280, 720
        self.display = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption('Showcase')
        
        # Shared memory setup (RGBA format)
        self.shm = shared_memory.SharedMemory(
            name="pygame_frame",
            create=True,
            size=self.width * self.height * 4  # 4 bytes per pixel (RGBA)
        )
        
        # Temporary RGBA surface for consistent format
        self.temp_surface = pygame.Surface((self.width, self.height), pygame.SRCALPHA)
        
        # Game state
        self.run = True
        self.clock = pygame.time.Clock()
        self._engine_mode = False
        self.angle = 0
        self.rotate = True
        self.rotation_speed = 1
        self.radius = 200
        self.center = (self.width // 2, self.height // 2)
        self.rect = pygame.Surface((100, 100))
        self.rect.fill((255, 255, 255))
        self.background_colour = pygame.Color(255, 100, 200)
        self.testObject = TestObject()

    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_w:
                    print('W Key Pressed!')
                elif event.key == pygame.K_r:
                    self.background_colour = pygame.Color(
                        random.randint(0, 255),
                        random.randint(0, 255),
                        random.randint(0, 255)
                    )
            if event.type == pygame.QUIT:
                self.run = False

    def update(self):
        import math
        self.nx = self.center[0] + self.radius * math.cos(math.radians(self.angle)) - 50
        self.ny = self.center[1] + self.radius * math.sin(math.radians(self.angle)) - 50
        if self.rotate:
            self.angle += self.rotation_speed
            if self.angle >= 360:
                self.angle -= 360
        self.testObject.changing_attr += 0.1

    def draw(self):
        # Draw to main display
        self.display.fill(self.background_colour)
        self.display.blit(self.rect, (self.nx, self.ny))
        
        # Copy to RGBA surface
        self.temp_surface.blit(self.display, (0, 0))
        
        # Get raw bytes and copy to shared memory
        pixels = pygame.image.tostring(self.temp_surface, "RGBA", True)
        self.shm.buf[:] = bytes(pixels)
        
        pygame.display.flip()

    def cleanup(self):
        """Release shared memory resources"""
        self.shm.close()
        self.shm.unlink()

    def test_run(self):
        """Main game loop"""
        try:
            while self.run:
                self.clock.tick(60)
                self.handle_events()
                self.update()
                self.draw()
        finally:
            self.cleanup()
            pygame.quit()
            sys.exit()

if __name__ == "__main__":
    game = Main()
    game.test_run()