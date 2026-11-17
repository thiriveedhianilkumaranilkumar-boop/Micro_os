#!/usr/bin/env python3
"""
MicroOS Master ISO Image Generator
Constructs a dual-boot (BIOS + UEFI) hybrid bootable ISO image
under the strict 50,000,000 byte maximum limit.
"""
import os, sys, shutil, hashlib, struct, pycdlib

BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
BUILD_DIR = os.path.join(BASE_DIR, "build")
RELEASE_DIR = os.path.join(BASE_DIR, "release")
BOOT_DIR = os.path.join(BASE_DIR, "boot")

KERNEL_PATH = os.path.join(BASE_DIR, "kernel", "vmlinuz")
INITRD_PATH = os.path.join(BUILD_DIR, "initramfs.cpio.gz")
ISO_OUTPUT = os.path.join(RELEASE_DIR, "microos.iso")
SHA256_OUTPUT = os.path.join(RELEASE_DIR, "microos.iso.sha256")
BUILD_INFO_OUT = os.path.join(RELEASE_DIR, "build-info.txt")
RELEASE_NOTES_OUT = os.path.join(RELEASE_DIR, "release-notes.md")

MAX_ISO_BYTES = 50_000_000 # 50 MB hard budget

def create_fat12_efi(img_path, efi_bin_path, cfg_path):
    """Generates a FAT12 disk image containing /EFI/BOOT/BOOTX64.EFI and config"""
    total_sectors = 2880
    bytes_per_sector = 512
    sectors_per_cluster = 1
    reserved_sectors = 1
    fat_count = 2
    root_entries = 224
    sectors_per_fat = 9
    
    with open(efi_bin_path, 'rb') as f:
        efi_data = f.read()
    with open(cfg_path, 'rb') as f:
        cfg_data = f.read()

    img = bytearray(total_sectors * bytes_per_sector)
    
    # BPB Header
    boot_sec = bytearray(512)
    boot_sec[0:3] = b'\xeb\x3c\x90'
    boot_sec[3:11] = b'MSDOS5.0'
    struct.pack_into('<H', boot_sec, 11, bytes_per_sector)
    boot_sec[13] = sectors_per_cluster
    struct.pack_into('<H', boot_sec, 14, reserved_sectors)
    boot_sec[16] = fat_count
    struct.pack_into('<H', boot_sec, 17, root_entries)
    struct.pack_into('<H', boot_sec, 19, total_sectors)
    boot_sec[21] = 0xf0
    struct.pack_into('<H', boot_sec, 22, sectors_per_fat)
    struct.pack_into('<H', boot_sec, 24, 18)
    struct.pack_into('<H', boot_sec, 26, 2)
    boot_sec[38] = 0x29
    struct.pack_into('<I', boot_sec, 39, 0x12345678)
    boot_sec[43:54] = b'MICROOS_EFI'
    boot_sec[54:62] = b'FAT12   '
    boot_sec[510:512] = b'\x55\xaa'
    img[0:512] = boot_sec
    
    fat1_offset = reserved_sectors * bytes_per_sector
    fat2_offset = fat1_offset + sectors_per_fat * bytes_per_sector
    root_offset = fat2_offset + sectors_per_fat * bytes_per_sector
    root_sectors = (root_entries * 32 + bytes_per_sector - 1) // bytes_per_sector
    data_offset = root_offset + root_sectors * bytes_per_sector
    
    img[fat1_offset:fat1_offset+3] = b'\xf0\xff\xff'
    img[fat2_offset:fat2_offset+3] = b'\xf0\xff\xff'
    
    fat1_view = memoryview(img)[fat1_offset:fat1_offset + sectors_per_fat * bytes_per_sector]
    def set_fat12(cluster, val):
        off = (cluster * 3) // 2
        if cluster % 2 == 0:
            fat1_view[off] = val & 0xff
            fat1_view[off + 1] = (fat1_view[off + 1] & 0xf0) | ((val >> 8) & 0x0f)
        else:
            fat1_view[off] = (fat1_view[off] & 0x0f) | ((val << 4) & 0xf0)
            fat1_view[off + 1] = (val >> 4) & 0xff

    # Clusters: 2=EFI dir, 3=BOOT dir
    set_fat12(2, 0xfff)
    set_fat12(3, 0xfff)
    
    # Root Dir: EFI entry
    root_view = memoryview(img)[root_offset:root_offset + root_sectors * bytes_per_sector]
    root_entry = bytearray(32)
    root_entry[0:11] = b'EFI        '
    root_entry[11] = 0x10 # DIR
    struct.pack_into('<H', root_entry, 26, 2)
    root_view[0:32] = root_entry
    
    # Cluster 2 (EFI dir)
    c2 = data_offset + (2 - 2) * bytes_per_sector * sectors_per_cluster
    dot = bytearray(32); dot[0:11] = b'.          '; dot[11] = 0x10; struct.pack_into('<H', dot, 26, 2)
    dotdot = bytearray(32); dotdot[0:11] = b'..         '; dotdot[11] = 0x10; struct.pack_into('<H', dotdot, 26, 0)
    b_dir = bytearray(32); b_dir[0:11] = b'BOOT       '; b_dir[11] = 0x10; struct.pack_into('<H', b_dir, 26, 3)
    img[c2:c2+32] = dot; img[c2+32:c2+64] = dotdot; img[c2+64:c2+96] = b_dir
    
    # Cluster 3 (BOOT dir): entries for BOOTX64.EFI and SYSLINUX.CFG
    c3 = data_offset + (3 - 2) * bytes_per_sector * sectors_per_cluster
    dot3 = bytearray(32); dot3[0:11] = b'.          '; dot3[11] = 0x10; struct.pack_into('<H', dot3, 26, 3)
    dotdot3 = bytearray(32); dotdot3[0:11] = b'..         '; dotdot3[11] = 0x10; struct.pack_into('<H', dotdot3, 26, 2)
    
    f_efi = bytearray(32); f_efi[0:11] = b'BOOTX64 EFI'; f_efi[11] = 0x20
    struct.pack_into('<H', f_efi, 26, 4)
    struct.pack_into('<I', f_efi, 28, len(efi_data))
    
    f_cfg = bytearray(32); f_cfg[0:11] = b'SYSLINUXCFG'; f_cfg[11] = 0x20
    
    # Write BOOTX64.EFI starting at cluster 4
    n_clust_efi = (len(efi_data) + 511) // 512
    for i in range(n_clust_efi):
        c = 4 + i
        nxt = 0xfff if i == n_clust_efi - 1 else c + 1
        set_fat12(c, nxt)
        c_off = data_offset + (c - 2) * 512
        chunk = efi_data[i * 512 : (i + 1) * 512]
        img[c_off:c_off+len(chunk)] = chunk
        
    cfg_start_cluster = 4 + n_clust_efi
    struct.pack_into('<H', f_cfg, 26, cfg_start_cluster)
    struct.pack_into('<I', f_cfg, 28, len(cfg_data))
    
    img[c3:c3+32] = dot3; img[c3+32:c3+64] = dotdot3
    img[c3+64:c3+96] = f_efi; img[c3+96:c3+128] = f_cfg
    
    n_clust_cfg = (len(cfg_data) + 511) // 512
    for i in range(n_clust_cfg):
        c = cfg_start_cluster + i
        nxt = 0xfff if i == n_clust_cfg - 1 else c + 1
        set_fat12(c, nxt)
        c_off = data_offset + (c - 2) * 512
        chunk = cfg_data[i * 512 : (i + 1) * 512]
        img[c_off:c_off+len(chunk)] = chunk
        
    img[fat2_offset:fat2_offset + sectors_per_fat * bytes_per_sector] = img[fat1_offset:fat1_offset + sectors_per_fat * bytes_per_sector]
    
    with open(img_path, 'wb') as f:
        f.write(img)

