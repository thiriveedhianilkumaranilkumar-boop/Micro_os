//! MicroOS Rust Edition v2.0 - Micro Arcade Interactive Launcher
//! Unified terminal launcher for all 8 offline arcade games.

use std::io::{self, BufRead, Write};
use std::process::Command;

fn show_menu() {
    print!("\x1b[2J\x1b[H\x1b[1;36m");
    println!("========================================");
    println!("             MICRO ARCADE               ");
    println!("          (Rust Edition v2.0)           ");
    println!("========================================");
    print!("\x1b[0m");
    println!("  \x1b[1;32m1.\x1b[0m Snake");
    println!("  \x1b[1;32m2.\x1b[0m Tetris");
    println!("  \x1b[1;32m3.\x1b[0m Pong");
    println!("  \x1b[1;32m4.\x1b[0m Minesweeper");
    println!("  \x1b[1;32m5.\x1b[0m 2048");
    println!("  \x1b[1;32m6.\x1b[0m Number Guessing");
    println!("  \x1b[1;32m7.\x1b[0m Tic-Tac-Toe");
    println!("  \x1b[1;32m8.\x1b[0m Space Invaders");
    println!("  \x1b[1;31m9.\x1b[0m Exit to Shell");
    print!("\x1b[1;36m========================================\x1b[0m\n");
    print!("Select game [1-9]: ");
    let _ = io::stdout().flush();
}

fn run_game(name: &str) {
    let bin_path = format!("/bin/{}", name);
    if Command::new(&bin_path).status().is_err() {
        let _ = Command::new(name).status();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        show_menu();
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }

        match line.trim() {
            "1" => run_game("snake"),
            "2" => run_game("tetris"),
            "3" => run_game("pong"),
            "4" => run_game("mines"),
            "5" => run_game("2048"),
            "6" => run_game("guess"),
            "7" => run_game("tictactoe"),
            "8" => run_game("spaceinvaders"),
            "9" => {
                println!("\nExiting Micro Arcade. Welcome back to MicroOS.\n");
                break;
            }
            _ => {
                println!("\nInvalid selection. Press Enter...");
                let mut dummy = String::new();
                let _ = reader.read_line(&mut dummy);
            }
        }
    }
}
