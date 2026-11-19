//! MicroOS Rust Edition v2.0 - Tetris Arcade Game
//! Falling tetrominoes, rotation, line clearing, score, and persistence.

use microos_common::{clear_screen, load_score, move_cursor, poll_key, save_score, Key, RawTerminal};
use std::thread;
use std::time::Duration;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

const TETROMINOES: [[[i32; 2]; 4]; 7] = [
    [[0, 0], [1, 0], [0, 1], [1, 1]], // O
    [[0, 0], [-1, 0], [1, 0], [2, 0]], // I
    [[0, 0], [-1, 0], [1, 0], [0, 1]], // T
    [[0, 0], [-1, 0], [0, 1], [1, 1]], // Z
    [[0, 0], [1, 0], [0, 1], [-1, 1]], // S
    [[0, 0], [-1, 0], [1, 0], [-1, 1]], // L
    [[0, 0], [-1, 0], [1, 0], [1, 1]],  // J
];

fn main() {
    let _raw = RawTerminal::enable();
    let mut high_score = load_score("tetris");

    loop {
        let mut board = [[0u8; BOARD_WIDTH]; BOARD_HEIGHT];
        let mut score = 0u32;
        let mut lines_cleared = 0u32;
        let mut piece_idx = 0usize;
        let mut px = (BOARD_WIDTH / 2) as i32;
        let mut py = 0i32;
        let mut current_blocks = TETROMINOES[0];
        let mut game_over = false;

        clear_screen();

        while !game_over {
            // Check input
            match poll_key() {
                Key::Left | Key::Char('a') | Key::Char('A') => {
                    if can_place(&board, &current_blocks, px - 1, py) { px -= 1; }
                }
                Key::Right | Key::Char('d') | Key::Char('D') => {
                    if can_place(&board, &current_blocks, px + 1, py) { px += 1; }
                }
                Key::Up | Key::Char('w') | Key::Char('W') => {
                    // Rotate 90 deg clockwise
                    let rotated: [[i32; 2]; 4] = [
                        [-current_blocks[0][1], current_blocks[0][0]],
                        [-current_blocks[1][1], current_blocks[1][0]],
                        [-current_blocks[2][1], current_blocks[2][0]],
                        [-current_blocks[3][1], current_blocks[3][0]],
                    ];
                    if can_place(&board, &rotated, px, py) {
                        current_blocks = rotated;
                    }
                }
                Key::Down | Key::Char('s') | Key::Char('S') => {
                    if can_place(&board, &current_blocks, px, py + 1) { py += 1; }
                }
                Key::Escape | Key::Char('q') | Key::Char('Q') => return,
                _ => {}
            }

            // Fall
            if can_place(&board, &current_blocks, px, py + 1) {
                py += 1;
            } else {
                // Lock piece into board
                for b in &current_blocks {
                    let bx = px + b[0];
                    let by = py + b[1];
                    if by >= 0 && by < BOARD_HEIGHT as i32 && bx >= 0 && bx < BOARD_WIDTH as i32 {
                        board[by as usize][bx as usize] = 1;
                    }
                }

                // Check line clears
                let mut cleared = 0;
                for y in (0..BOARD_HEIGHT).rev() {
                    if board[y].iter().all(|&cell| cell == 1) {
                        cleared += 1;
                        for ny in (1..=y).rev() {
                            board[ny] = board[ny - 1];
                        }
                        board[0] = [0; BOARD_WIDTH];
                    }
                }
                if cleared > 0 {
                    lines_cleared += cleared;
                    score += cleared * 100 * cleared;
                }

                // Spawn new piece
                piece_idx = (piece_idx + 1) % 7;
                current_blocks = TETROMINOES[piece_idx];
                px = (BOARD_WIDTH / 2) as i32;
                py = 0;

                if !can_place(&board, &current_blocks, px, py) {
                    game_over = true;
                }
            }

            // Render
            move_cursor(1, 1);
            print!("\x1b[1;36m");
            println!("┌{}┐  TETRIS (RUST EDITION)", "──".repeat(BOARD_WIDTH));
            for y in 0..BOARD_HEIGHT {
                print!("│");
                for x in 0..BOARD_WIDTH {
                    let is_active = current_blocks.iter().any(|b| px + b[0] == x as i32 && py + b[1] == y as i32);
                    if is_active {
                        print!("\x1b[1;32m[]\x1b[1;36m");
                    } else if board[y][x] == 1 {
                        print!("\x1b[1;33m##\x1b[1;36m");
                    } else {
                        print!("  ");
                    }
                }
                println!("│");
            }
            println!("└{}┘", "──".repeat(BOARD_WIDTH));
            print!("\x1b[0m");

            println!("Score: \x1b[1;33m{}\x1b[0m  Lines: {}  High: \x1b[1;32m{}\x1b[0m", score, lines_cleared, high_score);
            println!("Controls: A/D/Arrows=Move, W/Up=Rotate, S/Down=Drop, Q=Quit");

            let delay = if score > 500 { 100 } else { 180 };
            thread::sleep(Duration::from_millis(delay));
        }

        if score > high_score {
            high_score = score;
            save_score("tetris", high_score);
        }

        move_cursor(BOARD_HEIGHT / 2, BOARD_WIDTH);
        print!("\x1b[1;31m=== GAME OVER ===\x1b[0m");
        move_cursor(BOARD_HEIGHT / 2 + 1, BOARD_WIDTH);
        print!("Final Score: {}", score);
        move_cursor(BOARD_HEIGHT / 2 + 2, BOARD_WIDTH);
        print!("Press [R] to Restart, [Q] to Exit");

        loop {
            match poll_key() {
                Key::Char('r') | Key::Char('R') => break,
                Key::Char('q') | Key::Char('Q') | Key::Escape => return,
                _ => thread::sleep(Duration::from_millis(50)),
            }
        }
    }
}

fn can_place(board: &[[u8; BOARD_WIDTH]; BOARD_HEIGHT], piece: &[[i32; 2]; 4], px: i32, py: i32) -> bool {
    for b in piece {
        let x = px + b[0];
        let y = py + b[1];
        if x < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
            return false;
        }
        if y >= 0 && board[y as usize][x as usize] == 1 {
            return false;
        }
    }
    true
}
