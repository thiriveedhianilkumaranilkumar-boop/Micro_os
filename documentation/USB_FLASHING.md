# MicroOS USB Flashing & Boot Guide

MicroOS (`microos.iso`) is an ISO 9660 hybrid image engineered for direct boot from USB flash drives and external SSDs on standard x86_64 PC and laptop hardware (Intel and AMD).

## Integrity Verification

Always verify the SHA256 checksum prior to flashing:

### Windows (PowerShell)
```powershell
Get-FileHash -Algorithm SHA256 release\microos.iso
```
Expected SHA256 Hash:
```
51ce4c11efb1a6350e0f326580cde3d62a9900a5292f38aca4353c2e708ea72a
```

### Linux / macOS
```bash
sha256sum release/microos.iso
```

---

## Flashing Instructions

### Method 1: Rufus (Windows)
1. Download and launch **Rufus** (v4.x or later).
2. Insert your target USB flash drive (minimum 64 MB capacity).
3. Under **Device**, select your USB drive.
4. Under **Boot selection**, click **SELECT** and choose `microos.iso`.
5. Under **Partition scheme**:
   - For modern UEFI laptops: Choose **GPT** and **Target system: UEFI (non CSM)**.
   - For older legacy BIOS laptops: Choose **MBR** and **Target system: BIOS or UEFI**.
6. When prompted between *ISO Image mode* or *DD Image mode*, select **ISO Image mode** (or **DD mode** if preferred).
7. Click **START**. Once complete, the USB drive is ready for boot.

### Method 2: balenaEtcher (Windows / Linux / macOS)
1. Download and run **balenaEtcher**.
2. Click **Flash from file** and select `microos.iso`.
3. Click **Select target** and select your USB flash drive.
4. Click **Flash!** and wait for the verification pass.

### Method 3: Ventoy (Multi-Boot USB)
1. Install **Ventoy** onto your USB drive following the official Ventoy installer.
2. Simply copy `microos.iso` directly into the root or an `ISO/` folder of the Ventoy USB drive partition.
3. Boot your laptop, select Ventoy in your BIOS/UEFI boot menu, and select `microos.iso`.
4. Choose **Boot in normal mode**.

### Method 4: Linux `dd` CLI
```bash
sudo dd if=release/microos.iso of=/dev/sdX bs=4M status=progress conv=fdatasync
```
*(Replace `/dev/sdX` with your actual USB block device; never specify internal partitions like `/dev/nvme0n1` or `/dev/sda1`)*.

---

## Laptop Boot Instructions
1. Insert the prepared USB flash drive into your physical laptop.
2. Power on the machine and immediately press the Boot Device Selector hotkey:
   - Lenovo: `F12` or `Novo Button`
   - Dell: `F12`
   - HP: `F9` or `Esc`
   - ASUS: `F8` or `Esc`
   - Acer: `F12`
3. Select your USB drive from the boot menu.
4. The MicroOS boot splash will appear:
   ```
   ====================================================
   MICROOS Minimal Linux. Maximum control.
   ====================================================
   ```
5. You will arrive at the interactive prompt:
   `micro@micro:~$ `
6. Run `hardware` to verify your physical CPU, RAM, Wi-Fi, and storage controllers!
