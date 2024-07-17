import pygame, sys
pygame.init()
pygame.mixer.init()

# Game window and system 

size = width, height = 600, 600
screen = pygame.display.set_mode(size)
bg_color = (0, 0, 0)
screen.fill(bg_color) 
clock = pygame.time.Clock()
grav_timer = 0
keytimer = [0,0]

pygame.mixer.music.load("/home/hedron/Code/pythonities/games/platformer/assets/reflected-light.mp3") # !!!!!! For the github viewers, change this path !!!!!!!
pygame.mixer.music.play(-1,0.0)

# Game objects
pos = [50,50]
player_size = [30, 30]
ground = pygame.Rect(0, 570, 600, 30)
player = pygame.Rect(*pos, *player_size)

while True:
    # quit handler
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
    
    keys = pygame.key.get_pressed() #key handler

    #move handler
    if keys[pygame.K_LEFT]:
        pos[0] -= 10 + (5 * (keytimer[0]/10))
        keytimer[1] = 0
        if keytimer[0] <= 50:    
            keytimer[0] += 1
    if keys[pygame.K_RIGHT]:
        pos[0] += 10 + (5 * (keytimer[1]/10))
        keytimer[0] = 0
        if keytimer[1] <= 50:
            keytimer[1] += 1
    if keys[pygame.K_UP]:
        pos[1] -= 20

    #resize handler
    if keys[pygame.K_a]:
        player_size[0] += 5
    if keys[pygame.K_d]:
        player_size[0] -= 5
    if keys[pygame.K_s]:
        player_size[1] += 5
    if keys[pygame.K_c]:
        player_size[1] -= 5
    
    if player_size[0] <= 0:
        player_size[0] = 5
    if player_size[1] <= 0:
        player_size[1] = 5

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
    pygame.display.update()

# proto8:
#Added acceleration, fixed the ground 
# Controls: Arrow keys to move | A to grow larger (x), D to grow smaller (x), S to grow larger (y), C to grow smaller (y)
# Ideas: Acceleration for gravity and horizontal movement, blurred trail
