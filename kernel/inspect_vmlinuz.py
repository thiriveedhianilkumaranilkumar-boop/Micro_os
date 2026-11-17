import zlib, re

with open(r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\kernel\vmlinuz', 'rb') as f:
    data = f.read()

gz_data = data[21188:]
d = zlib.decompressobj(16 + zlib.MAX_WBITS)
decompressed = d.decompress(gz_data)
print(f'Decompressed kernel size: {len(decompressed):,} bytes')

banners = re.findall(b'Linux version [^\x00\r\n]+', decompressed)
for b in banners:
    print('Kernel banner:', b.decode('latin1', errors='ignore'))

# Check for IKCONFIG magic: "IKCFG_ST"
ik_idx = decompressed.find(b'IKCFG_ST')
print(f'Embedded config index: {ik_idx}')
if ik_idx != -1:
    cfg_data = decompressed[ik_idx+8:]
    try:
        cfg = zlib.decompress(cfg_data, 16 + zlib.MAX_WBITS)
        print('Extracted IKCONFIG! Length:', len(cfg))
        with open(r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\configs\kernel.config', 'wb') as cf:
            cf.write(cfg)
        print('Saved configs/kernel.config')
    except Exception as e:
        print('IKCONFIG decompress error:', e)
