# MicroOS Known Issues & Design Mitigations

This document records architectural boundaries, hardware detection limitations, and technical mitigations applied to guarantee the strict 50 MB ISO limit and zero-defect bootability.

## 1. 50 MB ISO Limit vs Debian APT Cache
- **Issue**: Standard Debian repositories with full `dpkg` and `apt` userspace bundles (including full libc, perl, coreutils, libapt) can easily exceed 150 MB, which would violate the hard 50 MB ISO ceiling.
- **Mitigation**: Decoupled architecture. The base ISO contains a static BusyBox userland, native networking stack, and kernel. The persistent APT environment is initialized and housed on dedicated persistent SSD / USB storage (`/mnt/ssd`). If persistent storage is not attached, MicroOS boots cleanly into RAM live mode and reports that APT operations require persistent media.

## 2. Wi-Fi Firmware Size Constraints
- **Issue**: The upstream `linux-firmware` repository is multiple gigabytes in size. Embedding all Wi-Fi firmwares would severely breach the 50 MB budget.
- **Mitigation**: Selective integration. Selected common Wi-Fi drivers and minimal firmware stubs are included for testing; additional vendor firmwares can be fetched dynamically or mounted from persistent media.

## 3. Emulated vs Physical Network Adapters
- **Issue**: In Oracle VirtualBox testing, network adapters appear as emulated Intel 82540EM (e1000) or virtio Ethernet devices, not physical 802.11 Wi-Fi.
- **Mitigation**: MicroOS hardware detection and `wifi` scripts explicitly check interface types via `/sys/class/net/*` and `/sys/class/net/*/wireless`, properly classifying wired Ethernet vs native wireless chipsets without fabrication.
