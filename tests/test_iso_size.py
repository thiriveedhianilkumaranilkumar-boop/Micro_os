#!/usr/bin/env python3
"""
Test ISO size budget compliance
Strictly enforces <= 50,000,000 bytes.
"""
import os, sys, hashlib

BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
ISO_PATH = os.path.join(BASE_DIR, "release", "microos.iso")
MAX_BYTES = 50_000_000

def main():
    print("=== MicroOS ISO Size Budget Verification ===")
    if not os.path.exists(ISO_PATH):
        print(f"[FAIL] ISO not found at {ISO_PATH}!", file=sys.stderr)
        sys.exit(1)
        
    actual_size = os.path.getsize(ISO_PATH)
    actual_mb = actual_size / (1024 * 1024)
    print(f"Target ISO File: {ISO_PATH}")
    print(f"Actual Size    : {actual_size:,} bytes ({actual_mb:.2f} MB)")
    print(f"Maximum Budget : {MAX_BYTES:,} bytes (50.00 MB)")
    
    with open(ISO_PATH, "rb") as f:
        sha256 = hashlib.sha256(f.read()).hexdigest()
    print(f"SHA256 Hash    : {sha256}")
    
    if actual_size > MAX_BYTES:
        overflow = actual_size - MAX_BYTES
        print(f"[FAIL] ISO exceeds budget by {overflow:,} bytes!", file=sys.stderr)
        sys.exit(1)
        
    headroom = MAX_BYTES - actual_size
    headroom_mb = headroom / (1024 * 1024)
    print(f"[PASS] Size is strictly compliant! Headroom: {headroom:,} bytes ({headroom_mb:.2f} MB)")

if __name__ == "__main__":
    main()
