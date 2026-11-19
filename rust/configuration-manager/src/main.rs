//! MicroOS Rust Edition v2.0 - Configuration Manager
//! Persists user settings, preferences, and arcade scores on /mnt/ssd.

use std::env;
use std::fs;
use std::path::Path;

const PERSIST_CFG: &str = "/mnt/ssd/microos/config.ini";

fn main() {
    let args: Vec<String> = env::args().collect();
    let action = if args.len() > 1 { args[1].as_str() } else { "show" };

    match action {
        "show" => {
            println!("-------------- PERSISTENT CONFIGURATION ------------");
            if Path::new(PERSIST_CFG).exists() {
                if let Ok(content) = fs::read_to_string(PERSIST_CFG) {
                    println!("{}", content);
                    return;
                }
            }
            println!("No persistent configuration found. (Run in persistent mode with /mnt/ssd)");
        }
        "set" if args.len() > 3 => {
            let key = &args[2];
            let val = &args[3];
            let _ = fs::create_dir_all("/mnt/ssd/microos");
            let line = format!("{}={}\n", key, val);
            let _ = fs::write(PERSIST_CFG, line);
            println!("Configuration updated: {} = {}", key, val);
        }
        _ => {
            println!("Usage: configmgr [show | set <key> <val>]");
        }
    }
}
