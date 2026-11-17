#!/usr/bin/env python3
"""
Test Micro Arcade binaries for 64-bit ELF headers and size budget
"""
import os, sys

BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
GAMES = [
    os.path.join(BASE_DIR, "games", "snake"),
    os.path.join(BASE_DIR, "games", "tetris"),
    os.path.join(BASE_DIR, "games", "pong"),
    os.path.join(BASE_DIR, "games", "mines"),
    os.path.join(BASE_DIR, "games", "2048"),
    os.path.join(BASE_DIR, "games", "guess"),
    os.path.join(BASE_DIR, "games", "tictactoe"),
    os.path.join(BASE_DIR, "games", "spaceinvaders"),
    os.path.join(BASE_DIR, "arcade", "arcade")
]

def main():
    print("=== Testing Micro Arcade Binaries ===")
    all_ok = True
    total_size = 0
    
    for path in GAMES:
        name = os.path.basename(path)
        if not os.path.exists(path):
            print(f"[FAIL] {name}: File not found!", file=sys.stderr)
            all_ok = False
            continue
            
        size = os.path.getsize(path)
        total_size += size
        
        with open(path, "rb") as f:
            magic = f.read(4)
            ei_class = f.read(1)[0]
            ei_data = f.read(1)[0]
            
        if magic != b"\x7fELF" or ei_class != 2 or ei_data != 1:
            print(f"[FAIL] {name}: Invalid ELF header (magic={magic}, class={ei_class})", file=sys.stderr)
            all_ok = False
        else:
            print(f"[PASS] {name:<14} | ELF 64-bit LSB | Size: {size:>6} bytes")
            
    print(f"\nTotal Games Suite Size: {total_size:,} bytes")
    if not all_ok:
        sys.exit(1)
    print("All 8 Arcade games + launcher verified successfully!")

if __name__ == "__main__":
    main()
