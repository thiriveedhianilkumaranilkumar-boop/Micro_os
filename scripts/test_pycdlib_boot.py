import pycdlib, os

test_iso = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\build\test.iso'
if os.path.exists(test_iso): os.remove(test_iso)

iso = pycdlib.PyCdlib()
iso.new(interchange_level=3, rock_ridge='1.09', vol_ident='MICROOS')

# Add boot directory
iso.add_directory('/BOOT', rr_name='boot')
iso.add_directory('/BOOT/ISOLINUX', rr_name='isolinux')

# Add isolinux.bin
isolinux_bin = r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\boot\usr\share\syslinux\isolinux.bin'
iso.add_file(isolinux_bin, '/BOOT/ISOLINUX/ISOLINUX.BIN;1', rr_name='isolinux.bin')

# Add eltorito with boot_info_table
iso.add_eltorito('/BOOT/ISOLINUX/ISOLINUX.BIN;1', boot_info_table=True, boot_load_size=4)

iso.write(test_iso)
iso.close()

print('Generated test ISO successfully! Size:', os.path.getsize(test_iso), 'bytes')
