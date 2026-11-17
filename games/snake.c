#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>

#define WIDTH 40
#define HEIGHT 20

typedef struct {
    int x, y;
} Point;

static struct termios orig_termios;

void reset_terminal(void) {
    tcsetattr(STDIN_FILENO, TCSANOW, &orig_termios);
    printf("\033[?25h\033[0m\n"); // Show cursor, reset colors
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
    printf("\033[?25l"); // Hide cursor
    fflush(stdout);
}

int kbhit(void) {
    char ch;
    int n = read(STDIN_FILENO, &ch, 1);
    if (n == 1) {
        if (ch == '\033') { // Escape sequence (e.g. arrow keys)
            char seq[2];
            if (read(STDIN_FILENO, &seq[0], 1) == 1 && read(STDIN_FILENO, &seq[1], 1) == 1) {
                if (seq[0] == '[') {
                    if (seq[1] == 'A') return 'w'; // Up
                    if (seq[1] == 'B') return 's'; // Down
                    if (seq[1] == 'C') return 'd'; // Right
                    if (seq[1] == 'D') return 'a'; // Left
                }
            }
            return 'q';
        }
        return ch;
    }
    return 0;
}

void load_high_score(int *high) {
    *high = 0;
    FILE *f = fopen("/mnt/ssd/microos/scores_snake.txt", "r");
    if (!f) f = fopen("/tmp/scores_snake.txt", "r");
    if (f) {
        if (fscanf(f, "%d", high) != 1) *high = 0;
        fclose(f);
    }
}

void save_high_score(int score) {
    FILE *f = fopen("/mnt/ssd/microos/scores_snake.txt", "w");
    if (!f) f = fopen("/tmp/scores_snake.txt", "w");
    if (f) {
        fprintf(f, "%d\n", score);
        fclose(f);
    }
}

int main(void) {
    setup_terminal();
    srand(time(NULL));

    int highScore = 0;
    load_high_score(&highScore);

    while (1) {
        Point snake[WIDTH * HEIGHT];
        int length = 3;
        snake[0].x = WIDTH / 2;
        snake[0].y = HEIGHT / 2;
        snake[1].x = snake[0].x - 1;
        snake[1].y = snake[0].y;
        snake[2].x = snake[0].x - 2;
        snake[2].y = snake[0].y;

        int dirX = 1, dirY = 0;
        int score = 0;
        int gameOver = 0;
        int speedUs = 120000; // Base speed: 120ms

        Point food;
        food.x = 2 + rand() % (WIDTH - 4);
        food.y = 2 + rand() % (HEIGHT - 4);

        while (!gameOver) {
            int key = kbhit();
            if (key == 'q' || key == 'Q') {
                return 0;
            } else if ((key == 'w' || key == 'W') && dirY != 1) {
                dirX = 0; dirY = -1;
            } else if ((key == 's' || key == 'S') && dirY != -1) {
                dirX = 0; dirY = 1;
            } else if ((key == 'a' || key == 'A') && dirX != 1) {
                dirX = -1; dirY = 0;
            } else if ((key == 'd' || key == 'D') && dirX != -1) {
                dirX = 1; dirY = 0;
            }

            // Move body
            for (int i = length - 1; i > 0; i--) {
                snake[i] = snake[i - 1];
            }
            snake[0].x += dirX;
            snake[0].y += dirY;

            // Wall collision
            if (snake[0].x <= 0 || snake[0].x >= WIDTH - 1 ||
                snake[0].y <= 0 || snake[0].y >= HEIGHT - 1) {
                gameOver = 1;
            }

            // Self collision
            for (int i = 1; i < length; i++) {
                if (snake[0].x == snake[i].x && snake[0].y == snake[i].y) {
                    gameOver = 1;
                    break;
                }
            }

            // Food collision
            if (snake[0].x == food.x && snake[0].y == food.y) {
                if (length < WIDTH * HEIGHT - 1) length++;
                score += 10;
                if (score > highScore) {
                    highScore = score;
                    save_high_score(highScore);
                }
                if (speedUs > 40000) speedUs -= 2000; // Increase difficulty
                food.x = 2 + rand() % (WIDTH - 4);
                food.y = 2 + rand() % (HEIGHT - 4);
            }

            // Render frame
            printf("\033[H"); // Home cursor
            printf("\033[1;36m=== MICROOS ARCADE: SNAKE ===\033[0m\n");
            printf("Score: \033[1;32m%d\033[0m  High Score: \033[1;33m%d\033[0m  (Controls: WASD/Arrows, Q: Quit)\n", score, highScore);

            for (int y = 0; y < HEIGHT; y++) {
                for (int x = 0; x < WIDTH; x++) {
                    if (y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1) {
                        printf("\033[1;34m#\033[0m");
                    } else if (x == snake[0].x && y == snake[0].y) {
                        printf("\033[1;32m@\033[0m"); // Head
                    } else if (x == food.x && y == food.y) {
                        printf("\033[1;31m*\033[0m"); // Food
                    } else {
                        int isBody = 0;
                        for (int i = 1; i < length; i++) {
                            if (snake[i].x == x && snake[i].y == y) {
                                isBody = 1;
                                break;
                            }
                        }
                        if (isBody) printf("\033[0;32mo\033[0m");
                        else printf(" ");
                    }
                }
                printf("\n");
            }
            fflush(stdout);
            usleep(speedUs);
        }

        // Game over screen
        printf("\n\033[1;31m*** GAME OVER! Final Score: %d ***\033[0m\n", score);
        printf("Press \033[1;32m[R]\033[0m to Restart or \033[1;31m[Q]\033[0m to Return to Shell: ");
        fflush(stdout);

        while (1) {
            int ch = kbhit();
            if (ch == 'r' || ch == 'R') break;
            if (ch == 'q' || ch == 'Q' || ch == 27) return 0;
            usleep(20000);
        }
    }
    return 0;
}
