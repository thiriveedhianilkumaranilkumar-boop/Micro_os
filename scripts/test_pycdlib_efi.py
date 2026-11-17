import pycdlib, os

test_iso = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\build\test_efi.iso'
if os.path.exists(test_iso): os.remove(test_iso)

iso = pycdlib.PyCdlib()
iso.new(interchange_level=3, rock_ridge='1.09', vol_ident='MICROOS')

iso.add_directory('/BOOT', rr_name='boot')
iso.add_directory('/BOOT/ISOLINUX', rr_name='isolinux')

isolinux_bin = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\boot\usr\share\syslinux\isolinux.bin'
iso.add_file(isolinux_bin, '/BOOT/ISOLINUX/ISOLINUX.BIN;1', rr_name='isolinux.bin')

# Primary BIOS boot
iso.add_eltorito('/BOOT/ISOLINUX/ISOLINUX.BIN;1', boot_info_table=True, boot_load_size=4)

# EFI boot image
efi_img = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\build\efi.img'
iso.add_file(efi_img, '/BOOT/EFI.IMG;1', rr_name='efi.img')

# Try adding secondary EFI eltorito
try:
    iso.add_eltorito('/BOOT/EFI.IMG;1', efi=True, platform_id=0xef)
    print('Secondary EFI eltorito added!')
except Exception as e:
    print('Secondary eltorito exception:', e)

iso.write(test_iso)
iso.close()
print('ISO written successfully:', os.path.getsize(test_iso))
