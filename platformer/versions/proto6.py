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
sizex = 30
sizey = 30

while True:
    # quit handler
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    
    keys = pygame.key.get_pressed() #key handler

    #move handler
    if keys[pygame.K_LEFT]:
        posx -= 5
    if keys[pygame.K_RIGHT]:
        posx += 5
    if keys[pygame.K_DOWN]:
        posy += 5
    if keys[pygame.K_UP]:
        posy -= 5

    #resize handler
    if keys[pygame.K_a]:
        sizex += 5
    if keys[pygame.K_d]:
        sizex -= 5
    if keys[pygame.K_s]:
        sizey += 5
    if keys[pygame.K_c]:
        sizey -= 5
    
    if sizex <= 0:
        sizex = 5
    if sizey <= 0:
        sizey = 5

    #containment facility
    if posx > 600:
        posx = 0
    if posx < 0:
        posx = 600
    if posy > 600:
        posy = 0
    if posy < 0:
        posy = 600
    
    screen.fill(bg_color)
    clock.tick(30)
    pygame.draw.rect(screen,(255,153,173),pygame.Rect(posx,posy,sizex,sizey))
    pygame.display.update()

# proto5:
# This world has become a prison + resizement capaibilites^^
# Controls: Arrow keys to move | A to grow larger (x), D to grow smaller (x), S to grow larger (y), C to grow smaller (y)
