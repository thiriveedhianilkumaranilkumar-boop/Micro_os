#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(void) {
    srand(time(NULL));

    printf("\033[2J\033[H");
    printf("\033[1;36m========================================\033[0m\n");
    printf("\033[1;36m=== MICROOS ARCADE: NUMBER GUESSING  ===\033[0m\n");
    printf("\033[1;36m========================================\033[0m\n\n");

    printf("Select Difficulty:\n");
    printf("  1. Easy   (Range: 1 to 50,  10 attempts max)\n");
    printf("  2. Medium (Range: 1 to 100,  7 attempts max)\n");
    printf("  3. Hard   (Range: 1 to 500,  8 attempts max)\n");
    printf("Choice [1-3, or 0 to Exit]: ");
    fflush(stdout);

    int diff = 0;
    if (scanf("%d", &diff) != 1 || diff <= 0 || diff > 3) {
        printf("Exiting to shell.\n");
        return 0;
    }

    int max_range = 100;
    int max_attempts = 7;
    if (diff == 1) { max_range = 50; max_attempts = 10; }
    else if (diff == 3) { max_range = 500; max_attempts = 8; }

    int secret = 1 + rand() % max_range;
    int attempts = 0;
    int guessed = 0;

    printf("\nI'm thinking of a number between 1 and %d. Can you guess it?\n", max_range);

    while (attempts < max_attempts) {
        attempts++;
        printf("\n[Attempt %d/%d] Enter your guess: ", attempts, max_attempts);
        fflush(stdout);

        int guess;
        if (scanf("%d", &guess) != 1) {
            printf("Invalid input. Game aborted.\n");
            return 0;
        }

        if (guess == secret) {
            printf("\033[1;32m\n*** BINGO! You guessed the secret number (%d) in %d attempts! ***\033[0m\n", secret, attempts);
            guessed = 1;
            break;
        } else if (guess < secret) {
            printf("\033[1;33m--> HIGHER! Try a larger number.\033[0m\n");
        } else {
            printf("\033[1;34m--> LOWER! Try a smaller number.\033[0m\n");
        }
    }

    if (!guessed) {
        printf("\033[1;31m\n*** OUT OF ATTEMPTS! The secret number was %d. Better luck next time! ***\033[0m\n", secret);
    }

    printf("\nPress Enter to return to shell...");
    fflush(stdout);
    while (getchar() != '\n');
    getchar();
    return 0;
}
