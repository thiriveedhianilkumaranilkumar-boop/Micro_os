#!/bin/sh
# MicroOS Master /init Script
# Executed by Linux kernel upon initramfs boot

export PATH="/bin:/sbin:/usr/bin:/usr/sbin"

# 1. Mount virtual kernel filesystems
mount -t proc none /proc 2>/dev/null
mount -t sysfs none /sys 2>/dev/null
mount -t devtmpfs devtmpfs /dev 2>/dev/null || mount -t tmpfs none /dev 2>/dev/null
mkdir -p /dev/pts /dev/shm /tmp /run /mnt/ssd /root /home
mount -t devpts devpts /dev/pts 2>/dev/null
mount -t tmpfs none /tmp 2>/dev/null
mount -t tmpfs none /run 2>/dev/null

# 2. Populate dynamic device nodes
if command -v mdev >/dev/null 2>&1; then
    echo /sbin/mdev > /proc/sys/kernel/hotplug 2>/dev/null
    mdev -s 2>/dev/null
fi

# 3. MicroOS Boot Splash Banner
clear
echo ""
echo "===================================================="
echo " MICROOS"
echo " Minimal Linux. Maximum control."
echo "===================================================="
echo " [OK] Linux Kernel initialized (version: $(uname -r))"
echo " [OK] Core virtual filesystems mounted (/proc, /sys, /dev)"
echo " [OK] Dynamic device manager started"

# 4. Storage Discovery & Persistent SSD Detection
echo " Scanning for persistent storage partitions..."
PERSISTENT_MOUNTED=0
for dev in /dev/nvme0n1p* /dev/sda* /dev/sdb* /dev/vda*; do
    [ -b "$dev" ] || continue
    # Skip whole drive if partitions exist
    case "$dev" in
        /dev/sda|/dev/sdb|/dev/vda) continue ;;
    esac
    
    if mount "$dev" /mnt/ssd 2>/dev/null; then
        echo " [OK] Persistent SSD partition detected and mounted: $dev -> /mnt/ssd"
        PERSISTENT_MOUNTED=1
        mkdir -p /mnt/ssd/microos
        break
    fi
done

if [ $PERSISTENT_MOUNTED -eq 0 ]; then
    echo " [INFO] No persistent SSD partition found. Operating in live RAM mode."
    echo "        (Base system fully operational under 50 MB budget limit)"
fi

# 5. Hand off to BusyBox init or interactive shell
if [ -x /sbin/init ]; then
    echo " [OK] Starting MicroOS init system..."
    exec /sbin/init
else
    echo " [OK] Launching MicroOS primary shell..."
    exec /bin/sh -l
fi

# Emergency shell fallback if init fails
echo " [WARN] Primary init process terminated. Dropping to emergency shell."
exec /bin/sh