def main():
    print("=== Generating MicroOS Bootable Hybrid ISO ===")
    os.makedirs(RELEASE_DIR, exist_ok=True)
    
    if not os.path.exists(KERNEL_PATH):
        raise FileNotFoundError(f"Kernel binary missing: {KERNEL_PATH}")
    if not os.path.exists(INITRD_PATH):
        raise FileNotFoundError(f"Initramfs missing: {INITRD_PATH}")
        
    # 1. Build efi.img
    efi_bin = os.path.join(BOOT_DIR, "usr", "share", "syslinux", "efi64", "syslinux.efi")
    efi_cfg = os.path.join(BOOT_DIR, "isolinux.cfg")
    efi_img_path = os.path.join(BUILD_DIR, "efi.img")
    create_fat12_efi(efi_img_path, efi_bin, efi_cfg)
    
    # 2. Build ISO with pycdlib
    if os.path.exists(ISO_OUTPUT):
        os.remove(ISO_OUTPUT)
        
    iso = pycdlib.PyCdlib()
    iso.new(interchange_level=3, rock_ridge='1.09', joliet=3, vol_ident='MICROOS')
    
    # Directories
    iso.add_directory('/BOOT', rr_name='boot', joliet_path='/boot')
    iso.add_directory('/BOOT/ISOLINUX', rr_name='isolinux', joliet_path='/boot/isolinux')
    iso.add_directory('/BOOT/GRUB', rr_name='grub', joliet_path='/boot/grub')
    iso.add_directory('/EFI', rr_name='efi', joliet_path='/efi')
    iso.add_directory('/EFI/BOOT', rr_name='boot', joliet_path='/efi/boot')
    
    # Kernel & Initrd
    print(f"Adding kernel {KERNEL_PATH}...")
    iso.add_file(KERNEL_PATH, '/BOOT/VMLINUZ;1', rr_name='vmlinuz', joliet_path='/boot/vmlinuz')
    print(f"Adding initramfs {INITRD_PATH}...")
    iso.add_file(INITRD_PATH, '/BOOT/INITRD.GZ;1', rr_name='initrd.gz', joliet_path='/boot/initrd.gz')
    
    # BIOS Boot files
    isolinux_bin = os.path.join(BOOT_DIR, "usr", "share", "syslinux", "isolinux.bin")
    ldlinux_c32 = os.path.join(BOOT_DIR, "usr", "share", "syslinux", "ldlinux.c32")
    menu_c32 = os.path.join(BOOT_DIR, "usr", "share", "syslinux", "menu.c32")
    libutil_c32 = os.path.join(BOOT_DIR, "usr", "share", "syslinux", "libutil.c32")
    isolinux_cfg = os.path.join(BOOT_DIR, "isolinux.cfg")
    grub_cfg = os.path.join(BOOT_DIR, "grub.cfg")
    
    iso.add_file(isolinux_bin, '/BOOT/ISOLINUX/ISOLINUX.BIN;1', rr_name='isolinux.bin', joliet_path='/boot/isolinux/isolinux.bin')
    iso.add_file(ldlinux_c32, '/BOOT/ISOLINUX/LDLINUX.C32;1', rr_name='ldlinux.c32', joliet_path='/boot/isolinux/ldlinux.c32')
    iso.add_file(menu_c32, '/BOOT/ISOLINUX/MENU.C32;1', rr_name='menu.c32', joliet_path='/boot/isolinux/menu.c32')
    iso.add_file(libutil_c32, '/BOOT/ISOLINUX/LIBUTIL.C32;1', rr_name='libutil.c32', joliet_path='/boot/isolinux/libutil.c32')
    iso.add_file(isolinux_cfg, '/BOOT/ISOLINUX/ISOLINUX.CFG;1', rr_name='isolinux.cfg', joliet_path='/boot/isolinux/isolinux.cfg')
    iso.add_file(grub_cfg, '/BOOT/GRUB/GRUB.CFG;1', rr_name='grub.cfg', joliet_path='/boot/grub/grub.cfg')
    
    # EFI Boot files
    iso.add_file(efi_img_path, '/BOOT/EFI.IMG;1', rr_name='efi.img', joliet_path='/boot/efi.img')
    iso.add_file(efi_bin, '/EFI/BOOT/BOOTX64.EFI;1', rr_name='bootx64.efi', joliet_path='/efi/boot/bootx64.efi')
    iso.add_file(isolinux_cfg, '/EFI/BOOT/SYSLINUX.CFG;1', rr_name='syslinux.cfg', joliet_path='/efi/boot/syslinux.cfg')
    iso.add_file(grub_cfg, '/EFI/BOOT/GRUB.CFG;1', rr_name='grub.cfg', joliet_path='/efi/boot/grub.cfg')
    
    # El Torito BIOS boot configuration
    print("Configuring El Torito BIOS boot catalog...")
    iso.add_eltorito('/BOOT/ISOLINUX/ISOLINUX.BIN;1', boot_info_table=True, boot_load_size=4)
    
    # El Torito UEFI boot configuration
    print("Configuring El Torito UEFI boot catalog...")
    iso.add_eltorito('/BOOT/EFI.IMG;1', efi=True, platform_id=0xef)
    
    print(f"Writing final ISO image to {ISO_OUTPUT}...")
    iso.write(ISO_OUTPUT)
    iso.close()
    
    # Verification
    iso_size = os.path.getsize(ISO_OUTPUT)
    iso_size_mb = iso_size / (1024 * 1024)
    print("-" * 50)
    print(f"MicroOS Final ISO Created: {ISO_OUTPUT}")
    print(f"Actual Size: {iso_size:,} bytes ({iso_size_mb:.2f} MB)")
    print(f"Maximum Allowed Budget: {MAX_ISO_BYTES:,} bytes (50.00 MB)")
    
    if iso_size > MAX_ISO_BYTES:
        raise ValueError(f"FATAL: ISO size {iso_size:,} exceeds hard limit of {MAX_ISO_BYTES:,} bytes!")
    headroom = MAX_ISO_BYTES - iso_size
    print(f"[COMPLIANT] Under budget by {headroom:,} bytes ({headroom / (1024*1024):.2f} MB headroom)")
    
    # SHA256 Checksum
    with open(ISO_OUTPUT, 'rb') as f:
        sha256 = hashlib.sha256(f.read()).hexdigest()
    with open(SHA256_OUTPUT, 'w') as f:
        f.write(f"{sha256}  microos.iso\n")
    print(f"SHA256: {sha256}")
    
    # Generate build-info.txt
    with open(BUILD_INFO_OUT, 'w') as f:
        f.write("MicroOS Build Information\n")
        f.write("=========================\n")
        f.write(f"OS Name: MicroOS\n")
        f.write(f"Version: 1.0.0\n")
        f.write(f"Architecture: x86_64\n")
        f.write(f"Kernel Version: 6.6.134-0-lts SMP\n")
        f.write(f"Bootloader: El Torito Dual Hybrid (BIOS ISOLINUX + UEFI 64-bit)\n")
        f.write(f"ISO File: microos.iso\n")
        f.write(f"ISO Size: {iso_size:,} bytes ({iso_size_mb:.2f} MB)\n")
        f.write(f"ISO SHA256: {sha256}\n")
        f.write(f"Budget Limit: {MAX_ISO_BYTES:,} bytes (50.00 MB)\n")
        f.write(f"Size Compliance: PASSED ({headroom:,} bytes headroom)\n")
        f.write(f"Target Hypervisor: Oracle VirtualBox 7.x (Offline, 1024MB RAM, 1 CPU)\n")
        f.write(f"Physical Target: Bare-metal Intel/AMD PC & Laptops\n")

    # Generate release-notes.md
    with open(RELEASE_NOTES_OUT, 'w') as f:
        f.write("# MicroOS v1.0.0 Release Notes\n\n")
        f.write("MicroOS is an ultra-lightweight, 64-bit x86_64 Linux operating system engineered for native bare-metal execution and Oracle VirtualBox virtualization within a strict 50 MB ISO budget.\n\n")
        f.write("## Key Release Features\n")
        f.write(f"- **Final ISO Size**: {iso_size:,} bytes ({iso_size_mb:.2f} MB) — well below the 50 MB ceiling.\n")
        f.write(f"- **Kernel**: Linux LTS 6.6.134 with Intel & AMD support, ACPI, APIC, NVMe, SATA, USB, ext4, and Wi-Fi drivers.\n")
        f.write("- **Bootloader**: Dual BIOS & UEFI boot support via El Torito boot catalog.\n")
        f.write("- **Root Filesystem**: BusyBox static userland, complete ash shell, and custom /init sequence.\n")
        f.write("- **Hardware Discovery**: Native sysfs/procfs inspection suite (`hardware cpu`, `memory`, `storage`, `network`, `usb`, `pci`).\n")
        f.write("- **Wi-Fi & Networking**: Linux networking CLI (`wifi status`, `scan`, `connect`, `disconnect`).\n")
        f.write("- **Storage Management**: Non-destructive disk inspection (`disks list`, `info`, `mount`, `unmount`).\n")
        f.write("- **Persistent Application Architecture**: Debian-compatible APT/dpkg layer decoupled onto persistent SSD media (`/mnt/ssd`).\n")
        f.write("- **Micro Arcade**: 8 built-in terminal arcade games (`snake`, `tetris`, `pong`, `mines`, `2048`, `guess`, `tictactoe`, `spaceinvaders`) and `arcade` launcher.\n\n")
        f.write("## File Verification\n")
        f.write(f"- **ISO**: `microos.iso`\n")
        f.write(f"- **SHA256**: `{sha256}`\n")

    print("ISO generation and release documentation complete!")

if __name__ == '__main__':
    main()
