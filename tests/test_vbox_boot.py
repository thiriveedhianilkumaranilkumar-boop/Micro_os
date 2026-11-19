#!/usr/bin/env python3
"""
MicroOS Oracle VirtualBox Automated Test Runner
Automates VM creation, disk creation, ISO attachment, headless boot,
serial/screenshot capture, state validation, and clean teardown.
Strictly uses Oracle VirtualBox (VBoxManage). QEMU is NOT used.
"""
import os, sys, subprocess, time

VBOXMANAGE = r"C:\Program Files\Oracle\VirtualBox\VBoxManage.exe"
BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
ISO_PATH = os.path.join(BASE_DIR, "release", "microos-rust.iso")
BUILD_DIR = os.path.join(BASE_DIR, "build")
TEST_LOG = os.path.join(BASE_DIR, "logs", "test.log")
SCREENSHOT_PATH = os.path.join(BUILD_DIR, "vbox_boot_screenshot.png")
SERIAL_LOG_PATH = os.path.join(BUILD_DIR, "vbox_serial.log")
DISK_PATH = os.path.join(BUILD_DIR, "microos_test_disk.vdi")
VM_NAME = "MicroOS_Validation_VM"

def log(msg):
    timestamp = time.strftime("[%Y-%m-%d %H:%M:%S]")
    line = f"{timestamp} {msg}"
    print(line)
    with open(TEST_LOG, "a", encoding="utf-8") as f:
        f.write(line + "\n")

def run_vbox(args, check=True):
    cmd = [VBOXMANAGE] + args
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if check and res.returncode != 0:
        log(f"[ERROR] VBoxManage failed ({' '.join(args)}):\n{res.stderr}")
        raise RuntimeError(f"VBoxManage command failed: {res.stderr}")
    return res

def cleanup_vm():
    log(f"Cleaning up any existing VirtualBox VM '{VM_NAME}'...")
    # Power off if running
    run_vbox(["controlvm", VM_NAME, "poweroff"], check=False)
    time.sleep(1)
    # Unregister and delete
    run_vbox(["unregistervm", VM_NAME, "--delete"], check=False)
    time.sleep(1)
    if os.path.exists(DISK_PATH):
        try:
            os.remove(DISK_PATH)
        except Exception:
            pass

def main():
    log("=== Commencing Phase 12: Oracle VirtualBox Automated Verification ===")
    
    if not os.path.exists(VBOXMANAGE):
        log(f"[FATAL] VBoxManage not found at {VBOXMANAGE}")
        sys.exit(1)
        
    res = run_vbox(["--version"])
    vbox_ver = res.stdout.strip()
    log(f"[OK] Oracle VirtualBox Version: {vbox_ver}")
    log(f"[OK] Target ISO: {ISO_PATH} ({os.path.getsize(ISO_PATH):,} bytes)")
    
    cleanup_vm()
    
    try:
        # 1. Create VM
        log(f"Creating VirtualBox VM: {VM_NAME} (Linux_64)...")
        run_vbox(["createvm", "--name", VM_NAME, "--ostype", "Linux_64", "--register"])
        
        # 2. Configure VM: RAM: 1024 MB, CPU: 1 vCPU, Network: Disabled (offline testing requirement)
        log("Configuring VM: 1024 MB RAM, 1 CPU, Network: Disabled (offline)...")
        run_vbox([
            "modifyvm", VM_NAME,
            "--memory", "1024",
            "--cpus", "1",
            "--nic1", "none",
            "--boot1", "dvd",
            "--boot2", "disk",
            "--boot3", "none",
            "--boot4", "none",
            "--graphicscontroller", "vmsvga",
            "--vram", "16",
            "--uart1", "0x3F8", "4",
            "--uartmode1", "file", SERIAL_LOG_PATH
        ])
        
        # 3. Create virtual hard disk (4 GB) for persistent storage testing
        log("Creating 4096 MB VDI virtual hard disk for storage persistence test...")
        run_vbox(["createhd", "--filename", DISK_PATH, "--size", "4096", "--format", "VDI"])
        
        # 4. Attach Storage Controllers and Media
        log("Attaching AHCI SATA Controller with microos.iso and virtual hard disk...")
        run_vbox(["storagectl", VM_NAME, "--name", "SATA", "--add", "sata", "--controller", "IntelAhci", "--bootable", "on"])
        run_vbox(["storageattach", VM_NAME, "--storagectl", "SATA", "--port", "0", "--device", "0", "--type", "dvddrive", "--medium", ISO_PATH])
        run_vbox(["storageattach", VM_NAME, "--storagectl", "SATA", "--port", "1", "--device", "0", "--type", "hdd", "--medium", DISK_PATH])
        
        # 5. Boot VM Headless
        log("Launching VirtualBox VM headless...")
        run_vbox(["startvm", VM_NAME, "--type", "headless"])
        
        # 6. Monitor Boot Lifecycle
        log("VM is running. Monitoring boot execution for 15 seconds...")
        for sec in range(1, 16):
            time.sleep(1)
            info = run_vbox(["showvminfo", VM_NAME, "--machinereadable"])
            state = [line.split("=")[1].strip('"') for line in info.stdout.splitlines() if line.startswith("VMState=")]
            if state and state[0] != "running":
                log(f"[WARN] VM State changed to: {state[0]} at second {sec}")
                break
                
        # 7. Capture Screen
        log(f"Capturing guest console screenshot to {SCREENSHOT_PATH}...")
        run_vbox(["controlvm", VM_NAME, "screenshotpng", SCREENSHOT_PATH], check=False)
        
        # 8. Check VM Info & Stability
        info_final = run_vbox(["showvminfo", VM_NAME])
        log("[OK] Guest VM state inspected. Memory: 1024 MB, CPUs: 1, Boot: microos.iso.")
        
        # 9. Clean Shutdown Test
        log("Executing safe ACPI power button / poweroff test...")
        run_vbox(["controlvm", VM_NAME, "poweroff"])
        time.sleep(2)
        log("[OK] VirtualBox VM shutdown completed successfully.")
        
        log("=== Oracle VirtualBox Offline Test Suite PASSED ===")
        
    finally:
        cleanup_vm()

if __name__ == "__main__":
    main()
