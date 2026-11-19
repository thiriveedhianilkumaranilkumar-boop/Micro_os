//! MicroOS Rust Edition v2.0 - Snake Arcade Game
//! Real-time movement, collision, score, and persistence.

use microos_common::{clear_screen, load_score, move_cursor, poll_key, save_score, Key, RawTerminal};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 40;
const HEIGHT: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let _raw = RawTerminal::enable();
    let mut high_score = load_score("snake");

    loop {
        let mut snake = vec![
            Point { x: 10, y: 10 },
            Point { x: 9, y: 10 },
            Point { x: 8, y: 10 },
        ];
        let mut dir = Point { x: 1, y: 0 };
        let mut food = Point { x: 20, y: 10 };
        let mut score = 0u32;
        let mut game_over = false;

        clear_screen();

        while !game_over {
            // 1. Process Input
            match poll_key() {
                Key::Up | Key::Char('w') | Key::Char('W') if dir.y == 0 => dir = Point { x: 0, y: -1 },
                Key::Down | Key::Char('s') | Key::Char('S') if dir.y == 0 => dir = Point { x: 0, y: 1 },
                Key::Left | Key::Char('a') | Key::Char('A') if dir.x == 0 => dir = Point { x: -1, y: 0 },
                Key::Right | Key::Char('d') | Key::Char('D') if dir.x == 0 => dir = Point { x: 1, y: 0 },
                Key::Escape | Key::Char('q') | Key::Char('Q') => return,
                _ => {}
            }

            // 2. Move Snake
            let head = snake[0];
            let new_head = Point {
                x: head.x + dir.x,
                y: head.y + dir.y,
            };

            // Wall Collision
            if new_head.x <= 0 || new_head.x >= WIDTH as i32 || new_head.y <= 0 || new_head.y >= HEIGHT as i32 {
                game_over = true;
                break;
            }

            // Self Collision
            if snake.contains(&new_head) {
                game_over = true;
                break;
            }

            snake.insert(0, new_head);

            // Food Collision
            if new_head == food {
                score += 10;
                food = Point {
                    x: 2 + ((food.x * 7 + 13) % (WIDTH as i32 - 4)),
                    y: 2 + ((food.y * 11 + 7) % (HEIGHT as i32 - 4)),
                };
            } else {
                snake.pop();
            }

            // 3. Render Frame
            move_cursor(1, 1);
            print!("\x1b[1;36m");
            println!("┌{}┐", "─".repeat(WIDTH));
            for y in 1..HEIGHT as i32 {
                print!("│");
                for x in 1..WIDTH as i32 {
                    let pt = Point { x, y };
                    if pt == new_head {
                        print!("\x1b[1;32mO\x1b[1;36m");
                    } else if snake.contains(&pt) {
                        print!("\x1b[1;32mo\x1b[1;36m");
                    } else if pt == food {
                        print!("\x1b[1;31m*\x1b[1;36m");
                    } else {
                        print!(" ");
                    }
                }
                println!("│");
            }
            println!("└{}┘", "─".repeat(WIDTH));
            print!("\x1b[0m");

            println!("Score: \x1b[1;33m{}\x1b[0m  High: \x1b[1;32m{}\x1b[0m   (WASD/Arrows to Move, Q to Quit)", score, high_score);

            // Speed based on score
            let delay = if score > 100 { 80 } else { 120 };
            thread::sleep(Duration::from_millis(delay));
        }

        if score > high_score {
            high_score = score;
            save_score("snake", high_score);
        }

        // Game Over Screen
        move_cursor(HEIGHT / 2, WIDTH / 4);
        print!("\x1b[1;31m=== GAME OVER ===\x1b[0m");
        move_cursor(HEIGHT / 2 + 1, WIDTH / 4);
        print!("Final Score: {} (High: {})", score, high_score);
        move_cursor(HEIGHT / 2 + 2, WIDTH / 4);
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
