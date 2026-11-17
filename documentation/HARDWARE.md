# MicroOS Hardware Support & Detection Architecture

MicroOS is designed for direct native hardware execution on physical laptops and workstations (Intel & AMD x86_64) as well as Oracle VirtualBox virtualization.

## Supported Core Subsystems

1. **Processors**:
   - 64-bit x86_64 CPU architecture.
   - Intel Core (Nehalem through Raptor Lake / Meteor Lake).
   - AMD (K8, Zen 1 through Zen 5).
   - Multi-core SMP, APIC, ACPI power and thermal management.

2. **Memory**:
   - Detection and dynamic mapping of physical RAM via `/proc/meminfo` and sysfs memory blocks.
   - Tested under resource-constrained conditions down to 1024 MB RAM.

3. **Storage Controllers & Drives**:
   - NVMe PCIe storage controllers (`/dev/nvme*`).
   - AHCI / SATA HDD and SSD controllers (`/dev/sd*`).
   - USB Mass Storage (flash drives, external SSDs) (`/dev/sd*`).
   - Partitions: GPT (GUID Partition Table) and legacy MBR.
   - Filesystems: ext4 (native Linux), FAT32 / VFAT (EFI/boot media), exFAT, and NTFS read support.

4. **PCI and PCIe Devices**:
   - Dynamic enumeration through sysfs (`/sys/bus/pci/devices/`).
   - Class-based identification (VGA, Network, Mass Storage, Bridge).

5. **USB Devices**:
   - USB 2.0 (EHCI) and USB 3.0 (xHCI) controllers.
   - Enumeration of input devices (keyboards, mice) and storage controllers via `/sys/bus/usb/devices/`.

6. **Networking & Wireless (Wi-Fi)**:
   - Wired Ethernet: Intel e1000/e1000e (common in physical laptops and VirtualBox), Realtek r8169.
   - Wireless controllers: Intel Wireless (iwlwifi), Realtek (rtw88/rtl8192), MediaTek (mt76).
   - Hardware detection engine inspects `/sys/class/net/` and distinguishes between `wlan*` (physical 802.11) and `eth*` interfaces.
