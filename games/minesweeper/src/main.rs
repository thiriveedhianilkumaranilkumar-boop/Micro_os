//! MicroOS Rust Edition v2.0 - Minesweeper Arcade Game
//! Real-time cursor navigation, tile reveal, flagging, recursive expansion, win/loss.

use microos_common::{clear_screen, move_cursor, poll_key, Key, RawTerminal};
use std::thread;
use std::time::Duration;

const GRID_SIZE: usize = 9;
const NUM_MINES: usize = 10;

#[derive(Clone, Copy, Default)]
struct Cell {
    is_mine: bool,
    revealed: bool,
    flagged: bool,
    adj_mines: u8,
}

fn main() {
    let _raw = RawTerminal::enable();

    loop {
        let mut grid = [[Cell::default(); GRID_SIZE]; GRID_SIZE];
        let mut cx = 0usize;
        let mut cy = 0usize;
        let mut game_over = false;
        let mut victory = false;

        // Place 10 mines deterministically with pseudo-random seed
        let mut placed = 0;
        let mut seed = 42u64;
        while placed < NUM_MINES {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let rx = (seed as usize >> 16) % GRID_SIZE;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let ry = (seed as usize >> 16) % GRID_SIZE;

            if !grid[ry][rx].is_mine {
                grid[ry][rx].is_mine = true;
                placed += 1;
            }
        }

        // Calculate adjacent mine counts
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if !grid[y][x].is_mine {
                    let mut count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let ny = y as i32 + dy;
                            let nx = x as i32 + dx;
                            if ny >= 0 && ny < GRID_SIZE as i32 && nx >= 0 && nx < GRID_SIZE as i32 {
                                if grid[ny as usize][nx as usize].is_mine {
                                    count += 1;
                                }
                            }
                        }
                    }
                    grid[y][x].adj_mines = count;
                }
            }
        }

        clear_screen();

        while !game_over && !victory {
            // Render
            move_cursor(1, 1);
            print!("\x1b[1;36m");
            println!("┌{}┐  MINESWEEPER (RUST)", "───".repeat(GRID_SIZE));
            for y in 0..GRID_SIZE {
                print!("│");
                for x in 0..GRID_SIZE {
                    let is_cursor = x == cx && y == cy;
                    let cell = &grid[y][x];

                    let sym = if cell.revealed {
                        if cell.is_mine {
                            "\x1b[1;31m * \x1b[0m"
                        } else if cell.adj_mines > 0 {
                            match cell.adj_mines {
                                1 => "\x1b[1;34m 1 \x1b[0m",
                                2 => "\x1b[1;32m 2 \x1b[0m",
                                3 => "\x1b[1;31m 3 \x1b[0m",
                                _ => "\x1b[1;35m 4 \x1b[0m",
                            }
                        } else {
                            "   "
                        }
                    } else if cell.flagged {
                        "\x1b[1;33m P \x1b[0m"
                    } else {
                        "\x1b[1;30m . \x1b[0m"
                    };

                    if is_cursor {
                        print!("\x1b[7m{}\x1b[0m", sym);
                    } else {
                        print!("{}", sym);
                    }
                }
                println!("\x1b[1;36m│\x1b[0m");
            }
            println!("\x1b[1;36m└{}┘\x1b[0m", "───".repeat(GRID_SIZE));
            println!("Controls: WASD/Arrows=Move, [Space/Enter]=Reveal, [F]=Flag, [Q]=Quit");

            // Process Input
            match poll_key() {
                Key::Up | Key::Char('w') | Key::Char('W') if cy > 0 => cy -= 1,
                Key::Down | Key::Char('s') | Key::Char('S') if cy < GRID_SIZE - 1 => cy += 1,
                Key::Left | Key::Char('a') | Key::Char('A') if cx > 0 => cx -= 1,
                Key::Right | Key::Char('d') | Key::Char('D') if cx < GRID_SIZE - 1 => cx += 1,
                Key::Char('f') | Key::Char('F') => {
                    if !grid[cy][cx].revealed {
                        grid[cy][cx].flagged = !grid[cy][cx].flagged;
                    }
                }
                Key::Enter | Key::Char(' ') => {
                    if !grid[cy][cx].flagged {
                        reveal(&mut grid, cx, cy);
                        if grid[cy][cx].is_mine {
                            game_over = true;
                        }
                    }
                }
                Key::Escape | Key::Char('q') | Key::Char('Q') => return,
                _ => {}
            }

            // Check victory condition
            let mut unrevealed_safe = 0;
            for row in &grid {
                for c in row {
                    if !c.is_mine && !c.revealed {
                        unrevealed_safe += 1;
                    }
                }
            }
            if unrevealed_safe == 0 {
                victory = true;
            }

            thread::sleep(Duration::from_millis(50));
        }

        // Reveal all mines on game over
        for row in &mut grid {
            for c in row {
                if c.is_mine { c.revealed = true; }
            }
        }

        // Final message
        move_cursor(GRID_SIZE + 4, 1);
        if victory {
            println!("\x1b[1;32m=== VICTORY! ALL MINES CLEARED! ===\x1b[0m");
        } else {
            println!("\x1b[1;31m=== BOOM! GAME OVER ===\x1b[0m");
        }
        println!("Press [R] to Restart, [Q] to Exit to Shell");

        loop {
            match poll_key() {
                Key::Char('r') | Key::Char('R') => break,
                Key::Char('q') | Key::Char('Q') | Key::Escape => return,
                _ => thread::sleep(Duration::from_millis(50)),
            }
        }
    }
}

fn reveal(grid: &mut [[Cell; GRID_SIZE]; GRID_SIZE], x: usize, y: usize) {
    if grid[y][x].revealed || grid[y][x].flagged {
        return;
    }
    grid[y][x].revealed = true;

    if grid[y][x].adj_mines == 0 && !grid[y][x].is_mine {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if ny >= 0 && ny < GRID_SIZE as i32 && nx >= 0 && nx < GRID_SIZE as i32 {
                    reveal(grid, nx as usize, ny as usize);
                }
            }
        }
    }
}
