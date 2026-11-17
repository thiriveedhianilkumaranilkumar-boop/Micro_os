#include <stdio.h>
#include <stdlib.h>
#include <time.h>

char board[3][3];

void init_board(void) {
    for (int r = 0; r < 3; r++)
        for (int c = 0; c < 3; c++)
            board[r][c] = ' ';
}

void print_board(void) {
    printf("\033[H\033[2J");
    printf("\033[1;36m=== MICROOS ARCADE: TIC-TAC-TOE ===\033[0m\n\n");
    printf("       1   2   3\n");
    printf("     +---+---+---+\n");
    for (int r = 0; r < 3; r++) {
        printf("   %d |", r + 1);
        for (int c = 0; c < 3; c++) {
            if (board[r][c] == 'X') {
                printf(" \033[1;32mX\033[0m |");
            } else if (board[r][c] == 'O') {
                printf(" \033[1;31mO\033[0m |");
            } else {
                printf("   |");
            }
        }
        printf("\n     +---+---+---+\n");
    }
    printf("\n");
}

int check_winner(char p) {
    for (int i = 0; i < 3; i++) {
        if (board[i][0] == p && board[i][1] == p && board[i][2] == p) return 1;
        if (board[0][i] == p && board[1][i] == p && board[2][i] == p) return 1;
    }
    if (board[0][0] == p && board[1][1] == p && board[2][2] == p) return 1;
    if (board[0][2] == p && board[1][1] == p && board[2][0] == p) return 1;
    return 0;
}

int is_board_full(void) {
    for (int r = 0; r < 3; r++)
        for (int c = 0; c < 3; c++)
            if (board[r][c] == ' ') return 0;
    return 1;
}

void cpu_move(void) {
    // Win if possible
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < 3; c++) {
            if (board[r][c] == ' ') {
                board[r][c] = 'O';
                if (check_winner('O')) return;
                board[r][c] = ' ';
            }
        }
    }
    // Block opponent win
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < 3; c++) {
            if (board[r][c] == ' ') {
                board[r][c] = 'X';
                if (check_winner('X')) {
                    board[r][c] = 'O';
                    return;
                }
                board[r][c] = ' ';
            }
        }
    }
    // Take center if free
    if (board[1][1] == ' ') { board[1][1] = 'O'; return; }
    // Random move
    int empty_r[9], empty_c[9], count = 0;
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < 3; c++) {
            if (board[r][c] == ' ') {
                empty_r[count] = r;
                empty_c[count] = c;
                count++;
            }
        }
    }
    if (count > 0) {
        int idx = rand() % count;
        board[empty_r[idx]][empty_c[idx]] = 'O';
    }
}

int main(void) {
    srand(time(NULL));
    init_board();

    printf("\033[2J\033[H");
    printf("\033[1;36m=== MICROOS ARCADE: TIC-TAC-TOE ===\033[0m\n\n");
    printf("Select Game Mode:\n");
    printf("  1. Player vs AI Computer\n");
    printf("  2. Two-Player (Pass & Play)\n");
    printf("Choice [1/2, or 0 to Exit]: ");
    fflush(stdout);

    int mode = 1;
    if (scanf("%d", &mode) != 1 || (mode != 1 && mode != 2)) {
        return 0;
    }

    char current_player = 'X';

    while (1) {
        print_board();

        if (mode == 1 && current_player == 'O') {
            printf("Computer (O) is thinking...\n");
            cpu_move();
        } else {
            int row, col;
            while (1) {
                printf("Player (%c), enter row and col (1-3, e.g. 1 2) [0 0 to quit]: ", current_player);
                fflush(stdout);
                if (scanf("%d %d", &row, &col) != 2) {
                    while (getchar() != '\n');
                    continue;
                }
                if (row == 0 && col == 0) return 0;
                row--; col--;
                if (row >= 0 && row < 3 && col >= 0 && col < 3 && board[row][col] == ' ') {
                    board[row][col] = current_player;
                    break;
                }
                printf("Invalid move! Cell is either occupied or out of bounds.\n");
            }
        }

        if (check_winner(current_player)) {
            print_board();
            printf("\033[1;32m*** PLAYER %c WINS! ***\033[0m\n", current_player);
            break;
        }

        if (is_board_full()) {
            print_board();
            printf("\033[1;33m*** IT'S A DRAW! ***\033[0m\n");
            break;
        }

        current_player = (current_player == 'X') ? 'O' : 'X';
    }

    printf("\nPress Enter to return to shell...");
    fflush(stdout);
    while (getchar() != '\n');
    getchar();
    return 0;
}
