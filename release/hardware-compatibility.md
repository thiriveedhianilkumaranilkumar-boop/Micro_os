# MicroOS Rust Edition v2.0 Hardware Compatibility Matrix

| Subsystem | Chipsets / Controllers | Driver / Implementation | VirtualBox Status | Physical Laptop Status |
|---|---|---|---|---|
| CPU Architecture | Intel Core (i3-i9) & AMD Ryzen | CONFIG_X86_64=y | PASSED | PENDING BARE-METAL FLASH |
| Memory (RAM) | DDR3 / DDR4 / DDR5 / LPDDR | Rust sysfs/procfs mapper | PASSED (1024 MB) | PENDING BARE-METAL FLASH |
| NVMe PCIe SSDs | M.2 NVMe PCIe Gen 3/4/5 | CONFIG_BLK_DEV_NVME=y | Implemented | PENDING BARE-METAL FLASH |
| SATA SSDs & HDDs | AHCI SATA 2.5" SSD & M.2 | CONFIG_SATA_AHCI=y | PASSED (/dev/sda) | PENDING BARE-METAL FLASH |
| USB Mass Storage | USB 2.0 / 3.0 / 3.1 / 3.2 | CONFIG_USB_STORAGE=y | Implemented | PENDING BARE-METAL FLASH |
| Ethernet | Intel e1000/e1000e, Realtek | CONFIG_E1000=y | PASSED | PENDING BARE-METAL FLASH |
| Wi-Fi (802.11) | Intel iwlwifi, Realtek, MediaTek | CONFIG_CFG80211=y, wifi CLI | Emulated Eth detected | PENDING BARE-METAL FLASH |
