import pygame, os, random
pygame.init()
"""
All code given is the bare minimum to safely run code within the engine.
Removing any code that already exists in not recommended and you WILL run into issues.
When compiled, parts of code are removed to optimise and simplify the file.
"""


class TestObject:

    def __init__(self):
        self.bool1 = True
        self.array = ['string', True, 100]
        self.changing_attr = 0.0


class Main:

    def __init__(self, fullscreen=False) ->None:
        self.display = pygame.display.set_mode((1280, 720))
        pygame.display.set_caption('Showcase')
        self.run = True
        self.clock = pygame.time.Clock()
        self._engine_mode = False
        """
        Make sure not to remove the super() method above, as it will break the whole script.
        """
        self.display = pygame.display.get_surface()
        if self._engine_mode:
            abspath = os.path.abspath(__file__)
            dname = os.path.dirname(abspath)
            os.chdir(dname)
        self.angle = 0
        self.rotate = True
        self.rotation_speed = 1
        self.radius = 200
        self.center = self.display.get_width() // 2, self.display.get_height(
            ) // 2
        self.rect = pygame.Surface((100, 100))
        self.rect.fill((255, 255, 255))
        self.publicAttr = 55.0
        self.stringVal = 'hello'
        self.tupleVal = 0, 1, 2
        self.listVal = ['hello', 'world']
        self.arrayVal = ['Hello', 100.0]
        self.testObject = TestObject()
        self.background_colour = pygame.Color(255, 100, 200)
        self.square_rect = self.rect.get_rect()

    def handle_events(self):
        """
        All your logic for handling events should go here.
        Its recommended you write code to do with event handling here.
        Make sure that you don't remove the `pygame.QUIT` event as the game won't be able to be shutdown.
        See pygame docs for more info: https://www.pygame.org/docs/ref/event.html.
        """
        for event in pygame.event.get():
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_w:
                    print('W Key Pressed!')
                elif event.key == pygame.K_r:
                    self.background_colour = random.randint(0, 255
                        ), random.randint(0, 255), random.randint(0, 255)
            if event.type == pygame.QUIT:
                self.run = False

    def update(self):
        import math
        """
        This is where you independant code goes.
        This is purely a conceptual seperator from the rest of the game code.
        Think of this as the "body" of your program.
        """
        self.nx = self.center[0] + self.radius * math.cos(math.radians(self
            .angle)) - 100 // 2
        self.ny = self.center[1] + self.radius * math.sin(math.radians(self
            .angle)) - 100 // 2
        if self.rotate:
            self.angle += self.rotation_speed
            if self.angle >= 360:
                self.angle -= 360
        self.testObject.changing_attr += 0.1

    def draw(self):
        """
        This is where your drawing code should do.
        Make sure that `pygame.display.flip()` is the last line.
        Make sure that `self.display.fill()` is at the start too.

        """
        self.display.fill(self.background_colour)
        self.display.blit(self.rect, (self.nx, self.ny))
        pygame.display.flip()

    def test_run(self):
        """Handles the running of the game"""
        while self.run:
            self.clock.tick()
            self.handle_events()
            self.update()
            self.draw()
        pygame.quit()
        sys.exit()


game = Main()
game.test_run()
