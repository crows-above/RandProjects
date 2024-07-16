import pygame, sys
pygame.init()
pygame.mixer.init()

# Game window and system 

size = width, height = 600, 600
screen = pygame.display.set_mode(size)
bg_color = (0, 0, 0)
screen.fill(bg_color) 
clock = pygame.time.Clock()

pygame.mixer.music.load("/home/hedron/Code/pythonities/games/platformer/assets/reflected-light.mp3")
pygame.mixer.music.play(-1,0.0)

# Game objects
player = pygame.draw.rect(screen,(255,153,173),pygame.Rect(30,30,30,30))


while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    
    clock.tick(30)
    pygame.display.update()

# proto4:
# Musicaa^^
