# MicroOS Rust Edition v2.0 Release Notes

MicroOS Rust Edition v2.0 is a complete, memory-safe, Rust-first Linux distribution engineered for direct bare-metal hardware execution and Oracle VirtualBox virtualization within a strictly enforced 50 MB ISO budget.

## Key Features in v2.0
- **Final Measured ISO Size**: 17,518,592 bytes (16.71 MB) - 32+ MB under budget!
- **100% Rust Early Userspace**: Native static Rust /init sequence replacing legacy shell init.
- **100% Rust Primary Shell**: `micro-shell` providing custom prompt, command validation, and built-in execution.
- **100% Rust System Daemons**: `hardware`, `disks`, `netmgr`, `wifi`, `apt`, `configmgr`, and `sysinfo`.
- **100% Rust Micro Arcade**: All 8 offline terminal arcade games (`snake`, `tetris`, `pong`, `mines`, `2048`, `guess`, `tictactoe`, `spaceinvaders`) rewritten in safe Rust.
- **Linux LTS 6.6 Kernel**: Upstream modular Linux kernel retained for maximum laptop hardware driver compatibility.
- **Debian Bookworm Persistent Layer**: Persistent `/mnt/ssd` storage retains package caches, downloaded .deb packages, and high scores.
- **Dual BIOS & UEFI Bootloader**: Tested and validated on legacy BIOS and modern UEFI hardware.
