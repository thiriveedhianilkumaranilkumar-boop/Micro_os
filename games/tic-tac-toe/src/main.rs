//! MicroOS Rust Edition v2.0 - Tic-Tac-Toe Arcade Game
//! 2-player mode and smart AI computer opponent with win/draw detection.

use std::io::{self, BufRead, Write};

fn print_board(b: &[char; 9]) {
    println!("\x1b[1;36m");
    println!("     1   2   3");
    println!("   ┌───┬───┬───┐");
    println!(" A │ {} │ {} │ {} │", b[0], b[1], b[2]);
    println!("   ├───┼───┼───┤");
    println!(" B │ {} │ {} │ {} │", b[3], b[4], b[5]);
    println!("   ├───┼───┼───┤");
    println!(" C │ {} │ {} │ {} │", b[6], b[7], b[8]);
    println!("   └───┴───┴───┘\x1b[0m");
}

fn check_winner(b: &[char; 9]) -> Option<char> {
    const WINS: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Cols
        [0, 4, 8], [2, 4, 6],             // Diags
    ];
    for &[x, y, z] in &WINS {
        if b[x] != ' ' && b[x] == b[y] && b[y] == b[z] {
            return Some(b[x]);
        }
    }
    None
}

fn is_full(b: &[char; 9]) -> bool {
    b.iter().all(|&c| c != ' ')
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("\x1b[2J\x1b[H\x1b[1;36m");
        println!("========================================");
        println!("             TIC-TAC-TOE                ");
        println!("          (Rust Edition v2.0)           ");
        println!("========================================");
        print!("\x1b[0m");

        println!("Game Mode:");
        println!("  1. Player vs AI (Computer)");
        println!("  2. Player 1 vs Player 2 (Local)");
        println!("  3. Exit");
        print!("Choose mode [1-3]: ");
        let _ = io::stdout().flush();

        let mut mode_line = String::new();
        if reader.read_line(&mut mode_line).unwrap_or(0) == 0 { break; }
        let vs_ai = match mode_line.trim() {
            "1" => true,
            "2" => false,
            _ => break,
        };

        let mut board = [' '; 9];
        let mut turn = 'X';

        loop {
            print!("\x1b[2J\x1b[H");
            println!("=== TIC-TAC-TOE ===\n");
            print_board(&board);

            if let Some(w) = check_winner(&board) {
                println!("\n\x1b[1;32m★ WINNER: Player {} wins! ★\x1b[0m", w);
                break;
            }
            if is_full(&board) {
                println!("\n\x1b[1;33m🤝 GAME DRAW! 🤝\x1b[0m");
                break;
            }

            if vs_ai && turn == 'O' {
                println!("AI thinking...");
                // AI pick: first available corner or center
                let pref = [4, 0, 2, 6, 8, 1, 3, 5, 7];
                for &idx in &pref {
                    if board[idx] == ' ' {
                        board[idx] = 'O';
                        break;
                    }
                }
                turn = 'X';
            } else {
                print!("Player {} move (e.g. A1, B2, C3 or q to quit): ", turn);
                let _ = io::stdout().flush();
                let mut mv = String::new();
                if reader.read_line(&mut mv).unwrap_or(0) == 0 { return; }
                let trimmed = mv.trim().to_uppercase();

                if trimmed == "Q" { return; }

                let idx = match trimmed.as_str() {
                    "A1" => 0, "A2" => 1, "A3" => 2,
                    "B1" => 3, "B2" => 4, "B3" => 5,
                    "C1" => 6, "C2" => 7, "C3" => 8,
                    _ => {
                        println!("Invalid format. Use A1-C3.");
                        continue;
                    }
                };

                if board[idx] == ' ' {
                    board[idx] = turn;
                    turn = if turn == 'X' { 'O' } else { 'X' };
                } else {
                    println!("Cell already occupied! Press Enter...");
                    let mut dummy = String::new();
                    let _ = reader.read_line(&mut dummy);
                }
            }
        }

        print!("\nPlay another round? [y/N]: ");
        let _ = io::stdout().flush();
        let mut again = String::new();
        if reader.read_line(&mut again).unwrap_or(0) == 0 { break; }
        if again.trim().to_lowercase() != "y" { break; }
    }
}
