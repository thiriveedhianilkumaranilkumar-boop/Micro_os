//! MicroOS Common Library (Rust Edition v2.0)
//! Provides ANSI terminal primitives, raw mode, non-blocking input, and persistence.

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

// --- ANSI Escape Code Helpers ---

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

pub fn move_cursor(row: usize, col: usize) {
    print!("\x1b[{};{}H", row, col);
    let _ = io::stdout().flush();
}

pub fn hide_cursor() {
    print!("\x1b[?25l");
    let _ = io::stdout().flush();
}

pub fn show_cursor() {
    print!("\x1b[?25h\x1b[0m");
    let _ = io::stdout().flush();
}

pub fn set_color_cyan() {
    print!("\x1b[1;36m");
}

pub fn set_color_yellow() {
    print!("\x1b[1;33m");
}

pub fn set_color_green() {
    print!("\x1b[1;32m");
}

pub fn set_color_red() {
    print!("\x1b[1;31m");
}

pub fn reset_color() {
    print!("\x1b[0m");
}

// --- Terminal Raw Mode via Direct Linux ioctls ---

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

impl Default for Termios {
    fn default() -> Self {
        Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        }
    }
}

const TCGETS: usize = 0x5401;
const TCSETS: usize = 0x5402;
const ECHO: u32 = 0x0008;
const ICANON: u32 = 0x0002;
const VMIN: usize = 6;
const VTIME: usize = 5;

pub struct RawTerminal {
    original: Termios,
}

impl RawTerminal {
    pub fn enable() -> Option<Self> {
        let mut orig = Termios::default();
        unsafe {
            // syscall: sys_ioctl(fd=0, cmd=TCGETS, arg=&mut orig)
            std::arch::asm!(
                "syscall",
                in("rax") 16usize,
                in("rdi") 0usize,
                in("rsi") TCGETS,
                in("rdx") &mut orig as *mut Termios,
                out("rcx") _,
                out("r11") _,
                lateout("rax") _,
            );
        };

        let mut raw = orig;
        raw.c_lflag &= !(ECHO | ICANON);
        raw.c_cc[VMIN] = 0;
        raw.c_cc[VTIME] = 0;

        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 16usize,
                in("rdi") 0usize,
                in("rsi") TCSETS,
                in("rdx") &raw as *const Termios,
                out("rcx") _,
                out("r11") _,
                lateout("rax") _,
            );
        }

        hide_cursor();
        Some(RawTerminal { original: orig })
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        unsafe {
            std::arch::asm!(
                "syscall",
                in("rax") 16usize,
                in("rdi") 0usize,
                in("rsi") TCSETS,
                in("rdx") &self.original as *const Termios,
                out("rcx") _,
                out("r11") _,
                lateout("rax") _,
            );
        }
        show_cursor();
    }
}

// --- Keyboard Input Helper ---

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
    None,
}

pub fn poll_key() -> Key {
    let mut buf = [0u8; 1];
    let mut stdin = io::stdin();
    if stdin.read(&mut buf).unwrap_or(0) == 1 {
        match buf[0] {
            b'\n' | b'\r' => Key::Enter,
            b'\x1b' => {
                let mut seq = [0u8; 2];
                if stdin.read(&mut seq[0..1]).unwrap_or(0) == 1 {
                    if seq[0] == b'[' {
                        if stdin.read(&mut seq[1..2]).unwrap_or(0) == 1 {
                            match seq[1] {
                                b'A' => return Key::Up,
                                b'B' => return Key::Down,
                                b'C' => return Key::Right,
                                b'D' => return Key::Left,
                                _ => return Key::Escape,
                            }
                        }
                    }
                }
                Key::Escape
            }
            c => Key::Char(c as char),
        }
    } else {
        Key::None
    }
}

// --- High Score Persistence ---

pub fn load_score(game_name: &str) -> u32 {
    let paths = [
        format!("/mnt/ssd/microos/scores_{}.txt", game_name),
        format!("/tmp/scores_{}.txt", game_name),
    ];
    for p in &paths {
        if let Ok(content) = fs::read_to_string(p) {
            if let Ok(score) = content.trim().parse::<u32>() {
                return score;
            }
        }
    }
    0
}

pub fn save_score(game_name: &str, score: u32) {
    let dir = "/mnt/ssd/microos";
    let target = if Path::new(dir).exists() {
        format!("{}/scores_{}.txt", dir, game_name)
    } else {
        format!("/tmp/scores_{}.txt", game_name)
    };
    let _ = fs::write(&target, score.to_string());
}
