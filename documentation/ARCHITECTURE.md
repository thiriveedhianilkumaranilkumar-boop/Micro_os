# MicroOS System Architecture

MicroOS is built around a decoupled architecture designed to deliver an instant-booting, sub-50MB base operating system while enabling full Debian-compatible APT/dpkg application extensibility through persistent external or dedicated internal storage.

```
+--------------------------------------------------------------------------+
|                            MicroOS Terminal                              |
|   Commands: help, sysinfo, hardware, wifi, disks, mount, unmount,        |
|             apt, arcade (snake, tetris, pong, mines, 2048, etc.)         |
+--------------------------------------------------------------------------+
|                       Userspace & Utilities                              |
|   BusyBox ash shell + coreutils + sysfs/procfs hardware tools            |
|   Persistent Layer: Debian-compatible glibc + dpkg + apt (/mnt/ssd)      |
+--------------------------------------------------------------------------+
|                         MicroOS Init System                              |
|   /init script -> devtmpfs, proc, sys, persistent storage detection      |
|   BusyBox init -> inittab -> respawn shell & arcade                      |
+--------------------------------------------------------------------------+
|                         Linux Kernel x86_64                              |
|   Modular/Built-in: ACPI, APIC, PCI/PCIe, NVMe, AHCI/SATA, USB 2/3       |
|   Networking: TCP/IP stack, mac80211, cfg80211, wireless drivers         |
|   Filesystems: ext4, vfat, exfat, devtmpfs, tmpfs, sysfs, procfs         |
+--------------------------------------------------------------------------+
|                             Bootloader                                   |
|   GRUB / El Torito Hybrid ISO (BIOS & UEFI support)                      |
+--------------------------------------------------------------------------+
```

## Architectural Principles

1. **Sub-50MB Budget Strict Enforcement**:
   The base ISO contains only what is essential for bare-metal boot, hardware discovery, storage management, native networking, Wi-Fi connectivity, and the built-in terminal arcade. All heavy package caches and secondary tools reside on persistent SSD media.
2. **Dual-Environment Strategy**:
   - *Core Boot Environment*: Powered by an ultra-compact statically linked BusyBox and C binaries.
   - *Extended Application Environment*: A persistent storage partition (`/mnt/ssd` or external USB) houses a Debian-compatible rootfs overlay with APT and dpkg, preventing library collisions and bloat in the core boot media.
3. **Pure Terminal Interface**:
   Zero X11 or Wayland dependencies. Terminal rendering uses ANSI escape sequences and VT100-compliant C routines for zero-overhead, high-performance interactive CLI and gaming.
4. **Offline First**:
   All core capabilities, including hardware inspection, disk formatting/mounting, and the 8 arcade games, operate 100% offline without network requirements.
