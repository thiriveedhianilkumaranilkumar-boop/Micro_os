#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <termios.h>
#include <time.h>

#define M_WIDTH 12
#define M_HEIGHT 10
#define NUM_MINES 15

typedef struct {
    int is_mine;
    int is_revealed;
    int is_flagged;
    int neighbor_mines;
} Cell;

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

Cell grid[M_HEIGHT][M_WIDTH];
int cur_x = 0, cur_y = 0;

void init_game(void) {
    memset(grid, 0, sizeof(grid));
    int placed = 0;
    while (placed < NUM_MINES) {
        int x = rand() % M_WIDTH;
        int y = rand() % M_HEIGHT;
        if (!grid[y][x].is_mine) {
            grid[y][x].is_mine = 1;
            placed++;
        }
    }

    // Calculate neighbors
    for (int y = 0; y < M_HEIGHT; y++) {
        for (int x = 0; x < M_WIDTH; x++) {
            if (grid[y][x].is_mine) continue;
            int count = 0;
            for (int dy = -1; dy <= 1; dy++) {
                for (int dx = -1; dx <= 1; dx++) {
                    int ny = y + dy;
                    int nx = x + dx;
                    if (ny >= 0 && ny < M_HEIGHT && nx >= 0 && nx < M_WIDTH) {
                        if (grid[ny][nx].is_mine) count++;
                    }
                }
            }
            grid[y][x].neighbor_mines = count;
        }
    }
}

void reveal(int x, int y) {
    if (x < 0 || x >= M_WIDTH || y < 0 || y >= M_HEIGHT) return;
    if (grid[y][x].is_revealed || grid[y][x].is_flagged) return;

    grid[y][x].is_revealed = 1;
    if (grid[y][x].neighbor_mines == 0 && !grid[y][x].is_mine) {
        for (int dy = -1; dy <= 1; dy++) {
            for (int dx = -1; dx <= 1; dx++) {
                reveal(x + dx, y + dy);
            }
        }
    }
}

int check_win(void) {
    for (int y = 0; y < M_HEIGHT; y++) {
        for (int x = 0; x < M_WIDTH; x++) {
            if (!grid[y][x].is_mine && !grid[y][x].is_revealed) return 0;
        }
    }
    return 1;
}

void render_grid(int reveal_all) {
    printf("\033[H");
    printf("\033[1;36m=== MICROOS ARCADE: MINESWEEPER ===\033[0m\n");
    printf("Mines: \033[1;31m%d\033[0m  (WASD: Move, Space/Enter: Reveal, F: Flag, Q: Quit)\n\n", NUM_MINES);

    printf("    ");
    for (int x = 0; x < M_WIDTH; x++) printf(" %2d", x + 1);
    printf("\n   +");
    for (int x = 0; x < M_WIDTH; x++) printf("---");
    printf("-+\n");

    for (int y = 0; y < M_HEIGHT; y++) {
        printf("%2d |", y + 1);
        for (int x = 0; x < M_WIDTH; x++) {
            int is_cursor = (x == cur_x && y == cur_y);
            if (is_cursor) printf("\033[7m"); // Reverse video for cursor

            if (reveal_all && grid[y][x].is_mine) {
                printf(" * ");
            } else if (grid[y][x].is_flagged) {
                printf("\033[1;33m F \033[0m");
                if (is_cursor) printf("\033[7m");
            } else if (!grid[y][x].is_revealed) {
                printf(" . ");
            } else if (grid[y][x].is_mine) {
                printf("\033[1;31m * \033[0m");
                if (is_cursor) printf("\033[7m");
            } else if (grid[y][x].neighbor_mines > 0) {
                printf(" %d ", grid[y][x].neighbor_mines);
            } else {
                printf("   ");
            }

            if (is_cursor) printf("\033[0m");
        }
        printf("|\n");
    }

    printf("   +");
    for (int x = 0; x < M_WIDTH; x++) printf("---");
    printf("-+\n");
    fflush(stdout);
}

int main(void) {
    setup_terminal();
    srand(time(NULL));

    init_game();
    printf("\033[2J");

    while (1) {
        render_grid(0);

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

        if ((ch == 'w' || ch == 'W') && cur_y > 0) cur_y--;
        if ((ch == 's' || ch == 'S') && cur_y < M_HEIGHT - 1) cur_y++;
        if ((ch == 'a' || ch == 'A') && cur_x > 0) cur_x--;
        if ((ch == 'd' || ch == 'D') && cur_x < M_WIDTH - 1) cur_x++;

        if (ch == 'f' || ch == 'F') {
            if (!grid[cur_y][cur_x].is_revealed) {
                grid[cur_y][cur_x].is_flagged = !grid[cur_y][cur_x].is_flagged;
            }
        }

        if (ch == ' ' || ch == '\n' || ch == '\r') {
            if (!grid[cur_y][cur_x].is_flagged) {
                if (grid[cur_y][cur_x].is_mine) {
                    render_grid(1);
                    printf("\n\033[1;31m*** BOOM! You hit a mine! Game Over. ***\033[0m\n");
                    printf("Press any key to return to shell...");
                    fflush(stdout);
                    read(STDIN_FILENO, &ch, 1);
                    return 0;
                } else {
                    reveal(cur_x, cur_y);
                    if (check_win()) {
                        render_grid(1);
                        printf("\n\033[1;32m*** CONGRATULATIONS! You cleared the minefield! ***\033[0m\n");
                        printf("Press any key to return to shell...");
                        fflush(stdout);
                        read(STDIN_FILENO, &ch, 1);
                        return 0;
                    }
                }
            }
        }
    }
    return 0;
}
