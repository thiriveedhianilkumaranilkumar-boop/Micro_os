//! MicroOS Rust Edition v2.0 - Space Invaders Arcade Game
//! Real-time ASCII space defense, laser projectiles, alien fleets, score, persistence.

use microos_common::{clear_screen, load_score, move_cursor, poll_key, save_score, Key, RawTerminal};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 40;
const HEIGHT: usize = 18;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let _raw = RawTerminal::enable();
    let mut high_score = load_score("spaceinvaders");

    loop {
        let mut player_x = (WIDTH / 2) as i32;
        let mut bullets: Vec<Point> = Vec::new();
        let mut aliens: Vec<Point> = Vec::new();
        let mut score = 0u32;
        let mut game_over = false;
        let mut alien_dir = 1i32;
        let mut tick = 0u32;

        // Spawn alien fleet
        for r in 1..=3 {
            for c in 0..8 {
                aliens.push(Point { x: 5 + c * 4, y: r * 2 });
            }
        }

        clear_screen();

        while !game_over {
            tick += 1;

            // 1. Process Input
            match poll_key() {
                Key::Left | Key::Char('a') | Key::Char('A') if player_x > 1 => player_x -= 1,
                Key::Right | Key::Char('d') | Key::Char('D') if player_x < WIDTH as i32 - 2 => player_x += 1,
                Key::Enter | Key::Char(' ') => {
                    bullets.push(Point { x: player_x, y: HEIGHT as i32 - 2 });
                }
                Key::Escape | Key::Char('q') | Key::Char('Q') => return,
                _ => {}
            }

            // 2. Update Bullets
            bullets.retain_mut(|b| {
                b.y -= 1;
                b.y > 0
            });

            // 3. Update Aliens every 5 ticks
            if tick % 5 == 0 {
                let mut reverse = false;
                for a in &aliens {
                    if (alien_dir > 0 && a.x >= WIDTH as i32 - 3) || (alien_dir < 0 && a.x <= 2) {
                        reverse = true;
                        break;
                    }
                }
                if reverse {
                    alien_dir = -alien_dir;
                    for a in &mut aliens {
                        a.y += 1;
                        if a.y >= HEIGHT as i32 - 2 {
                            game_over = true;
                        }
                    }
                } else {
                    for a in &mut aliens {
                        a.x += alien_dir;
                    }
                }
            }

            // 4. Collision Detection
            bullets.retain(|b| {
                if let Some(pos) = aliens.iter().position(|a| a.x == b.x && a.y == b.y) {
                    aliens.remove(pos);
                    score += 25;
                    false // Bullet consumed
                } else {
                    true
                }
            });

            if aliens.is_empty() {
                // Next wave
                for r in 1..=3 {
                    for c in 0..8 {
                        aliens.push(Point { x: 5 + c * 4, y: r * 2 });
                    }
                }
            }

            // 5. Render
            move_cursor(1, 1);
            print!("\x1b[1;36m");
            println!("┌{}┐  SPACE INVADERS (RUST)", "─".repeat(WIDTH));
            for y in 1..HEIGHT as i32 {
                print!("│");
                for x in 1..WIDTH as i32 {
                    let pt = Point { x, y };
                    if x == player_x && y == HEIGHT as i32 - 1 {
                        print!("\x1b[1;32m▲\x1b[1;36m");
                    } else if aliens.contains(&pt) {
                        print!("\x1b[1;31m👾\x1b[1;36m");
                    } else if bullets.contains(&pt) {
                        print!("\x1b[1;33m|\x1b[1;36m");
                    } else {
                        print!(" ");
                    }
                }
                println!("│");
            }
            println!("└{}┘", "─".repeat(WIDTH));
            print!("\x1b[0m");

            println!("Score: \x1b[1;33m{}\x1b[0m  High: \x1b[1;32m{}\x1b[0m   (A/D=Move, Space=Fire, Q=Quit)", score, high_score);

            thread::sleep(Duration::from_millis(60));
        }

        if score > high_score {
            high_score = score;
            save_score("spaceinvaders", high_score);
        }

        move_cursor(HEIGHT / 2, WIDTH / 4);
        print!("\x1b[1;31m=== DEFENSE BREACHED! GAME OVER ===\x1b[0m");
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
