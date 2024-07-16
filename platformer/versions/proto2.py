import pygame, sys
pygame.init()

size = width, height = 600, 600
screen = pygame.display.set_mode(size)
bg_color = (255, 152, 173)
screen.fill(bg_color) 

while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    pygame.display.update()

# proto2:
# background color stuffs^^
