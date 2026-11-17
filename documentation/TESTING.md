# MicroOS Testing Framework & Methodology

MicroOS mandates rigorous testing protocols combining virtualization validation and native hardware detection suites.

## Testing Standards

1. **Virtual Machine Testing Rule**:
   - Oracle VirtualBox ONLY (`VBoxManage` CLI).
   - QEMU is strictly prohibited by project specification.
   - Test VM Profile:
     - Memory: 1024 MB
     - CPU: 1 vCPU
     - Network: Disabled for offline baseline tests
     - Boot Media: microos.iso attached to IDE / SATA CD-ROM controller
     - Storage: Dynamic VDI virtual hard disk (4 GB) for partition & persistence validation

2. **Automated Testing Suite**:
   - `tests/test_iso_size.py`: Measures byte count of final ISO against the 50,000,000 byte ceiling.
   - `tests/test_vbox_boot.py`: Automates VirtualBox VM creation, ISO attachment, headless boot, serial console capture, and guest state validation.
   - `tests/test_games.py`: Verifies compilation, binary format (ELF 64-bit LSB), stripped status, and offline execution capabilities.
   - `tests/test_hardware_detection.py`: Validates syntax, sysfs queries, and output formatting of MicroOS hardware scripts.

3. **Physical Hardware Verification**:
   - When running on physical laptops, hardware inspection commands check real vendor IDs and device strings from sysfs.
