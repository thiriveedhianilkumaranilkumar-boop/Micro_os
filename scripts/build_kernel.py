#!/usr/bin/env python3
"""
MicroOS Kernel Provisioning and Configuration Verification Script
"""
import os, sys, hashlib

def main():
    kernel_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'kernel'))
    config_file = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'configs', 'kernel.config'))
    vmlinuz_file = os.path.join(kernel_dir, 'vmlinuz')
    
    if not os.path.exists(vmlinuz_file):
        print(f"Error: Kernel binary {vmlinuz_file} not found!", file=sys.stderr)
        sys.exit(1)
        
    size = os.path.getsize(vmlinuz_file)
    print(f"[OK] MicroOS Kernel binary found: {vmlinuz_file}")
    print(f"     Size: {size:,} bytes ({size / (1024*1024):.2f} MB)")
    
    with open(vmlinuz_file, 'rb') as f:
        sha256 = hashlib.sha256(f.read()).hexdigest()
    print(f"     SHA256: {sha256}")
    
    if not os.path.exists(config_file):
        print(f"Error: Kernel configuration {config_file} not found!", file=sys.stderr)
        sys.exit(1)
        
    with open(config_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    print(f"[OK] Kernel configuration verified: {len(lines)} configuration symbols.")

if __name__ == '__main__':
    main()
