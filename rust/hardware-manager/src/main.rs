//! MicroOS Rust Edition v2.0 - Hardware Discovery Engine
//! Inspects Linux /proc and /sys to report real physical or virtual hardware.

use std::env;
use std::fs;
use std::path::Path;

fn show_help() {
    println!("====================================================");
    println!(" MicroOS Hardware Detection System (Rust Edition)");
    println!("====================================================");
    println!("Usage: hardware [subcommand]");
    println!();
    println!("Subcommands:");
    println!("  cpu       Display CPU model, cores, frequency, flags");
    println!("  memory    Display total, free, and available RAM");
    println!("  network   Display wired and wireless network adapters");
    println!("  storage   Display NVMe, SATA, and block devices");
    println!("  usb       Display USB controllers and connected devices");
    println!("  pci       Display PCI and PCIe devices");
    println!("  all       Display full comprehensive hardware report");
    println!("====================================================");
}

fn detect_cpu() {
    println!("------------------ CPU INFORMATION ------------------");
    if let Ok(info) = fs::read_to_string("/proc/cpuinfo") {
        let mut model = "Unknown Processor";
        let mut cores = 0;
        let mut mhz = "";

        for line in info.lines() {
            if line.starts_with("model name") {
                if let Some(val) = line.split(':').nth(1) {
                    model = val.trim();
                }
            } else if line.starts_with("processor") {
                cores += 1;
            } else if line.starts_with("cpu MHz") {
                if let Some(val) = line.split(':').nth(1) {
                    mhz = val.trim();
                }
            }
        }

        println!("Architecture : x86_64");
        println!("Model Name   : {}", model);
        println!("Cores/vCPUs  : {}", cores);
        if !mhz.is_empty() {
            println!("Clock Speed  : {} MHz", mhz);
        }
    } else {
        println!("Error: /proc/cpuinfo not accessible.");
    }
}

fn detect_memory() {
    println!("----------------- MEMORY INFORMATION ----------------");
    if let Ok(info) = fs::read_to_string("/proc/meminfo") {
        let mut total_kb = 0u64;
        let mut free_kb = 0u64;
        let mut avail_kb = 0u64;

        for line in info.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let val = parts[1].parse::<u64>().unwrap_or(0);
                if line.starts_with("MemTotal:") {
                    total_kb = val;
                } else if line.starts_with("MemFree:") {
                    free_kb = val;
                } else if line.starts_with("MemAvailable:") {
                    avail_kb = val;
                }
            }
        }

        println!("Total RAM     : {:.2} MB ({} kB)", total_kb as f64 / 1024.0, total_kb);
        println!("Free RAM      : {:.2} MB ({} kB)", free_kb as f64 / 1024.0, free_kb);
        println!("Available RAM : {:.2} MB ({} kB)", avail_kb as f64 / 1024.0, avail_kb);
    } else {
        println!("Error: /proc/meminfo not accessible.");
    }
}

fn detect_network() {
    println!("---------------- NETWORK ADAPTERS ------------------");
    let net_path = Path::new("/sys/class/net");
    if let Ok(entries) = fs::read_dir(net_path) {
        for entry in entries.flatten() {
            let iface = entry.file_name().to_string_lossy().into_owned();
            if iface == "lo" {
                continue;
            }

            let path = entry.path();
            let is_wifi = path.join("wireless").exists() || path.join("phy80211").exists();
            let iface_type = if is_wifi { "Wi-Fi (802.11)" } else { "Ethernet" };

            let mac = fs::read_to_string(path.join("address"))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "Unknown".to_string());
            let state = fs::read_to_string(path.join("operstate"))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "UNKNOWN".to_string());

            println!("Device: {}", iface);
            println!("  Type        : {}", iface_type);
            println!("  State       : {}", state);
            println!("  MAC Address : {}", mac);
        }
    } else {
        println!("No network interfaces found in /sys/class/net.");
    }
}

