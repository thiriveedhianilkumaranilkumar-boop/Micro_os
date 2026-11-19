//! MicroOS Rust Edition v2.0 - System Overview Utility
//! High-contrast system overview and hardware banner.

use std::fs;
use std::process::Command;

fn main() {
    print!("\x1b[1;36m");
    println!(" __  __ _             ___  ____  ");
    println!("|  \\/  (_) ___ _ __ / _ \\/ ___| ");
    println!("| |\\/| | |/ __| '__| | | \\___ \\ ");
    println!("| |  | | | (__| |  | |_| |___) |");
    println!("|_|  |_|_|\\___|_|   \\___/|____/ ");
    print!("\x1b[0m");
    println!("\x1b[1;37m  Minimal Linux. Maximum control. (Rust Edition v2.0)\x1b[0m");
    println!("----------------------------------------------------");

    // Kernel & Arch
    let kernel = fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| "Linux 6.6.134-0-lts".to_string())
        .split_whitespace()
        .nth(2)
        .unwrap_or("6.6.134-0-lts")
        .to_string();
    println!("  \x1b[1;34mOS\x1b[0m          : MicroOS Rust Edition v2.0 (x86_64)");
    println!("  \x1b[1;34mKernel\x1b[0m      : Linux {}", kernel);

    // Uptime
    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(first) = uptime_str.split_whitespace().next() {
            if let Ok(secs_f) = first.parse::<f64>() {
                let secs = secs_f as u64;
                let mins = secs / 60;
                let hours = mins / 60;
                let days = hours / 24;
                println!("  \x1b[1;34mUptime\x1b[0m      : {}d {}h {}m", days, hours % 24, mins % 60);
            }
        }
    }

    // CPU
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        let mut model = "x86_64 Processor".to_string();
        let mut cores = 0;
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                if let Some(val) = line.split(':').nth(1) {
                    model = val.trim().to_string();
                }
            } else if line.starts_with("processor") {
                cores += 1;
            }
        }
        println!("  \x1b[1;34mCPU\x1b[0m         : {} ({} vCPUs)", model, cores);
    }

    // Memory
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        let mut total_kb = 0u64;
        let mut free_kb = 0u64;
        for line in meminfo.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let val = parts[1].parse::<u64>().unwrap_or(0);
                if line.starts_with("MemTotal:") { total_kb = val; }
                else if line.starts_with("MemAvailable:") { free_kb = val; }
            }
        }
        println!("  \x1b[1;34mMemory\x1b[0m      : {:.1} MB free / {:.1} MB total", free_kb as f64 / 1024.0, total_kb as f64 / 1024.0);
    }

    // Persistence
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        if mounts.contains("/mnt/ssd") {
            println!("  \x1b[1;32mSSD Storage\x1b[0m : Mounted on /mnt/ssd (Persistent Mode)");
        } else {
            println!("  \x1b[1;33mSSD Storage\x1b[0m : Not mounted (Run 'disks mount <dev> /mnt/ssd')");
        }
    }

    // Network IP
    let mut ip = "[Offline]".to_string();
    if let Ok(output) = Command::new("/bin/ip").args(["-4", "addr", "show"]).output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("inet ") && !line.contains("127.0.0.1") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    ip = val.to_string();
                    break;
                }
            }
        }
    }
    println!("  \x1b[1;34mNetwork IP\x1b[0m  : {}", ip);

    println!("----------------------------------------------------");
    println!("Type \x1b[1;32mhelp\x1b[0m for command reference or \x1b[1;33marcade\x1b[0m for games.");
}
