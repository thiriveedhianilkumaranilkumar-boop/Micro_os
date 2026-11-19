//! MicroOS Rust Edition v2.0 - 2048 Arcade Game
//! 4-way tile sliding, exponential merging, score calculation, and persistence.

use microos_common::{clear_screen, load_score, move_cursor, poll_key, save_score, Key, RawTerminal};
use std::thread;
use std::time::Duration;

fn main() {
    let _raw = RawTerminal::enable();
    let mut high_score = load_score("2048");

    loop {
        let mut board = [[0u32; 4]; 4];
        let mut score = 0u32;
        let mut seed = 12345u64;

        spawn_tile(&mut board, &mut seed);
        spawn_tile(&mut board, &mut seed);

        clear_screen();

        loop {
            // Render
            move_cursor(1, 1);
            print!("\x1b[1;36m");
            println!("┌────────────────────────┐  2048 (RUST EDITION)");
            for row in &board {
                print!("│");
                for &val in row {
                    if val == 0 {
                        print!("  .   ");
                    } else if val < 100 {
                        print!("\x1b[1;32m {:4} \x1b[1;36m", val);
                    } else if val < 1000 {
                        print!("\x1b[1;33m {:4} \x1b[1;36m", val);
                    } else {
                        print!("\x1b[1;31m {:4} \x1b[1;36m", val);
                    }
                }
                println!("│");
            }
            println!("└────────────────────────┘");
            print!("\x1b[0m");

            println!("Score: \x1b[1;33m{}\x1b[0m  High: \x1b[1;32m{}\x1b[0m", score, high_score);
            println!("Controls: WASD/Arrows to Slide, Q to Quit");

            // Process Input
            let key = poll_key();
            let mut moved = false;

            match key {
                Key::Up | Key::Char('w') | Key::Char('W') => {
                    moved = move_up(&mut board, &mut score);
                }
                Key::Down | Key::Char('s') | Key::Char('S') => {
                    moved = move_down(&mut board, &mut score);
                }
                Key::Left | Key::Char('a') | Key::Char('A') => {
                    moved = move_left(&mut board, &mut score);
                }
                Key::Right | Key::Char('d') | Key::Char('D') => {
                    moved = move_right(&mut board, &mut score);
                }
                Key::Escape | Key::Char('q') | Key::Char('Q') => return,
                _ => {
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }
            }

            if moved {
                spawn_tile(&mut board, &mut seed);
                if score > high_score {
                    high_score = score;
                    save_score("2048", high_score);
                }
            }

            // Check if game over
            if is_game_over(&board) {
                println!("\n\x1b[1;31m=== GAME OVER ===\x1b[0m");
                println!("Press [R] to Restart, [Q] to Exit");
                loop {
                    match poll_key() {
                        Key::Char('r') | Key::Char('R') => break,
                        Key::Char('q') | Key::Char('Q') | Key::Escape => return,
                        _ => thread::sleep(Duration::from_millis(50)),
                    }
                }
                break;
            }
        }
    }
}

fn spawn_tile(board: &mut [[u32; 4]; 4], seed: &mut u64) {
    let mut empty = Vec::new();
    for r in 0..4 {
        for c in 0..4 {
            if board[r][c] == 0 {
                empty.push((r, c));
            }
        }
    }
    if !empty.is_empty() {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let idx = (*seed as usize >> 16) % empty.len();
        let (r, c) = empty[idx];
        board[r][c] = if (*seed % 10) == 0 { 4 } else { 2 };
    }
}

fn slide_and_merge_row(row: &mut [u32; 4], score: &mut u32) -> bool {
    let mut non_zero: Vec<u32> = row.iter().copied().filter(|&x| x != 0).collect();
    let mut merged = Vec::new();
    let mut i = 0;
    while i < non_zero.len() {
        if i + 1 < non_zero.len() && non_zero[i] == non_zero[i + 1] {
            let val = non_zero[i] * 2;
            merged.push(val);
            *score += val;
            i += 2;
        } else {
            merged.push(non_zero[i]);
            i += 1;
        }
    }
    while merged.len() < 4 {
        merged.push(0);
    }
    let changed = *row != merged[..];
    row.copy_from_slice(&merged[..]);
    changed
}

fn move_left(board: &mut [[u32; 4]; 4], score: &mut u32) -> bool {
    let mut moved = false;
    for row in board.iter_mut() {
        if slide_and_merge_row(row, score) { moved = true; }
    }
    moved
}

fn move_right(board: &mut [[u32; 4]; 4], score: &mut u32) -> bool {
    let mut moved = false;
    for row in board.iter_mut() {
        row.reverse();
        if slide_and_merge_row(row, score) { moved = true; }
        row.reverse();
    }
    moved
}

fn move_up(board: &mut [[u32; 4]; 4], score: &mut u32) -> bool {
    let mut moved = false;
    for c in 0..4 {
        let mut col = [board[0][c], board[1][c], board[2][c], board[3][c]];
        if slide_and_merge_row(&mut col, score) {
            moved = true;
            for r in 0..4 { board[r][c] = col[r]; }
        }
    }
    moved
}

fn move_down(board: &mut [[u32; 4]; 4], score: &mut u32) -> bool {
    let mut moved = false;
    for c in 0..4 {
        let mut col = [board[3][c], board[2][c], board[1][c], board[0][c]];
        if slide_and_merge_row(&mut col, score) {
            moved = true;
            for r in 0..4 { board[3 - r][c] = col[r]; }
        }
    }
    moved
}

fn is_game_over(board: &[[u32; 4]; 4]) -> bool {
    for r in 0..4 {
        for c in 0..4 {
            if board[r][c] == 0 { return false; }
            if c + 1 < 4 && board[r][c] == board[r][c + 1] { return false; }
            if r + 1 < 4 && board[r][c] == board[r + 1][c] { return false; }
        }
    }
    true
}
