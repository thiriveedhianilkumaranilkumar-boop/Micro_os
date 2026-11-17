#!/usr/bin/env python3
"""
MicroOS Arcade Compilation Script
Cross-compiles all 8 arcade games and the arcade launcher to static x86_64 Linux ELF binaries.
"""
import os, sys, subprocess

ZIG_BIN = r"C:\Users\t_ani\.gemini\antigravity\scratch\tools\zig-x86_64-windows-0.17.1\zig.exe"
BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
GAMES_DIR = os.path.join(BASE_DIR, "games")
ARCADE_DIR = os.path.join(BASE_DIR, "arcade")

GAMES = [
    ("snake.c", "snake", GAMES_DIR),
    ("tetris.c", "tetris", GAMES_DIR),
    ("pong.c", "pong", GAMES_DIR),
    ("mines.c", "mines", GAMES_DIR),
    ("2048.c", "2048", GAMES_DIR),
    ("guess.c", "guess", GAMES_DIR),
    ("tictactoe.c", "tictactoe", GAMES_DIR),
    ("spaceinvaders.c", "spaceinvaders", GAMES_DIR),
    ("arcade.c", "arcade", ARCADE_DIR)
]

def main():
    print("=== Compiling Micro Arcade Games (x86_64-linux-musl, static, stripped) ===")
    total_bytes = 0
    all_ok = True

    for src_name, bin_name, folder in GAMES:
        src_path = os.path.join(folder, src_name)
        bin_path = os.path.join(folder, bin_name)
        
        cmd = [
            ZIG_BIN, "cc",
            "-target", "x86_64-linux-musl",
            "-static",
            "-O3",
            "-s",
            src_path,
            "-o", bin_path
        ]
        
        print(f"Compiling {src_name} -> {bin_name}...")
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        if res.returncode != 0:
            print(f"Error compiling {src_name}:\n{res.stderr}", file=sys.stderr)
            all_ok = False
            continue
            
        size = os.path.getsize(bin_path)
        total_bytes += size
        print(f"  [OK] {bin_name}: {size:,} bytes")

    print("-" * 50)
    print(f"Total Arcade Suite Size: {total_bytes:,} bytes ({total_bytes / 1024:.1f} KB)")
    
    if not all_ok:
        sys.exit(1)
    print("All 8 Micro Arcade games + launcher successfully compiled!")

if __name__ == "__main__":
    main()
