#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <termios.h>
#include <time.h>

#define SIZE 4

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
    raw.c_cc[VMIN] = 1;
    raw.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &raw);
    printf("\033[?25l");
    fflush(stdout);
}

int board[SIZE][SIZE];
int score = 0;

void add_random(void) {
    int empty_r[SIZE * SIZE], empty_c[SIZE * SIZE], count = 0;
    for (int r = 0; r < SIZE; r++) {
        for (int c = 0; c < SIZE; c++) {
            if (board[r][c] == 0) {
                empty_r[count] = r;
                empty_c[count] = c;
                count++;
            }
        }
    }
    if (count > 0) {
        int idx = rand() % count;
        board[empty_r[idx]][empty_c[idx]] = (rand() % 10 == 0) ? 4 : 2;
    }
}

int can_move(void) {
    for (int r = 0; r < SIZE; r++) {
        for (int c = 0; c < SIZE; c++) {
            if (board[r][c] == 0) return 1;
            if (c + 1 < SIZE && board[r][c] == board[r][c + 1]) return 1;
            if (r + 1 < SIZE && board[r][c] == board[r + 1][c]) return 1;
        }
    }
    return 0;
}

int has_won(void) {
    for (int r = 0; r < SIZE; r++) {
        for (int c = 0; c < SIZE; c++) {
            if (board[r][c] == 2048) return 1;
        }
    }
    return 0;
}

int slide_array(int arr[SIZE]) {
    int modified = 0;
    int temp[SIZE] = {0};
    int target = 0;

    for (int i = 0; i < SIZE; i++) {
        if (arr[i] != 0) {
            temp[target++] = arr[i];
        }
    }

    for (int i = 0; i < SIZE - 1; i++) {
        if (temp[i] != 0 && temp[i] == temp[i + 1]) {
            temp[i] *= 2;
            score += temp[i];
            temp[i + 1] = 0;
            i++;
            modified = 1;
        }
    }

    int result[SIZE] = {0};
    target = 0;
    for (int i = 0; i < SIZE; i++) {
        if (temp[i] != 0) {
            result[target++] = temp[i];
        }
    }

    for (int i = 0; i < SIZE; i++) {
        if (arr[i] != result[i]) modified = 1;
        arr[i] = result[i];
    }
    return modified;
}

int move_left(void) {
    int changed = 0;
    for (int r = 0; r < SIZE; r++) {
        if (slide_array(board[r])) changed = 1;
    }
    return changed;
}

int move_right(void) {
    int changed = 0;
    for (int r = 0; r < SIZE; r++) {
        int rev[SIZE];
        for (int c = 0; c < SIZE; c++) rev[c] = board[r][SIZE - 1 - c];
        if (slide_array(rev)) changed = 1;
        for (int c = 0; c < SIZE; c++) board[r][SIZE - 1 - c] = rev[c];
    }
    return changed;
}

int move_up(void) {
    int changed = 0;
    for (int c = 0; c < SIZE; c++) {
        int col[SIZE];
        for (int r = 0; r < SIZE; r++) col[r] = board[r][c];
        if (slide_array(col)) changed = 1;
        for (int r = 0; r < SIZE; r++) board[r][c] = col[r];
    }
    return changed;
}

int move_down(void) {
    int changed = 0;
    for (int c = 0; c < SIZE; c++) {
        int col[SIZE];
        for (int r = 0; r < SIZE; r++) col[r] = board[SIZE - 1 - r][c];
        if (slide_array(col)) changed = 1;
        for (int r = 0; r < SIZE; r++) board[SIZE - 1 - r][c] = col[r];
    }
    return changed;
}

const char* get_color(int val) {
    switch (val) {
        case 2: return "\033[1;37m";
        case 4: return "\033[1;36m";
        case 8: return "\033[1;33m";
        case 16: return "\033[1;32m";
        case 32: return "\033[1;35m";
        case 64: return "\033[1;31m";
        case 128: case 256: case 512: return "\033[1;34m";
        case 1024: case 2048: return "\033[1;33m";
        default: return "\033[0m";
    }
}

void render_board(void) {
    printf("\033[H");
    printf("\033[1;33m=== MICROOS ARCADE: 2048 ===\033[0m\n");
    printf("Score: \033[1;32m%d\033[0m  (Controls: WASD/Arrows, Q: Quit)\n\n", score);

    for (int r = 0; r < SIZE; r++) {
        printf("+------+------+------+------+\n|");
        for (int c = 0; c < SIZE; c++) {
            if (board[r][c] == 0) {
                printf("      |");
            } else {
                printf(" %s%4d\033[0m |", get_color(board[r][c]), board[r][c]);
            }
        }
        printf("\n");
    }
    printf("+------+------+------+------+\n");
    fflush(stdout);
}

int main(void) {
    setup_terminal();
    srand(time(NULL));

    memset(board, 0, sizeof(board));
    add_random();
    add_random();
    printf("\033[2J");

    while (1) {
        render_board();

        if (has_won()) {
            printf("\n\033[1;32m*** YOU WON! Reached 2048! ***\033[0m\n");
            printf("Press any key to return to shell...");
            fflush(stdout);
            char c; read(STDIN_FILENO, &c, 1);
            return 0;
        }

        if (!can_move()) {
            printf("\n\033[1;31m*** GAME OVER! No moves left. Score: %d ***\033[0m\n", score);
            printf("Press any key to return to shell...");
            fflush(stdout);
            char c; read(STDIN_FILENO, &c, 1);
            return 0;
        }

        char ch;
        int n = read(STDIN_FILENO, &ch, 1);
        if (n != 1) continue;

        if (ch == 'q' || ch == 'Q') return 0;
        if (ch == '\033') {
            char seq[2];
            if (read(STDIN_FILENO, &seq[0], 1) == 1 && read(STDIN_FILENO, &seq[1], 1) == 1) {
                if (seq[0] == '[') {
                    if (seq[1] == 'A') ch = 'w';
                    if (seq[1] == 'B') ch = 's';
                    if (seq[1] == 'C') ch = 'd';
                    if (seq[1] == 'D') ch = 'a';
                }
            }
        }

        int moved = 0;
        if (ch == 'w' || ch == 'W') moved = move_up();
        else if (ch == 's' || ch == 'S') moved = move_down();
        else if (ch == 'a' || ch == 'A') moved = move_left();
        else if (ch == 'd' || ch == 'D') moved = move_right();

        if (moved) {
            add_random();
        }
    }
    return 0;
}
