import pygame, sys
pygame.init()
pygame.mixer.init()

# Game window and system 

size = width, height = 600, 600
screen = pygame.display.set_mode(size)
bg_color = (0, 0, 0)
screen.fill(bg_color) 
clock = pygame.time.Clock()
grav_timer = 0 # for acceleration
keytimer = [0,0] # for acceleration

pygame.mixer.music.load("/home/hedron/Music/ascension.mp3") # !!!!!! For the github viewers, change this path !!!!!!!
pygame.mixer.music.play(-1,0.0)

# Game objects
pos = [50,50]
shade_change = [0,0,0] #x, y, switchx
player_size = [30, 30]
ground = pygame.Rect(0, 570, 600, 30)
player = pygame.Rect(*pos, *player_size)
shade_color = (255, 193, 213)
shadea = pygame.Rect(pos[0] + shade_change[0], pos[1] - shade_change[1], *player_size)

while True:
    # quit handler
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    
    keys = pygame.key.get_pressed() #key handler

    #move handler

    if keys[pygame.K_LEFT] or keys[pygame.K_a]:
        pos[0] -= 10 + (5 * (keytimer[0]/10))
        keytimer[1] = 0
        if keytimer[0] <= 50:    
            keytimer[0] += 1
        if shade_change[0] > -50:
            shade_change[0] += 1
        if shade_change[2] != 1:
            shade_change[2] = 1
            shade_change[0] = 0
    if keys[pygame.K_RIGHT] or keys[pygame.K_d]:
        pos[0] += 10 + (5 * (keytimer[1]/10))
        keytimer[0] = 0
        if keytimer[1] <= 50:
            keytimer[1] += 1
        if shade_change[0] < 50:
            shade_change[0] -= 1
        if shade_change[2] != 0:
            shade_change[2] = 0
            shade_change[0] = 0
    if keys[pygame.K_UP] or keys[pygame.K_w]:
        pos[1] -= 20
    
    if pos[1] < 540:
        shade_change[1] -= 1
    else:
        shade_change[1] = 0

    #containment facility
    grav_timer += clock.get_time()
    if pos[1] == 540:
        grav_timer = 0
    if pos[1] != 540:
       pos[1] += 5 * (grav_timer/100) # gravity
    if pos[0] > 600:
        pos[0] = 0
    if pos[0] < 0:
        pos[0] = 600
    if pos[1] > 600:
        pos[1] = 540
    if pos[1] < 0:
        pos[1] = 540

    if player.colliderect(ground):
        pos[1] = 540
    
    screen.fill(bg_color)
    clock.tick(30)
    pygame.draw.rect(screen,(73,0,130),ground)
    player = pygame.Rect(*pos,*player_size)
    pygame.draw.rect(screen,(255,153,173),player)
    shadea = pygame.Rect(pos[0] + shade_change[0], pos[1] - shade_change[1], *player_size)
    pygame.draw.rect(screen,(shade_color),shadea)
    pygame.display.update()

# proto9:
# Added a blur mechanic, and removed the resize mechanic, added support for wasd (without the s) 
# Controls: Arrow keys to move | A to grow larger (x), D to grow smaller (x), S to grow larger (y), C to grow smaller (y)
# Ideas: I'd like to imagine the shade that follows your character as your characters dead sibling whos helping you on your journey. Don't ask me why I'm giving pink little cubes a storyline, I don't even know myself
# The game mechanics still feel pretty clunky :/

