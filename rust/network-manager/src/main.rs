//! MicroOS Rust Edition v2.0 - Network Management Daemon
//! Native Linux networking interface manager and status inspector.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn show_help() {
    println!("====================================================");
    println!(" MicroOS Network Manager (Rust Edition)");
    println!("====================================================");
    println!("Usage: netmgr [command] [args...]");
    println!();
    println!("Commands:");
    println!("  status         Display overall IP, gateway, and DNS configuration");
    println!("  interfaces     List network interfaces with MAC and state");
    println!("  up <iface>     Bring network interface up");
    println!("  down <iface>   Bring network interface down");
    println!("  dhcp <iface>   Request dynamic IPv4 address via udhcpc");
    println!("====================================================");
}

fn cmd_status() {
    println!("---------------- NETWORK STATUS --------------------");
    let net_path = Path::new("/sys/class/net");
    if let Ok(entries) = fs::read_dir(net_path) {
        for entry in entries.flatten() {
            let iface = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let is_wifi = path.join("wireless").exists() || path.join("phy80211").exists();
            let dev_type = if iface == "lo" {
                "Loopback"
            } else if is_wifi {
                "Wi-Fi (802.11)"
            } else {
                "Wired Ethernet"
            };

            let oper = fs::read_to_string(path.join("operstate"))
                .unwrap_or_else(|_| "UNKNOWN".to_string())
                .trim()
                .to_string();

            println!("Interface: {} ({})", iface, dev_type);
            println!("  Link State : {}", oper);

            // Fetch IP via /bin/ip if available
            if let Ok(output) = Command::new("/bin/ip")
                .args(["-4", "addr", "show", "dev", &iface])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    if line.contains("inet ") {
                        let ip = line.split_whitespace().nth(1).unwrap_or("");
                        println!("  IPv4 Addr  : {}", ip);
                    }
                }
            }
        }
    }

    println!();
    println!("Default Gateway:");
    if let Ok(output) = Command::new("/bin/ip").args(["route", "show", "default"]).output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            println!("  {}", line);
        }
    }

    println!("DNS Resolvers (/etc/resolv.conf):");
    if let Ok(resolv) = fs::read_to_string("/etc/resolv.conf") {
        for line in resolv.lines() {
            if line.starts_with("nameserver") {
                println!("  {}", line);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = if args.len() > 1 { args[1].as_str() } else { "status" };

    match cmd {
        "status" => cmd_status(),
        "interfaces" => {
            println!("Network interfaces:");
            if let Ok(entries) = fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    println!("  - {}", name);
                }
            }
        }
        "up" if args.len() > 2 => {
            let iface = &args[2];
            let _ = Command::new("/bin/ip").args(["link", "set", iface, "up"]).status();
            println!("Interface {} set UP", iface);
        }
        "down" if args.len() > 2 => {
            let iface = &args[2];
            let _ = Command::new("/bin/ip").args(["link", "set", iface, "down"]).status();
            println!("Interface {} set DOWN", iface);
        }
        "dhcp" if args.len() > 2 => {
            let iface = &args[2];
            println!("Requesting DHCP lease on {}...", iface);
            let _ = Command::new("/sbin/udhcpc").args(["-i", iface, "-b", "-t", "5"]).status();
        }
        _ => show_help(),
    }
}
