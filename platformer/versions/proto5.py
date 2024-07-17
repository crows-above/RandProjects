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
posx = 50
posy = 50

player = pygame.draw.rect(screen,(255,153,173),pygame.Rect(posx,posy,30,30))


while True:
    player = pygame.draw.rect(screen,(255,153,173),pygame.Rect(posx,posy,30,30))

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    
    keys = pygame.key.get_pressed() 
    if keys[pygame.K_LEFT]:
        posx -= 5
    if keys[pygame.K_RIGHT]:
        posx += 5
    if keys[pygame.K_DOWN]:
        posy += 5
    if keys[pygame.K_UP]:
        posy -= 5
    
    clock.tick(30)
    pygame.display.update()

# proto5:
# May it move freely^^
