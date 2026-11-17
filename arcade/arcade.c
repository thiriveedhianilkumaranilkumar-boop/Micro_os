#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void show_menu(void) {
    printf("\033[2J\033[H");
    printf("\033[1;36m========================================\033[0m\n");
    printf("\033[1;33m             MICRO ARCADE               \033[0m\n");
    printf("\033[1;36m========================================\033[0m\n");
    printf("  \033[1;32m1.\033[0m Snake\n");
    printf("  \033[1;32m2.\033[0m Tetris\n");
    printf("  \033[1;32m3.\033[0m Pong\n");
    printf("  \033[1;32m4.\033[0m Minesweeper\n");
    printf("  \033[1;32m5.\033[0m 2048\n");
    printf("  \033[1;32m6.\033[0m Number Guessing\n");
    printf("  \033[1;32m7.\033[0m Tic-Tac-Toe\n");
    printf("  \033[1;32m8.\033[0m Space Invaders\n");
    printf("  \033[1;31m9.\033[0m Exit to Shell\n");
    printf("\033[1;36m========================================\033[0m\n");
    printf("Select game [1-9]: ");
    fflush(stdout);
}

int main(void) {
    while (1) {
        show_menu();
        int choice = 0;
        if (scanf("%d", &choice) != 1) {
            while (getchar() != '\n');
            continue;
        }

        switch (choice) {
            case 1: system("snake"); break;
            case 2: system("tetris"); break;
            case 3: system("pong"); break;
            case 4: system("mines"); break;
            case 5: system("2048"); break;
            case 6: system("guess"); break;
            case 7: system("tictactoe"); break;
            case 8: system("spaceinvaders"); break;
            case 9:
                printf("\nExiting Micro Arcade. Welcome back to MicroOS.\n\n");
                return 0;
            default:
                printf("\nInvalid selection. Press Enter...");
                while (getchar() != '\n');
                getchar();
                break;
        }
    }
    return 0;
}
