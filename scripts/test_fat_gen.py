import struct, os

def create_fat12_efi(img_path, efi_bin_path):
    # 1.44MB floppy geometry: 2880 sectors of 512 bytes = 1,474,560 bytes
    total_sectors = 2880
    bytes_per_sector = 512
    sectors_per_cluster = 1
    reserved_sectors = 1
    fat_count = 2
    root_entries = 224
    sectors_per_fat = 9
    
    with open(efi_bin_path, 'rb') as f:
        efi_data = f.read()

    # Pre-allocate image with zeros
    img = bytearray(total_sectors * bytes_per_sector)
    
    # 1. BPB / Boot sector
    boot_sec = bytearray(512)
    boot_sec[0:3] = b'\xeb\x3c\x90' # jmp short 3c
    boot_sec[3:11] = b'MSDOS5.0'     # OEM Name
    struct.pack_into('<H', boot_sec, 11, bytes_per_sector)
    boot_sec[13] = sectors_per_cluster
    struct.pack_into('<H', boot_sec, 14, reserved_sectors)
    boot_sec[16] = fat_count
    struct.pack_into('<H', boot_sec, 17, root_entries)
    struct.pack_into('<H', boot_sec, 19, total_sectors)
    boot_sec[21] = 0xf0             # Media descriptor (1.44MB)
    struct.pack_into('<H', boot_sec, 22, sectors_per_fat)
    struct.pack_into('<H', boot_sec, 24, 18) # Sectors per track
    struct.pack_into('<H', boot_sec, 26, 2)  # Number of heads
    boot_sec[38] = 0x29             # Extended boot signature
    struct.pack_into('<I', boot_sec, 39, 0x12345678) # Volume ID
    boot_sec[43:54] = b'MICROOS_EFI'
    boot_sec[54:62] = b'FAT12   '
    boot_sec[510:512] = b'\x55\xaa' # MBR signature
    img[0:512] = boot_sec
    
    # Calculate offsets
    fat1_offset = reserved_sectors * bytes_per_sector
    fat2_offset = fat1_offset + sectors_per_fat * bytes_per_sector
    root_offset = fat2_offset + sectors_per_fat * bytes_per_sector
    root_sectors = (root_entries * 32 + bytes_per_sector - 1) // bytes_per_sector
    data_offset = root_offset + root_sectors * bytes_per_sector
    
    # Initialize FATs (first 3 bytes: F0 FF FF)
    img[fat1_offset:fat1_offset+3] = b'\xf0\xff\xff'
    img[fat2_offset:fat2_offset+3] = b'\xf0\xff\xff'
    
    def set_fat12(fat_buf, cluster, value):
        byte_offset = (cluster * 3) // 2
        if cluster % 2 == 0:
            fat_buf[byte_offset] = value & 0xff
            fat_buf[byte_offset + 1] = (fat_buf[byte_offset + 1] & 0xf0) | ((value >> 8) & 0x0f)
        else:
            fat_buf[byte_offset] = (fat_buf[byte_offset] & 0x0f) | ((value << 4) & 0xf0)
            fat_buf[byte_offset + 1] = (value >> 4) & 0xff

    # Subdirectories: EFI -> BOOT -> BOOTX64.EFI
    # Cluster 2: EFI directory table
    # Cluster 3: BOOT directory table
    # Cluster 4+: BOOTX64.EFI file data
    
    fat1_view = memoryview(img)[fat1_offset:fat1_offset + sectors_per_fat * bytes_per_sector]
    set_fat12(fat1_view, 2, 0xfff) # EOF for cluster 2
    set_fat12(fat1_view, 3, 0xfff) # EOF for cluster 3
    
    # Add EFI entry in Root Dir
    root_view = memoryview(img)[root_offset:root_offset + root_sectors * bytes_per_sector]
    # Root Dir Entry 0: "EFI        ", attr=0x10 (DIR), cluster=2
    root_entry = bytearray(32)
    root_entry[0:11] = b'EFI        '
    root_entry[11] = 0x10 # Directory
    struct.pack_into('<H', root_entry, 26, 2) # First cluster = 2
    root_view[0:32] = root_entry
    
    # Cluster 2 (EFI dir): contains "." (cluster 2), ".." (cluster 0), and "BOOT       " (cluster 3)
    c2_offset = data_offset + (2 - 2) * bytes_per_sector * sectors_per_cluster
    dot = bytearray(32); dot[0:11] = b'.          '; dot[11] = 0x10; struct.pack_into('<H', dot, 26, 2)
    dotdot = bytearray(32); dotdot[0:11] = b'..         '; dotdot[11] = 0x10; struct.pack_into('<H', dotdot, 26, 0)
    boot_dir = bytearray(32); boot_dir[0:11] = b'BOOT       '; boot_dir[11] = 0x10; struct.pack_into('<H', boot_dir, 26, 3)
    img[c2_offset:c2_offset+32] = dot
    img[c2_offset+32:c2_offset+64] = dotdot
    img[c2_offset+64:c2_offset+96] = boot_dir
    
    # Cluster 3 (BOOT dir): contains "." (cluster 3), ".." (cluster 2), and "BOOTX64 EFI"
    c3_offset = data_offset + (3 - 2) * bytes_per_sector * sectors_per_cluster
    dot3 = bytearray(32); dot3[0:11] = b'.          '; dot3[11] = 0x10; struct.pack_into('<H', dot3, 26, 3)
    dotdot3 = bytearray(32); dotdot3[0:11] = b'..         '; dotdot3[11] = 0x10; struct.pack_into('<H', dotdot3, 26, 2)
    file_entry = bytearray(32); file_entry[0:11] = b'BOOTX64 EFI'; file_entry[11] = 0x20 # Archive
    struct.pack_into('<H', file_entry, 26, 4) # First cluster = 4
    struct.pack_into('<I', file_entry, 28, len(efi_data))
    img[c3_offset:c3_offset+32] = dot3
    img[c3_offset+32:c3_offset+64] = dotdot3
    img[c3_offset+64:c3_offset+96] = file_entry
    
    # Write BOOTX64.EFI file data starting at cluster 4
    cluster_bytes = bytes_per_sector * sectors_per_cluster
    num_clusters = (len(efi_data) + cluster_bytes - 1) // cluster_bytes
    for i in range(num_clusters):
        cur_clust = 4 + i
        next_clust = 0xfff if i == num_clusters - 1 else cur_clust + 1
        set_fat12(fat1_view, cur_clust, next_clust)
        
        c_off = data_offset + (cur_clust - 2) * cluster_bytes
        chunk = efi_data[i * cluster_bytes : (i + 1) * cluster_bytes]
        img[c_off:c_off+len(chunk)] = chunk
        
    # Copy FAT1 to FAT2
    img[fat2_offset:fat2_offset + sectors_per_fat * bytes_per_sector] = img[fat1_offset:fat1_offset + sectors_per_fat * bytes_per_sector]
    
    with open(img_path, 'wb') as f:
        f.write(img)
    print(f'Created valid FAT12 EFI image: {img_path} ({len(img):,} bytes)')

efi_bin = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\boot\usr\share\syslinux\efi64\syslinux.efi'
out_img = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\build\efi.img'
create_fat12_efi(out_img, efi_bin)
