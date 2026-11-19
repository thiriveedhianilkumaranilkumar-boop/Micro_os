//! MicroOS Rust Edition v2.0 - Interactive Micro-Shell
//! Provides custom prompt, command parsing, built-in execution,
//! hardware, networking, storage, package management, and arcade games.

use std::env;
use std::io::{self, BufRead, Write};
use std::process::Command;

fn print_welcome() {
    print!("\x1b[1;36m");
    println!("====================================================");
    println!(" MICROOS RUST EDITION v2.0");
    println!(" Welcome to MicroOS.");
    println!(" Minimal Linux. Maximum control.");
    println!("====================================================");
    print!("\x1b[0m");
    println!("Type 'help' for commands, 'sysinfo' for overview, or 'arcade' for games.\n");
}

fn show_help() {
    println!("====================================================");
    println!(" MICROOS COMMAND-LINE SYSTEM REFERENCE (RUST EDITION)");
    println!("====================================================");
    println!();
    println!("System Operations:");
    println!("  help        Display this command summary");
    println!("  clear       Clear the terminal console screen");
    println!("  sysinfo     Display system specs, uptime, RAM, and storage");
    println!("  reboot      Safely unmount filesystems and reboot system");
    println!("  poweroff    Safely shut down the system");
    println!("  exit        Exit current shell session");
    println!();
    println!("Hardware Discovery:");
    println!("  hardware [subcommand]  (cpu, memory, network, storage, usb, pci, all)");
    println!();
    println!("Networking & Wi-Fi:");
    println!("  wifi [status|scan|connect|disconnect|dhcp]");
    println!();
    println!("Storage Management:");
    println!("  disks [list|info|mount|unmount|mounts]");
    println!("  mount <device> [target]    Mount filesystem");
    println!("  unmount <target>           Safely unmount filesystem");
    println!();
    println!("Package Management (Persistent Userspace):");
    println!("  apt [update|search|install|remove|status|init]");
    println!();
    println!("Micro Arcade:");
    println!("  arcade                     Launch interactive Arcade menu");
    println!("  Direct game commands:      snake, tetris, pong, mines,");
    println!("                             2048, guess, tictactoe, spaceinvaders");
    println!("====================================================");
}

fn execute_command(line: &str) -> bool {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return true;
    }

    let cmd = tokens[0];
    let args = &tokens[1..];

    match cmd {
        "help" => show_help(),
        "clear" => {
            print!("\x1b[2J\x1b[H");
            let _ = io::stdout().flush();
        }
        "sysinfo" => {
            if Command::new("/bin/sysinfo").status().is_err() {
                let _ = Command::new("sysinfo").status();
            }
        }
        "hardware" => {
            let mut c = Command::new("/bin/hardware");
            c.args(args);
            if c.status().is_err() {
                let _ = Command::new("hardware").args(args).status();
            }
        }
        "wifi" => {
            let mut c = Command::new("/bin/wifi");
            c.args(args);
            if c.status().is_err() {
                let _ = Command::new("wifi").args(args).status();
            }
        }
        "disks" => {
            let mut c = Command::new("/bin/disks");
            c.args(args);
            if c.status().is_err() {
                let _ = Command::new("disks").args(args).status();
            }
        }
        "mount" => {
            let mut c = Command::new("/bin/mount");
            c.args(args);
            let _ = c.status();
        }
        "unmount" | "umount" => {
            let mut c = Command::new("/bin/umount");
            c.args(args);
            let _ = c.status();
        }
        "apt" => {
            let mut c = Command::new("/bin/apt");
            c.args(args);
            if c.status().is_err() {
                let _ = Command::new("apt").args(args).status();
            }
        }
        "arcade" => {
            if Command::new("/bin/arcade").status().is_err() {
                let _ = Command::new("arcade").status();
            }
        }
        "snake" | "tetris" | "pong" | "mines" | "2048" | "guess" | "tictactoe" | "spaceinvaders" => {
            let bin_path = format!("/bin/{}", cmd);
            if Command::new(&bin_path).status().is_err() {
                let _ = Command::new(cmd).status();
            }
        }
        "reboot" => {
            println!("Synchronizing filesystems and rebooting...");
            let _ = Command::new("/bin/sync").status();
            let _ = Command::new("/sbin/reboot").status();
        }
        "poweroff" | "halt" => {
            println!("Synchronizing filesystems and powering off...");
            let _ = Command::new("/bin/sync").status();
            let _ = Command::new("/sbin/poweroff").status();
        }
        "cd" => {
            let target = if args.is_empty() {
                "/root"
            } else {
                args[0]
            };
            if let Err(e) = env::set_current_dir(target) {
                eprintln!("cd: {}: {}", target, e);
            }
        }
        "exit" => return false,
        _ => {
            // Fallback to executing standard system binary
            let mut c = Command::new(cmd);
            c.args(args);
            match c.status() {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("microos: {}: command not found. Type 'help' for available commands.", cmd);
                }
            }
        }
    }

    true
}

fn main() {
    print_welcome();

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        let cwd = env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "/".to_string());

        let display_dir = if cwd == "/root" {
            "~".to_string()
        } else {
            cwd
        };

        // Print recognizable micro@micro:~$ prompt
        print!("\x1b[1;36mmicro@micro\x1b[0m:\x1b[1;34m{}\x1b[0m$ ", display_dir);
        let _ = io::stdout().flush();

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    if !execute_command(trimmed) {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }

    println!("\nSession terminated.");
}
