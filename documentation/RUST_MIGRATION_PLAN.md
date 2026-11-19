# MicroOS Rust Edition v2.0 — Migration Plan & Inventory

This document establishes the component-by-component migration roadmap from the legacy C and shell-based MicroOS implementation to the Rust-first MicroOS Edition v2.0 architecture.

## Architectural Goal

Transform MicroOS into a high-reliability, memory-safe, Rust-first Linux distribution:
- **Kernel**: Retain upstream Linux LTS 6.6 (`6.6.134-0-lts x86_64 SMP`) with native C hardware drivers.
- **Early Userspace**: Replace shell `/init` with a native static Rust init system (`rust/system-init`).
- **Shell**: Replace BusyBox ash with the custom Rust MicroOS shell (`rust/micro-shell`).
- **System Services & Utilities**: Rewrite hardware detection, storage management, networking, Wi-Fi, package management bridge, and system monitoring in 100% safe Rust.
- **Micro Arcade**: Rewrite all 8 terminal arcade games and the arcade launcher in 100% Rust.
- **Persistent Userspace**: Maintain Debian-compatible APT/dpkg ecosystem on persistent `/mnt/ssd`.
- **ISO Budget**: Strictly enforce the <= 50,000,000 bytes (50 MB) budget ceiling.

---

## Migration Inventory & Component Tracking

| Existing Component | Original Language | Functionality | Rust Replacement | Dependencies | Migration Status | Validation Status |
|---|---|---|---|---|---|---|
| `init/init.sh` | Shell | Mounts proc/sys/dev, mdev, persistence detection, init handoff | `rust/system-init` (`/init`) | Linux syscalls, libc/nix | IN_PROGRESS | Pending Build |
| `shell/profile` + ash | Shell / BusyBox | Interactive command-line shell and prompt | `rust/micro-shell` (`/bin/micro-shell`, `/bin/sh`) | POSIX termios, ANSI | IN_PROGRESS | Pending Build |
| `hardware/hardware.sh` | Shell | CPU, RAM, storage, network, usb, pci discovery | `rust/hardware-manager` (`/bin/hardware`) | `/sys`, `/proc` parsing | IN_PROGRESS | Pending Build |
| `storage/disks.sh` | Shell | Non-destructive disk listing, info, safe mount/umount | `rust/storage-manager` (`/bin/disks`, `/bin/mount`, `/bin/unmount`) | `/sys/block`, mount syscall | IN_PROGRESS | Pending Build |
| `networking/wifi.sh` | Shell | Wi-Fi scanning, connection, status, dhcp trigger | `rust/wifi-manager` (`/bin/wifi`) | `/sys/class/net`, iwd/udhcpc | IN_PROGRESS | Pending Build |
| Networking scripts | Shell | Interface setup, IP configuration, gateway & DNS | `rust/network-manager` (`/bin/netmgr`) | Linux socket/ioctl | IN_PROGRESS | Pending Build |
| `package-manager/apt.sh`| Shell | Bridge to persistent Debian APT/dpkg on `/mnt/ssd` | `rust/package-manager` (`/bin/apt`) | APT/dpkg CLI invocation | IN_PROGRESS | Pending Build |
| `shell/sysinfo.sh` | Shell | High-contrast system overview and hardware banner | `rust/system-monitor` (`/bin/sysinfo`) | `/proc/uptime`, `/proc/meminfo` | IN_PROGRESS | Pending Build |
| `arcade/arcade.c` | C | Interactive terminal menu game launcher | `rust/arcade` (`/bin/arcade`) | ANSI, raw termios | IN_PROGRESS | Pending Build |
| `games/snake.c` | C | Real-time Snake with WASD/arrow keys, high scores | `games/snake` (`/bin/snake`) | ANSI, raw termios, timer | IN_PROGRESS | Pending Build |
| `games/tetris.c` | C | Falling tetrominoes, rotation, line clears, score | `games/tetris` (`/bin/tetris`) | ANSI, raw termios, timer | IN_PROGRESS | Pending Build |
| `games/pong.c` | C | 2-player paddle mechanics and ball trajectory | `games/pong` (`/bin/pong`) | ANSI, raw termios, timer | IN_PROGRESS | Pending Build |
| `games/mines.c` | C | Minesweeper minefield, tile reveal, flagging | `games/minesweeper` (`/bin/mines`) | ANSI, raw termios | IN_PROGRESS | Pending Build |
| `games/2048.c` | C | 4-way tile sliding, exponential merging | `games/game2048` (`/bin/2048`) | ANSI, raw termios | IN_PROGRESS | Pending Build |
| `games/guess.c` | C | Number guessing with difficulty and attempts | `games/number-guessing` (`/bin/guess`)| Standard I/O, random | IN_PROGRESS | Pending Build |
| `games/tictactoe.c` | C | 2-player or AI computer opponent | `games/tic-tac-toe` (`/bin/tictactoe`)| Standard I/O, minimax AI | IN_PROGRESS | Pending Build |
| `games/spaceinvaders.c`| C | ASCII base defense, shooting, advancing aliens | `games/space-invaders` (`/bin/spaceinvaders`)| ANSI, raw termios, timer | IN_PROGRESS | Pending Build |

---

## Safety & Rollback Strategy

1. **Non-Destructive Migration**: Original C source files in `arcade/arcade.c` and `games/*.c` and shell scripts in `hardware/`, `networking/`, `storage/`, `package-manager/` are preserved in place during the migration.
2. **Side-by-Side Validation**: Each Rust component is verified and benchmarked against its original implementation before replacing the initramfs entry.
3. **No External Network Dependencies**: All Rust crates are self-contained using safe Rust and standard library capabilities, preventing cargo registry network failures.
4. **Target Toolchain**: `x86_64-unknown-linux-musl` with internal `rust-lld` linker for zero-dependency static ELF64 binaries.
