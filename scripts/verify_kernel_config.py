required_patterns = [
    'CONFIG_X86_64=y',
    'CONFIG_SMP=y',
    'CONFIG_ACPI=y',
    'CONFIG_PCI=y',
    'CONFIG_USB=y',
    'CONFIG_ATA=y',
    'CONFIG_BLK_DEV_NVME',
    'CONFIG_SCSI=y',
    'CONFIG_NET=y',
    'CONFIG_ETHERNET=y',
    'CONFIG_WLAN=y',
    'CONFIG_EXT4_FS',
    'CONFIG_VFAT_FS',
    'CONFIG_INPUT=y',
    'CONFIG_TTY=y',
    'CONFIG_VT=y'
]

with open(r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\configs\kernel.config', 'r') as f:
    content = f.read()

print('=== KERNEL CONFIG VERIFICATION ===')
all_found = True
for pat in required_patterns:
    found = any(line.strip().startswith(pat) for line in content.splitlines())
    status = 'OK' if found else 'MISSING'
    print(f'[{status}] {pat}')
    if not found:
        all_found = False

print(f'\nAll required kernel features verified: {all_found}')
