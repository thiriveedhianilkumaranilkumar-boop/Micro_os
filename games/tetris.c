#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>
#include <time.h>

#define BOARD_W 10
#define BOARD_H 20

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
                    if (seq[1] == 'A') return 'w'; // Rotate
                    if (seq[1] == 'B') return 's'; // Drop
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

// 7 Tetrominoes (4x4 blocks)
static const int TETROMINOES[7][4][4] = {
    // I
    {{0,0,0,0},{1,1,1,1},{0,0,0,0},{0,0,0,0}},
    // J
    {{1,0,0,0},{1,1,1,0},{0,0,0,0},{0,0,0,0}},
    // L
    {{0,0,1,0},{1,1,1,0},{0,0,0,0},{0,0,0,0}},
    // O
    {{1,1,0,0},{1,1,0,0},{0,0,0,0},{0,0,0,0}},
    // S
    {{0,1,1,0},{1,1,0,0},{0,0,0,0},{0,0,0,0}},
    // T
    {{0,1,0,0},{1,1,1,0},{0,0,0,0},{0,0,0,0}},
    // Z
    {{1,1,0,0},{0,1,1,0},{0,0,0,0},{0,0,0,0}}
};

int board[BOARD_H][BOARD_W];
int currentPiece[4][4];
int curX, curY;
int score = 0;
int linesCleared = 0;

void rotate(int piece[4][4]) {
    int temp[4][4];
    for (int y = 0; y < 4; y++) {
        for (int x = 0; x < 4; x++) {
            temp[x][3 - y] = piece[y][x];
        }
    }
    memcpy(piece, temp, sizeof(temp));
}

int check_collision(int piece[4][4], int px, int py) {
    for (int y = 0; y < 4; y++) {
        for (int x = 0; x < 4; x++) {
            if (piece[y][x]) {
                int nx = px + x;
                int ny = py + y;
                if (nx < 0 || nx >= BOARD_W || ny >= BOARD_H) return 1;
                if (ny >= 0 && board[ny][nx]) return 1;
            }
        }
    }
    return 0;
}

void lock_piece(void) {
    for (int y = 0; y < 4; y++) {
        for (int x = 0; x < 4; x++) {
            if (currentPiece[y][x]) {
                int ny = curY + y;
                int nx = curX + x;
                if (ny >= 0 && ny < BOARD_H && nx >= 0 && nx < BOARD_W) {
                    board[ny][nx] = 1;
                }
            }
        }
    }
}

void clear_lines(void) {
    for (int y = BOARD_H - 1; y >= 0; y--) {
        int full = 1;
        for (int x = 0; x < BOARD_W; x++) {
            if (!board[y][x]) { full = 0; break; }
        }
        if (full) {
            linesCleared++;
            score += 100;
            for (int k = y; k > 0; k--) {
                for (int x = 0; x < BOARD_W; x++) {
                    board[k][x] = board[k - 1][x];
                }
            }
            for (int x = 0; x < BOARD_W; x++) board[0][x] = 0;
            y++; // Check same line again
        }
    }
}

void spawn_piece(void) {
    int type = rand() % 7;
    memcpy(currentPiece, TETROMINOES[type], sizeof(currentPiece));
    curX = BOARD_W / 2 - 2;
    curY = 0;
}

void render(void) {
    printf("\033[H");
    printf("\033[1;35m=== MICROOS ARCADE: TETRIS ===\033[0m\n");
    printf("Score: \033[1;32m%d\033[0m  Lines: \033[1;33m%d\033[0m  (A/D: Move, W: Rotate, S: Drop, Q: Quit)\n", score, linesCleared);

    for (int y = 0; y < BOARD_H; y++) {
        printf("\033[1;34m|\033[0m");
        for (int x = 0; x < BOARD_W; x++) {
            int isPiece = 0;
            if (y >= curY && y < curY + 4 && x >= curX && x < curX + 4) {
                if (currentPiece[y - curY][x - curX]) isPiece = 1;
            }
            if (isPiece) {
                printf("\033[1;33m[]\033[0m");
            } else if (board[y][x]) {
                printf("\033[1;36m##\033[0m");
            } else {
                printf(" .");
            }
        }
        printf("\033[1;34m|\033[0m\n");
    }
    printf("\033[1;34m+--------------------+\033[0m\n");
    fflush(stdout);
}

int main(void) {
    setup_terminal();
    srand(time(NULL));

    memset(board, 0, sizeof(board));
    spawn_piece();

    int tick = 0;
    int dropSpeed = 25; // 25 ticks * 20ms = 500ms

    while (1) {
        int key = kbhit();
        if (key == 'q' || key == 'Q') return 0;
        else if (key == 'a' || key == 'A') {
            if (!check_collision(currentPiece, curX - 1, curY)) curX--;
        } else if (key == 'd' || key == 'D') {
            if (!check_collision(currentPiece, curX + 1, curY)) curX++;
        } else if (key == 's' || key == 'S') {
            if (!check_collision(currentPiece, curX, curY + 1)) {
                curY++;
                score += 1;
            }
        } else if (key == 'w' || key == 'W') {
            int temp[4][4];
            memcpy(temp, currentPiece, sizeof(temp));
            rotate(temp);
            if (!check_collision(temp, curX, curY)) {
                memcpy(currentPiece, temp, sizeof(currentPiece));
            }
        }

        tick++;
        if (tick >= dropSpeed) {
            tick = 0;
            if (!check_collision(currentPiece, curX, curY + 1)) {
                curY++;
            } else {
                lock_piece();
                clear_lines();
                spawn_piece();
                if (check_collision(currentPiece, curX, curY)) {
                    // Game Over
                    render();
                    printf("\n\033[1;31m*** GAME OVER! Final Score: %d ***\033[0m\n", score);
                    printf("Press any key to return to shell...");
                    fflush(stdout);
                    while (!kbhit()) usleep(50000);
                    return 0;
                }
            }
            if (linesCleared > 5 && dropSpeed > 10) dropSpeed = 20;
            if (linesCleared > 10 && dropSpeed > 7) dropSpeed = 15;
            if (linesCleared > 20 && dropSpeed > 4) dropSpeed = 10;
        }

        render();
        usleep(20000); // 20ms frame time
    }
    return 0;
}
