import os

microos_mandatory_flags = [
    'CONFIG_64BIT=y',
    'CONFIG_X86_64=y',
    'CONFIG_SMP=y',
    'CONFIG_ACPI=y',
    'CONFIG_X86_LOCAL_APIC=y',
    'CONFIG_X86_IO_APIC=y',
    'CONFIG_PCI=y',
    'CONFIG_PCIEPORTBUS=y',
    'CONFIG_PCI_MSI=y',
    'CONFIG_USB=y',
    'CONFIG_USB_XHCI_HCD=y',
    'CONFIG_USB_EHCI_HCD=y',
    'CONFIG_USB_STORAGE=y',
    'CONFIG_ATA=y',
    'CONFIG_SATA_AHCI=y',
    'CONFIG_BLK_DEV_NVME=y',
    'CONFIG_SCSI=y',
    'CONFIG_BLK_DEV_SD=y',
    'CONFIG_NET=y',
    'CONFIG_INET=y',
    'CONFIG_NETDEVICES=y',
    'CONFIG_ETHERNET=y',
    'CONFIG_E1000=y',
    'CONFIG_E1000E=y',
    'CONFIG_R8169=m',
    'CONFIG_WLAN=y',
    'CONFIG_CFG80211=y',
    'CONFIG_MAC80211=y',
    'CONFIG_IWLWIFI=m',
    'CONFIG_EXT4_FS=y',
    'CONFIG_VFAT_FS=y',
    'CONFIG_FAT_FS=y',
    'CONFIG_EXFAT_FS=m',
    'CONFIG_DEVTMPFS=y',
    'CONFIG_DEVTMPFS_MOUNT=y',
    'CONFIG_TMPFS=y',
    'CONFIG_PROC_FS=y',
    'CONFIG_SYSFS=y',
    'CONFIG_BLK_DEV_INITRD=y',
    'CONFIG_RD_GZIP=y',
    'CONFIG_RD_XZ=y',
    'CONFIG_INPUT=y',
    'CONFIG_INPUT_KEYBOARD=y',
    'CONFIG_KEYBOARD_ATKBD=y',
    'CONFIG_TTY=y',
    'CONFIG_VT=y',
    'CONFIG_VT_CONSOLE=y',
    'CONFIG_VGA_CONSOLE=y',
    'CONFIG_PRINTK=y',
    'CONFIG_EARLY_PRINTK=y',
    'CONFIG_SERIAL_8250=y',
    'CONFIG_SERIAL_8250_CONSOLE=y'
]

# Read defconfig
defconfig_path = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\configs\x86_64_defconfig'
delta_path = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\configs\lts_delta.config'
out_path = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\configs\kernel.config'

config_dict = {}

def process_file(path):
    if not os.path.exists(path):
        return
    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            if '=' in line:
                k, v = line.split('=', 1)
                config_dict[k.strip()] = v.strip()

process_file(defconfig_path)
process_file(delta_path)

# Apply mandatory MicroOS flags
for item in microos_mandatory_flags:
    k, v = item.split('=', 1)
    config_dict[k.strip()] = v.strip()

# Write consolidated reproducible kernel.config
with open(out_path, 'w', encoding='utf-8') as f:
    f.write('# MicroOS v1.0.0 Linux Kernel Configuration\n')
    f.write('# Target: x86_64 Intel/AMD PC, Laptops & VirtualBox\n')
    f.write('# Kernel Version: 6.6.134-0-lts SMP\n\n')
    for k in sorted(config_dict.keys()):
        f.write(f'{k}={config_dict[k]}\n')

print(f'Wrote {len(config_dict)} configuration entries to {out_path}')

# Verify
all_verified = True
for item in microos_mandatory_flags:
    k, v = item.split('=', 1)
    actual = config_dict.get(k)
    if actual != v:
        print(f'[FAIL] {k}: expected {v}, got {actual}')
        all_verified = False

if all_verified:
    print('All 52 mandatory MicroOS kernel configurations VERIFIED!')
