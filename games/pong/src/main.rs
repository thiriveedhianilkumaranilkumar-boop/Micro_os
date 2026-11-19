//! MicroOS Rust Edition v2.0 - Pong Arcade Game
//! Real-time paddle and ball mechanics with single-player and 2-player support.

use microos_common::{clear_screen, move_cursor, poll_key, Key, RawTerminal};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 16;
const PADDLE_HEIGHT: i32 = 4;

fn main() {
    let _raw = RawTerminal::enable();

    let mut p1_y = (HEIGHT as i32 - PADDLE_HEIGHT) / 2;
    let mut p2_y = (HEIGHT as i32 - PADDLE_HEIGHT) / 2;
    let mut ball_x = (WIDTH / 2) as i32;
    let mut ball_y = (HEIGHT / 2) as i32;
    let mut ball_dx = 1i32;
    let mut ball_dy = 1i32;
    let mut p1_score = 0u32;
    let mut p2_score = 0u32;

    clear_screen();

    loop {
        // 1. Process Input (W/S for Player 1, Up/Down for Player 2)
        match poll_key() {
            Key::Char('w') | Key::Char('W') if p1_y > 1 => p1_y -= 1,
            Key::Char('s') | Key::Char('S') if p1_y + PADDLE_HEIGHT < HEIGHT as i32 - 1 => p1_y += 1,
            Key::Up if p2_y > 1 => p2_y -= 1,
            Key::Down if p2_y + PADDLE_HEIGHT < HEIGHT as i32 - 1 => p2_y += 1,
            Key::Escape | Key::Char('q') | Key::Char('Q') => break,
            _ => {}
        }

        // Simple AI for Player 2 if no manual arrow keys
        if ball_dx > 0 {
            if ball_y > p2_y + 1 && p2_y + PADDLE_HEIGHT < HEIGHT as i32 - 1 {
                p2_y += 1;
            } else if ball_y < p2_y + 1 && p2_y > 1 {
                p2_y -= 1;
            }
        }

        // 2. Move Ball
        ball_x += ball_dx;
        ball_y += ball_dy;

        // Top/Bottom Bounce
        if ball_y <= 1 || ball_y >= HEIGHT as i32 - 2 {
            ball_dy = -ball_dy;
        }

        // Paddle 1 Collision
        if ball_x == 2 && ball_y >= p1_y && ball_y < p1_y + PADDLE_HEIGHT {
            ball_dx = 1;
        }
        // Paddle 2 Collision
        if ball_x == WIDTH as i32 - 3 && ball_y >= p2_y && ball_y < p2_y + PADDLE_HEIGHT {
            ball_dx = -1;
        }

        // Score Check
        if ball_x <= 0 {
            p2_score += 1;
            ball_x = (WIDTH / 2) as i32;
            ball_y = (HEIGHT / 2) as i32;
            ball_dx = 1;
        } else if ball_x >= WIDTH as i32 - 1 {
            p1_score += 1;
            ball_x = (WIDTH / 2) as i32;
            ball_y = (HEIGHT / 2) as i32;
            ball_dx = -1;
        }

        // 3. Render
        move_cursor(1, 1);
        print!("\x1b[1;36m");
        println!("┌{}┐  PONG (RUST EDITION)", "─".repeat(WIDTH));
        for y in 0..HEIGHT as i32 {
            print!("│");
            for x in 0..WIDTH as i32 {
                if x == 1 && y >= p1_y && y < p1_y + PADDLE_HEIGHT {
                    print!("\x1b[1;32m█\x1b[1;36m");
                } else if x == WIDTH as i32 - 2 && y >= p2_y && y < p2_y + PADDLE_HEIGHT {
                    print!("\x1b[1;33m█\x1b[1;36m");
                } else if x == ball_x && y == ball_y {
                    print!("\x1b[1;37mO\x1b[1;36m");
                } else if x == (WIDTH / 2) as i32 {
                    print!("┆");
                } else {
                    print!(" ");
                }
            }
            println!("│");
        }
        println!("└{}┘", "─".repeat(WIDTH));
        print!("\x1b[0m");

        println!("Player 1 (W/S): \x1b[1;32m{}\x1b[0m   |   Player 2 / AI (Up/Down): \x1b[1;33m{}\x1b[0m   (Q to Quit)", p1_score, p2_score);

        thread::sleep(Duration::from_millis(50));
    }
}
