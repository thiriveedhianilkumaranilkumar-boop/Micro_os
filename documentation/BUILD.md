# MicroOS Build System & Pipeline

The MicroOS build system produces a reproducible, hybrid BIOS/UEFI bootable ISO within the strict 50 MB budget limit.

## Build Pipeline Components

1. **Kernel Provisioning & Configuration**:
   - Upstream Linux x86_64 kernel tailored with essential drivers:
     - Processor support: Intel & AMD x86_64
     - Storage: NVMe, AHCI/SATA, USB Mass Storage, SCSI disk
     - Filesystems: ext4, vfat, devtmpfs, tmpfs, overlayfs
     - Networking: TCP/IP, packet filter, mac80211, cfg80211
     - Input/Console: TTY, VT, standard PC keyboard/mouse
   - Kernel compression: XZ or GZIP compressed bzImage / vmlinuz.

2. **Root Filesystem (Initramfs)**:
   - Rootfs structure:
     `/bin`, `/sbin`, `/usr/bin`, `/usr/sbin`, `/proc`, `/sys`, `/dev`, `/tmp`, `/run`, `/etc`, `/mnt`, `/root`, `/home`
   - BusyBox statically compiled providing:
     - Core utilities (`ls`, `cat`, `ps`, `df`, `free`, `dmesg`, `grep`, `awk`, `sed`, `tar`, etc.)
     - Init system (`init`, `inittab`, `rcS`)
     - Networking utilities (`ip`, `ifconfig`, `udhcpc`, `route`)
     - Storage tools (`mount`, `umount`, `fdisk`, `blkid`)
   - Custom MicroOS utilities:
     - `/bin/hardware`: Modular hardware discovery suite
     - `/bin/wifi`: Wi-Fi management CLI
     - `/bin/disks`: Storage discovery and management
     - `/bin/apt`: APT persistence bridge
     - `/bin/arcade`: Interactive terminal game launcher
   - Built-in arcade games:
     - `snake`, `tetris`, `pong`, `mines`, `2048`, `guess`, `tictactoe`, `spaceinvaders`
   - Archive generation:
     - CPIO newc format compressed with maximum gzip / xz compression.

3. **Bootloader Architecture**:
   - GRUB 2.0x / El Torito bootloader configuration.
   - Hybrid ISO filesystem configured with BIOS El Torito boot catalog and EFI system partition.
