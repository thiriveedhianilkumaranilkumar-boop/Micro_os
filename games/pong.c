#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>

#define PONG_W 46
#define PONG_H 18
#define PADDLE_SIZE 4

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
                    if (seq[1] == 'A') return 'i'; // P2 Up
                    if (seq[1] == 'B') return 'k'; // P2 Down
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

    int p1_y = PONG_H / 2 - PADDLE_SIZE / 2;
    int p2_y = PONG_H / 2 - PADDLE_SIZE / 2;
    int ball_x = PONG_W / 2;
    int ball_y = PONG_H / 2;
    int ball_dx = 1, ball_dy = 1;
    int score1 = 0, score2 = 0;

    int two_player = 0;
    printf("\033[2J\033[H");
    printf("\033[1;36m=== MICROOS ARCADE: PONG ===\033[0m\n\n");
    printf("Select Mode:\n");
    printf("1. Single Player vs CPU\n");
    printf("2. Two-Player Mode (P1: W/S, P2: Up/Down or I/K)\n");
    printf("Choice [1/2]: ");
    fflush(stdout);

    while (1) {
        int c = kbhit();
        if (c == '1') { two_player = 0; break; }
        if (c == '2') { two_player = 1; break; }
        if (c == 'q' || c == 'Q') return 0;
        usleep(30000);
    }

    while (1) {
        int key = kbhit();
        if (key == 'q' || key == 'Q') return 0;
        if ((key == 'w' || key == 'W') && p1_y > 1) p1_y--;
        if ((key == 's' || key == 'S') && p1_y + PADDLE_SIZE < PONG_H - 1) p1_y++;

        if (two_player) {
            if ((key == 'i' || key == 'I') && p2_y > 1) p2_y--;
            if ((key == 'k' || key == 'K') && p2_y + PADDLE_SIZE < PONG_H - 1) p2_y++;
        } else {
            // CPU AI: track ball with slight lag
            if (ball_y > p2_y + PADDLE_SIZE / 2 && p2_y + PADDLE_SIZE < PONG_H - 1 && (rand() % 4 != 0)) p2_y++;
            if (ball_y < p2_y + PADDLE_SIZE / 2 && p2_y > 1 && (rand() % 4 != 0)) p2_y--;
        }

        // Ball movement
        ball_x += ball_dx;
        ball_y += ball_dy;

        // Top/Bottom bounce
        if (ball_y <= 1) { ball_y = 1; ball_dy = -ball_dy; }
        if (ball_y >= PONG_H - 2) { ball_y = PONG_H - 2; ball_dy = -ball_dy; }

        // Paddle 1 collision (x=2)
        if (ball_x == 2) {
            if (ball_y >= p1_y && ball_y < p1_y + PADDLE_SIZE) {
                ball_dx = 1;
                ball_x = 3;
            }
        }

        // Paddle 2 collision (x=PONG_W-3)
        if (ball_x == PONG_W - 3) {
            if (ball_y >= p2_y && ball_y < p2_y + PADDLE_SIZE) {
                ball_dx = -1;
                ball_x = PONG_W - 4;
            }
        }

        // Left miss
        if (ball_x < 1) {
            score2++;
            ball_x = PONG_W / 2;
            ball_y = PONG_H / 2;
            ball_dx = 1;
        }

        // Right miss
        if (ball_x >= PONG_W - 1) {
            score1++;
            ball_x = PONG_W / 2;
            ball_y = PONG_H / 2;
            ball_dx = -1;
        }

        // Render
        printf("\033[H");
        printf("\033[1;36m=== MICROOS ARCADE: PONG ===\033[0m\n");
        printf("Player 1: \033[1;32m%d\033[0m   |   Player 2 (%s): \033[1;33m%d\033[0m   (Q: Quit)\n",
               score1, two_player ? "Human" : "CPU", score2);

        for (int y = 0; y < PONG_H; y++) {
            for (int x = 0; x < PONG_W; x++) {
                if (y == 0 || y == PONG_H - 1) {
                    printf("\033[1;34m-\033[0m");
                } else if (x == 0 || x == PONG_W - 1) {
                    printf("\033[1;34m|\033[0m");
                } else if (x == 2 && y >= p1_y && y < p1_y + PADDLE_SIZE) {
                    printf("\033[1;32m|\033[0m");
                } else if (x == PONG_W - 3 && y >= p2_y && y < p2_y + PADDLE_SIZE) {
                    printf("\033[1;33m|\033[0m");
                } else if (x == ball_x && y == ball_y) {
                    printf("\033[1;31mO\033[0m");
                } else if (x == PONG_W / 2) {
                    printf(":");
                } else {
                    printf(" ");
                }
            }
            printf("\n");
        }
        fflush(stdout);

        if (score1 >= 10 || score2 >= 10) {
            printf("\n\033[1;32m*** MATCH FINISHED! Winner: %s ***\033[0m\n",
                   score1 >= 10 ? "Player 1" : (two_player ? "Player 2" : "CPU"));
            printf("Press any key to return to shell...");
            fflush(stdout);
            while (!kbhit()) usleep(50000);
            return 0;
        }

        usleep(45000); // Frame delay ~45ms
    }
    return 0;
}
