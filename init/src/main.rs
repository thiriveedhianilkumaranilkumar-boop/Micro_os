//! MicroOS /init master entrypoint (Rust Edition v2.0)
use std::path::Path;
use std::process::Command;

fn main() {
    if Path::new("/bin/system-init").exists() {
        let _ = Command::new("/bin/system-init").status();
    } else if Path::new("/bin/micro-shell").exists() {
        let _ = Command::new("/bin/micro-shell").status();
    } else {
        let _ = Command::new("/bin/sh").status();
    }
}
