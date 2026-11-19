//! MicroOS Rust Edition v2.0 - Storage & SSD Manager
//! Non-destructive block device discovery, partition inspection, and safe mounting.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn show_help() {
    println!("====================================================");
    println!(" MicroOS Storage & SSD Management CLI (Rust Edition)");
    println!("====================================================");
    println!("Usage: disks [command] [args...]");
    println!();
    println!("Commands:");
    println!("  list                  List all block devices and partitions");
    println!("  info [dev]            Detailed drive and filesystem inspection");
    println!("  mount <dev> [dir]     Safely mount storage partition");
    println!("  unmount <dev|dir>     Safely unmount partition");
    println!("  mounts                List all currently active mount points");
    println!("====================================================");
}

fn cmd_list() {
    println!("------------------ DETECTED DISKS ------------------");
    println!("{:<12} {:<10} {:<16} {:<16}", "DEVICE", "SIZE", "TYPE", "MODEL");
    println!("{:<12} {:<10} {:<16} {:<16}", "------", "----", "----", "-----");

    let block_path = Path::new("/sys/block");
    if let Ok(entries) = fs::read_dir(block_path) {
        for entry in entries.flatten() {
            let dev = entry.file_name().to_string_lossy().into_owned();
            if dev.starts_with("loop") || dev.starts_with("ram") || dev.starts_with("zram") {
                continue;
            }

            let path = entry.path();
            let sectors: u64 = fs::read_to_string(path.join("size"))
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or(0);
            let size_mb = sectors * 512 / 1024 / 1024;

            let mut dev_type = "Block Device";
            if dev.starts_with("nvme") {
                dev_type = "NVMe SSD";
            } else if dev.starts_with("sd") {
                dev_type = "SATA/USB Drive";
            } else if dev.starts_with("sr") {
                dev_type = "CD/DVD-ROM";
            } else if dev.starts_with("vd") {
                dev_type = "VirtIO Disk";
            }

            let model = fs::read_to_string(path.join("device/model"))
                .unwrap_or_default()
                .trim()
                .to_string();
            let model_display = if model.is_empty() { "Standard" } else { &model };

            println!("{:<12} {:<10} {:<16} {:<16}", format!("/dev/{}", dev), format!("{} MB", size_mb), dev_type, model_display);

            // List child partitions
            if let Ok(subentries) = fs::read_dir(&path) {
                for subentry in subentries.flatten() {
                    let part_name = subentry.file_name().to_string_lossy().into_owned();
                    if part_name.starts_with(&dev) && part_name != dev && subentry.path().is_dir() {
                        let psectors: u64 = fs::read_to_string(subentry.path().join("size"))
                            .unwrap_or_default()
                            .trim()
                            .parse()
                            .unwrap_or(0);
                        let pmb = psectors * 512 / 1024 / 1024;

                        let dev_node = format!("/dev/{}", part_name);
                        let mnt = get_mount_point(&dev_node).unwrap_or_else(|| "[unmounted]".to_string());
                        println!("  |- {:<10} {:<8} {:<14} {}", dev_node, format!("{} MB", pmb), "Partition", mnt);
                    }
                }
            }
        }
    }
}

fn get_mount_point(dev: &str) -> Option<String> {
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == dev {
                return Some(parts[1].to_string());
            }
        }
    }
    None
}

fn cmd_info(dev: &str) {
    let bname = dev.trim_start_matches("/dev/");
    println!("--------------- DRIVE DETAILS: /dev/{} --------------", bname);
    let sys_path = Path::new("/sys/class/block").join(bname);
    if sys_path.exists() {
        let sectors: u64 = fs::read_to_string(sys_path.join("size"))
            .unwrap_or_default()
            .trim()
            .parse()
            .unwrap_or(0);
        let bytes = sectors * 512;
        let mb = bytes / 1024 / 1024;
        println!("Capacity       : {} MB ({} bytes)", mb, bytes);
        println!("Total Sectors  : {} (512-byte logical)", sectors);

        let dev_node = format!("/dev/{}", bname);
        if let Some(mnt) = get_mount_point(&dev_node) {
            println!("Mount Status   : MOUNTED on {}", mnt);
        } else {
            println!("Mount Status   : UNMOUNTED");
        }
    } else {
        println!("Error: Device /dev/{} not found.", bname);
    }
}

fn cmd_mount(dev: &str, target: &str) {
    let dev_path = if dev.starts_with("/dev/") { dev.to_string() } else { format!("/dev/{}", dev) };
    let _ = fs::create_dir_all(target);

    println!("Attempting safe mount of {} -> {}...", dev_path, target);
    let rw_ok = Command::new("/bin/mount").args([&dev_path, target]).status().map(|s| s.success()).unwrap_or(false);
    if rw_ok {
        println!("Mounted successfully read-write: {} on {}", dev_path, target);
        return;
    }

    let ro_ok = Command::new("/bin/mount").args(["-o", "ro", &dev_path, target]).status().map(|s| s.success()).unwrap_or(false);
    if ro_ok {
        println!("Mounted successfully read-only (safe fallback): {} on {}", dev_path, target);
    } else {
        println!("Error: Mount failed. Unsupported filesystem or device busy.");
    }
}

fn cmd_unmount(target: &str) {
    println!("Syncing disk buffers...");
    let _ = Command::new("/bin/sync").status();
    println!("Unmounting {}...", target);
    let ok = Command::new("/bin/umount").arg(target).status().map(|s| s.success()).unwrap_or(false);
    if ok {
        println!("Successfully unmounted {}", target);
    } else {
        println!("Error: Failed to unmount {} (in use?).", target);
    }
}

fn cmd_mounts() {
    println!("---------------- ACTIVE MOUNTPOINTS -----------------");
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            if line.starts_with("/dev/") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    println!("{:<16} on {:<16} type {:<8} ({})", parts[0], parts[1], parts[2], parts[3]);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() > 1 { args[1].as_str() } else { "list" };

    match cmd {
        "list" => cmd_list(),
        "info" => {
            let dev = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if dev.is_empty() {
                cmd_list();
            } else {
                cmd_info(dev);
            }
        }
        "mount" => {
            if args.len() < 3 {
                println!("Usage: disks mount <device> [target]");
                return;
            }
            let dev = &args[2];
            let target = args.get(3).map(|s| s.as_str()).unwrap_or("/mnt/ssd");
            cmd_mount(dev, target);
        }
        "unmount" | "umount" => {
            if args.len() < 3 {
                println!("Usage: disks unmount <target>");
                return;
            }
            cmd_unmount(&args[2]);
        }
        "mounts" => cmd_mounts(),
        _ => show_help(),
    }
}
