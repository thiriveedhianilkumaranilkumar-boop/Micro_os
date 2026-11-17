# MicroOS v1.0.0 Release Notes

MicroOS is an ultra-lightweight, 64-bit x86_64 Linux operating system engineered for native bare-metal execution and Oracle VirtualBox virtualization within a strict 50 MB ISO budget.

## Key Release Features
- **Final ISO Size**: 13,768,704 bytes (13.13 MB) — well below the 50 MB ceiling.
- **Kernel**: Linux LTS 6.6.134 with Intel & AMD support, ACPI, APIC, NVMe, SATA, USB, ext4, and Wi-Fi drivers.
- **Bootloader**: Dual BIOS & UEFI boot support via El Torito boot catalog.
- **Root Filesystem**: BusyBox static userland, complete ash shell, and custom /init sequence.
- **Hardware Discovery**: Native sysfs/procfs inspection suite (`hardware cpu`, `memory`, `storage`, `network`, `usb`, `pci`).
- **Wi-Fi & Networking**: Linux networking CLI (`wifi status`, `scan`, `connect`, `disconnect`).
- **Storage Management**: Non-destructive disk inspection (`disks list`, `info`, `mount`, `unmount`).
- **Persistent Application Architecture**: Debian-compatible APT/dpkg layer decoupled onto persistent SSD media (`/mnt/ssd`).
- **Micro Arcade**: 8 built-in terminal arcade games (`snake`, `tetris`, `pong`, `mines`, `2048`, `guess`, `tictactoe`, `spaceinvaders`) and `arcade` launcher.

## File Verification
- **ISO**: `microos.iso`
- **SHA256**: `51ce4c11efb1a6350e0f326580cde3d62a9900a5292f38aca4353c2e708ea72a`
