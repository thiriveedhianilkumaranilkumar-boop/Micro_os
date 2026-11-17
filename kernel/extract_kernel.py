import zlib, lzma, os

with open(r'C:\Users\t_ani\.gemini\antigravity\scratch\MicroOS\kernel\vmlinuz', 'rb') as f:
    data = f.read()

print(f'Total vmlinuz size: {len(data)} bytes')

# Search for gzip (1f 8b 08), xz (fd 37 7a 58 5a 00), or zstd (28 b5 2f fd)
gz_idx = data.find(b'\x1f\x8b\x08')
xz_idx = data.find(b'\xfd7zXZ\x00')
zstd_idx = data.find(b'(\xb5/\xfd')

print(f'Offsets: gzip={gz_idx}, xz={xz_idx}, zstd={zstd_idx}')

decompressed = None
if zstd_idx != -1:
    print('Kernel payload uses zstd')
elif xz_idx != -1:
    print('Kernel payload uses xz')
    try:
        decompressed = lzma.decompress(data[xz_idx:])
    except Exception as e:
        print('XZ decompress error:', e)
elif gz_idx != -1:
    print('Kernel payload uses gzip')

if decompressed:
    print(f'Decompressed kernel size: {len(decompressed)} bytes')
    import re
    matches = re.findall(b'Linux version [^\x00\r\n]+', decompressed)
    for m in matches:
        print('Found banner:', m.decode('latin1', errors='ignore'))