fn detect_storage() {
    println!("----------------- STORAGE DEVICES ------------------");
    let block_path = Path::new("/sys/block");
    if let Ok(entries) = fs::read_dir(block_path) {
        let mut count = 0;
        for entry in entries.flatten() {
            let dev = entry.file_name().to_string_lossy().into_owned();
            if dev.starts_with("loop") || dev.starts_with("ram") || dev.starts_with("zram") {
                continue;
            }
            count += 1;

            let path = entry.path();
            let size_sectors: u64 = fs::read_to_string(path.join("size"))
                .unwrap_or_default()
                .trim()
                .parse()
                .unwrap_or(0);
            let size_mb = size_sectors * 512 / 1024 / 1024;

            let mut bus_type = "Block Device";
            if dev.starts_with("nvme") {
                bus_type = "NVMe PCIe";
            } else if dev.starts_with("sd") {
                bus_type = "SATA / USB Mass Storage";
            } else if dev.starts_with("sr") {
                bus_type = "CD/DVD-ROM";
            } else if dev.starts_with("vd") {
                bus_type = "VirtIO Disk";
            }

            let model = fs::read_to_string(path.join("device/model"))
                .unwrap_or_default()
                .trim()
                .to_string();
            let model_display = if model.is_empty() { "Standard Storage Device" } else { &model };

            println!("Drive: /dev/{}", dev);
            println!("  Model       : {}", model_display);
            println!("  Bus/Type    : {}", bus_type);
            println!("  Capacity    : {} MB ({} GB)", size_mb, size_mb / 1024);
        }
        if count == 0 {
            println!("No block devices detected.");
        }
    } else {
        println!("Error: /sys/block not accessible.");
    }
}

fn detect_usb() {
    println!("------------------- USB DEVICES --------------------");
    let usb_path = Path::new("/sys/bus/usb/devices");
    if let Ok(entries) = fs::read_dir(usb_path) {
        let mut count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(vendor) = fs::read_to_string(path.join("idVendor")) {
                count += 1;
                let product = fs::read_to_string(path.join("idProduct")).unwrap_or_default();
                let mfr = fs::read_to_string(path.join("manufacturer")).unwrap_or_default();
                let prod = fs::read_to_string(path.join("product")).unwrap_or_default();
                let dev_id = entry.file_name().to_string_lossy().into_owned();

                println!("USB [{}] ID: {}:{}", dev_id, vendor.trim(), product.trim());
                if !mfr.trim().is_empty() {
                    println!("  Manufacturer: {}", mfr.trim());
                }
                if !prod.trim().is_empty() {
                    println!("  Product     : {}", prod.trim());
                }
            }
        }
        if count == 0 {
            println!("No external USB devices detected.");
        }
    } else {
        println!("Error: /sys/bus/usb/devices not accessible.");
    }
}

fn detect_pci() {
    println!("------------------- PCI DEVICES --------------------");
    let pci_path = Path::new("/sys/bus/pci/devices");
    if let Ok(entries) = fs::read_dir(pci_path) {
        let mut count = 0;
        for entry in entries.flatten() {
            count += 1;
            let path = entry.path();
            let slot = entry.file_name().to_string_lossy().into_owned();
            let vendor = fs::read_to_string(path.join("vendor")).unwrap_or_default();
            let device = fs::read_to_string(path.join("device")).unwrap_or_default();
            let class = fs::read_to_string(path.join("class")).unwrap_or_default();

            println!("PCI [{}] Class: {} ID: {}:{}", slot, class.trim(), vendor.trim(), device.trim());
        }
        println!("Total PCI/PCIe devices: {}", count);
    } else {
        println!("Error: /sys/bus/pci/devices not accessible.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd = if args.len() > 1 { args[1].as_str() } else { "all" };

    match subcmd {
        "cpu" => detect_cpu(),
        "mem" | "memory" => detect_memory(),
        "net" | "network" => detect_network(),
        "stor" | "storage" => detect_storage(),
        "usb" => detect_usb(),
        "pci" => detect_pci(),
        "all" => {
            println!("====================================================");
            println!("       MICROOS HARDWARE DISCOVERY REPORT (RUST)     ");
            println!("====================================================");
            detect_cpu();
            println!();
            detect_memory();
            println!();
            detect_storage();
            println!();
            detect_network();
            println!();
            detect_usb();
            println!();
            detect_pci();
            println!("====================================================");
        }
        "help" | "--help" | "-h" => show_help(),
        other => {
            println!("Unknown hardware target: '{}'", other);
            show_help();
        }
    }
}
