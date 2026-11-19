//! MicroOS Rust Edition v2.0 - Number Guessing Arcade Game
//! Random number generation, higher/lower hints, attempt counter, difficulty tiers.

use std::io::{self, BufRead, Write};
use std::time::SystemTime;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("\x1b[2J\x1b[H\x1b[1;36m");
        println!("========================================");
        println!("       NUMBER GUESSING CHALLENGE        ");
        println!("          (Rust Edition v2.0)           ");
        println!("========================================");
        print!("\x1b[0m");

        println!("Select Difficulty:");
        println!("  1. Easy   (Range: 1 - 50)");
        println!("  2. Medium (Range: 1 - 100)");
        println!("  3. Hard   (Range: 1 - 500)");
        println!("  4. Exit to Shell");
        print!("Choose tier [1-4]: ");
        let _ = io::stdout().flush();

        let mut diff_line = String::new();
        if reader.read_line(&mut diff_line).unwrap_or(0) == 0 { break; }

        let max_val = match diff_line.trim() {
            "1" => 50u32,
            "2" => 100u32,
            "3" => 500u32,
            "4" | "q" | "Q" => break,
            _ => 100u32,
        };

        // Pseudo-random generation based on system timestamp
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let secret = ((now % max_val as u128) + 1) as u32;
        let mut attempts = 0u32;

        println!("\nI have chosen a secret number between 1 and {}.", max_val);
        println!("Enter your guesses below (or type 'q' to quit):");

        loop {
            print!("Guess #{}: ", attempts + 1);
            let _ = io::stdout().flush();

            let mut guess_line = String::new();
            if reader.read_line(&mut guess_line).unwrap_or(0) == 0 { return; }
            let trimmed = guess_line.trim();

            if trimmed == "q" || trimmed == "Q" { break; }

            if let Ok(guess) = trimmed.parse::<u32>() {
                attempts += 1;
                if guess < secret {
                    println!("\x1b[1;33m  ▲ HIGHER! (Your guess {} is too low)\x1b[0m", guess);
                } else if guess > secret {
                    println!("\x1b[1;34m  ▼ LOWER! (Your guess {} is too high)\x1b[0m", guess);
                } else {
                    print!("\x1b[1;32m");
                    println!("========================================");
                    println!(" ★ BINGO! You guessed {} correctly! ★", secret);
                    println!(" Total Attempts: {}", attempts);
                    println!("========================================");
                    print!("\x1b[0m");
                    break;
                }
            } else {
                println!("Please enter a valid integer.");
            }
        }

        print!("\nPlay again? [y/N]: ");
        let _ = io::stdout().flush();
        let mut again = String::new();
        if reader.read_line(&mut again).unwrap_or(0) == 0 { break; }
        if again.trim().to_lowercase() != "y" { break; }
    }
}
