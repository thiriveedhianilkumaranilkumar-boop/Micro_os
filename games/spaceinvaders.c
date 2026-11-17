#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>

#define FIELD_W 40
#define FIELD_H 20
#define NUM_ALIENS 18
#define MAX_BULLETS 8

typedef struct {
    int x, y;
    int alive;
} Alien;

typedef struct {
    int x, y;
    int active;
} Bullet;

static struct termios orig_termios;

void reset_terminal(void) {
    tcsetattr(STDIN_FILENO, TCSANOW, &orig_termios);
    printf("\033[?25h\033[0m\n");
    fflush(stdout);
}

void setup_terminal(void) {
    tcgetattr(STDIN_FILENO, &orig_termios);
    atexit(reset_terminal);
    struct termios raw = orig_termios;
    raw.c_lflag &= ~(ECHO | ICANON);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &raw);
    printf("\033[?25l");
    fflush(stdout);
}

int kbhit(void) {
    char ch;
    int n = read(STDIN_FILENO, &ch, 1);
    if (n == 1) {
        if (ch == '\033') {
            char seq[2];
            if (read(STDIN_FILENO, &seq[0], 1) == 1 && read(STDIN_FILENO, &seq[1], 1) == 1) {
                if (seq[0] == '[') {
                    if (seq[1] == 'C') return 'd'; // Right
                    if (seq[1] == 'D') return 'a'; // Left
                    if (seq[1] == 'A') return ' '; // Shoot
                }
            }
            return 'q';
        }
        return ch;
    }
    return 0;
}

int main(void) {
    setup_terminal();
    srand(time(NULL));

    int player_x = FIELD_W / 2;
    int score = 0;
    Alien aliens[NUM_ALIENS];
    Bullet bullets[MAX_BULLETS];

    for (int i = 0; i < MAX_BULLETS; i++) bullets[i].active = 0;

    // 3 rows of 6 aliens
    int idx = 0;
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < 6; c++) {
            aliens[idx].x = 6 + c * 5;
            aliens[idx].y = 2 + r * 2;
            aliens[idx].alive = 1;
            idx++;
        }
    }

    int alien_dir = 1;
    int alien_speed_counter = 0;
    int alien_tick_limit = 12;

    while (1) {
        int key = kbhit();
        if (key == 'q' || key == 'Q') return 0;
        if ((key == 'a' || key == 'A') && player_x > 2) player_x--;
        if ((key == 'd' || key == 'D') && player_x < FIELD_W - 3) player_x++;
        if (key == ' ' || key == 'w' || key == 'W') {
            for (int i = 0; i < MAX_BULLETS; i++) {
                if (!bullets[i].active) {
                    bullets[i].x = player_x;
                    bullets[i].y = FIELD_H - 2;
                    bullets[i].active = 1;
                    break;
                }
            }
        }

        // Update bullets
        for (int i = 0; i < MAX_BULLETS; i++) {
            if (bullets[i].active) {
                bullets[i].y--;
                if (bullets[i].y <= 0) {
                    bullets[i].active = 0;
                    continue;
                }
                // Check alien hit
                for (int a = 0; a < NUM_ALIENS; a++) {
                    if (aliens[a].alive && bullets[i].active &&
                        (bullets[i].x == aliens[a].x || bullets[i].x == aliens[a].x + 1) &&
                        bullets[i].y == aliens[a].y) {
                        aliens[a].alive = 0;
                        bullets[i].active = 0;
                        score += 25;
                    }
                }
            }
        }

        // Update alien movement
        alien_speed_counter++;
        if (alien_speed_counter >= alien_tick_limit) {
            alien_speed_counter = 0;
            int change_dir = 0;
            for (int a = 0; a < NUM_ALIENS; a++) {
                if (!aliens[a].alive) continue;
                if ((alien_dir == 1 && aliens[a].x >= FIELD_W - 4) ||
                    (alien_dir == -1 && aliens[a].x <= 2)) {
                    change_dir = 1;
                    break;
                }
            }
            if (change_dir) {
                alien_dir = -alien_dir;
                for (int a = 0; a < NUM_ALIENS; a++) {
                    if (aliens[a].alive) aliens[a].y++;
                }
            } else {
                for (int a = 0; a < NUM_ALIENS; a++) {
                    if (aliens[a].alive) aliens[a].x += alien_dir;
                }
            }
        }

        // Check alien victory (aliens reached player)
        int all_dead = 1;
        for (int a = 0; a < NUM_ALIENS; a++) {
            if (aliens[a].alive) {
                all_dead = 0;
                if (aliens[a].y >= FIELD_H - 2) {
                    printf("\033[H\033[1;31m*** INVASION SUCCEEDED! Game Over. Final Score: %d ***\033[0m\n", score);
                    printf("Press any key to return to shell...");
                    fflush(stdout);
                    while (!kbhit()) usleep(50000);
                    return 0;
                }
            }
        }

        if (all_dead) {
            printf("\033[H\033[1;32m*** EARTH SAVED! All aliens destroyed! Score: %d ***\033[0m\n", score);
            printf("Press any key to return to shell...");
            fflush(stdout);
            while (!kbhit()) usleep(50000);
            return 0;
        }

        // Render frame
        printf("\033[H");
        printf("\033[1;32m=== MICROOS ARCADE: SPACE INVADERS ===\033[0m\n");
        printf("Score: \033[1;33m%d\033[0m  (A/D: Move, Space: Shoot, Q: Quit)\n", score);

        for (int y = 0; y < FIELD_H; y++) {
            for (int x = 0; x < FIELD_W; x++) {
                if (y == 0 || y == FIELD_H - 1) {
                    printf("\033[1;34m-\033[0m");
                } else if (x == 0 || x == FIELD_W - 1) {
                    printf("\033[1;34m|\033[0m");
                } else if (y == FIELD_H - 2 && (x == player_x || x == player_x - 1 || x == player_x + 1)) {
                    printf("\033[1;36m%c\033[0m", (x == player_x) ? '^' : '=');
                } else {
                    int printed = 0;
                    // Check bullet
                    for (int b = 0; b < MAX_BULLETS; b++) {
                        if (bullets[b].active && bullets[b].x == x && bullets[b].y == y) {
                            printf("\033[1;33m|\033[0m");
                            printed = 1;
                            break;
                        }
                    }
                    if (!printed) {
                        for (int a = 0; a < NUM_ALIENS; a++) {
                            if (aliens[a].alive && aliens[a].y == y) {
                                if (aliens[a].x == x) { printf("\033[1;31m{\033[0m"); printed = 1; break; }
                                if (aliens[a].x + 1 == x) { printf("\033[1;31m}\033[0m"); printed = 1; break; }
                            }
                        }
                    }
                    if (!printed) printf(" ");
                }
            }
            printf("\n");
        }
        fflush(stdout);

        usleep(30000); // 30ms frame time
    }
    return 0;
}
