# MicroOS — Ultra-Lightweight Linux Operating System

MicroOS is a custom-engineered, ultra-lightweight, 64-bit x86_64 Linux distribution designed for direct bare-metal boot and Oracle VirtualBox virtualization.

## Core Specifications

- **ISO Size Budget**: Maximum 50 MB (50,000,000 bytes) hard ceiling.
- **Architecture**: x86_64 (Intel & AMD CPU support with ACPI/APIC).
- **Kernel**: Modular Linux kernel with NVMe, SATA/AHCI, USB, ext4, FAT, and Wi-Fi drivers.
- **Init System**: BusyBox init with custom `/init` device and storage discovery.
- **Shell**: BusyBox ash with customized MicroOS prompt and brand environment.
- **Hardware Detection**: Native sysfs/procfs inspection suite (`hardware`, `disks`, `wifi`).
- **Networking**: Linux TCP/IP stack with Wi-Fi CLI (`wifi scan`, `wifi connect`, `wifi status`).
- **Storage Management**: Non-destructive partition detection, safe mounting (`/mnt/ssd`).
- **Package Architecture**: Persistent Debian-compatible APT/dpkg layer on external/SSD storage.
- **Built-in Micro Arcade**: 8 offline terminal arcade games written in C (`arcade`, `snake`, `tetris`, `pong`, `mines`, `2048`, `guess`, `tictactoe`, `spaceinvaders`).
- **Bootloader**: Hybrid BIOS/UEFI El Torito GRUB configuration.
- **Virtualization Target**: Oracle VirtualBox (1024 MB RAM, 1 vCPU, offline testing).

## Directory Structure

```
MicroOS/
├── boot/           # Bootloader configurations, GRUB menus, and EFI boot images
├── kernel/         # Kernel binaries, configuration scripts, and modules
├── rootfs/         # Root filesystem staging directory (/bin, /sbin, /etc, etc.)
├── init/           # MicroOS /init script, inittab, and rcS startup sequence
├── networking/     # Wi-Fi scripts, DHCP configuration, network scripts
├── hardware/       # Hardware discovery engine (cpu, memory, storage, pci, usb)
├── storage/        # Storage management scripts and persistence mounting tools
├── package-manager/# Debian-compatible APT/dpkg integration and bridge tools
├── shell/          # MicroOS shell branding, prompt configuration, and built-in CLI
├── arcade/         # Micro Arcade launcher
├── games/          # Source code and binaries for the 8 arcade games (C)
├── scripts/        # ISO build, image generation, and packaging scripts
├── configs/        # Kernel config, BusyBox config, and syslinux/grub configs
├── tests/          # VirtualBox test automation and verification suites
├── documentation/  # Architecture, Hardware, Build, Roadmap, and Testing docs
├── build/          # Intermediate build artifacts and progress.json
├── logs/           # Detailed build.log, test.log, and optimization.log
├── release/        # Final bootable microos.iso and SHA256 checksums
├── Makefile        # Automation Makefile for the full build pipeline
└── README.md       # Project overview and specifications
```

## Quick Start (Build & Test)

1. **Build ISO**:
   ```bash
   make all
   ```
2. **Verify ISO Size**:
   ```bash
   make verify-size
   ```
3. **Run VirtualBox Tests**:
   ```bash
   make test-vbox
   ```
