#!/bin/sh
# MicroOS Storage & Disk Management Engine
# Non-destructive detection, partition inspection, and safe mounting

show_help() {
    echo "===================================================="
    echo " MicroOS Storage & SSD Management CLI"
    echo "===================================================="
    echo "Usage: disks [command] [args...]"
    echo ""
    echo "Commands:"
    echo "  list                  List all block devices and partitions"
    echo "  info [dev]            Detailed drive and filesystem inspection"
    echo "  mount <dev> [dir]     Safely mount storage partition"
    echo "  unmount <dev|dir>     Safely unmount partition"
    echo "  mounts                List all currently mounted filesystems"
    echo "===================================================="
}

cmd_list() {
    echo "------------------ DETECTED DISKS ------------------"
    printf "%-12s %-10s %-16s %-16s\n" "DEVICE" "SIZE" "TYPE" "MODEL"
    printf "%-12s %-10s %-16s %-16s\n" "------" "----" "----" "-----"
    
    for b in /sys/block/*; do
        [ -e "$b" ] || continue
        dev=$(basename "$b")
        case "$dev" in
            loop*|ram*|zram*) continue ;;
        esac
        
        sectors=$(cat "$b/size" 2>/dev/null || echo 0)
        size_mb=$((sectors * 512 / 1024 / 1024))
        
        type="Block Device"
        case "$dev" in
            nvme*) type="NVMe SSD" ;;
            sd*)   type="SATA/USB Drive" ;;
            sr*)   type="CD/DVD-ROM" ;;
            vd*)   type="VirtIO Disk" ;;
        esac
        
        model=$(cat "$b/device/model" 2>/dev/null | sed 's/^[ \t]*//')
        [ -z "$model" ] && model=$(cat "$b/device/name" 2>/dev/null)
        [ -z "$model" ] && model="Standard"
        
        printf "%-12s %-10s %-16s %-16s\n" "/dev/$dev" "${size_mb} MB" "$type" "$model"
        
        # Check partitions
        for p in /sys/block/$dev/${dev}*; do
            if [ -d "$p" ] && [ "$p" != "$b" ]; then
                pname=$(basename "$p")
                psectors=$(cat "$p/size" 2>/dev/null || echo 0)
                pmb=$((psectors * 512 / 1024 / 1024))
                
                # Check mount
                mnt=$(grep -w "/dev/$pname" /proc/mounts | awk '{print $2}')
                [ -z "$mnt" ] && mnt="[unmounted]"
                
                printf "  |- %-10s %-8s %-14s %s\n" "/dev/$pname" "${pmb} MB" "Partition" "$mnt"
            fi
        done
    done
}

cmd_info() {
    dev="$1"
    if [ -z "$dev" ]; then
        cmd_list
        return 0
    fi
    
    bname=$(basename "$dev")
    echo "--------------- DRIVE DETAILS: /dev/$bname --------------"
    if [ -f "/sys/class/block/$bname/size" ]; then
        sectors=$(cat "/sys/class/block/$bname/size")
        bytes=$((sectors * 512))
        mb=$((bytes / 1024 / 1024))
        echo "Capacity       : $mb MB ($bytes bytes)"
        echo "Total Sectors  : $sectors (512-byte logical)"
        
        if command -v blkid >/dev/null 2>&1; then
            echo "Filesystem Info:"
            blkid "/dev/$bname" 2>/dev/null | awk '{print "  " $0}'
        fi
        
        mnt=$(grep -w "/dev/$bname" /proc/mounts | head -n1)
        if [ -n "$mnt" ]; then
            echo "Mount Status   : MOUNTED"
            echo "Mount Point    : $(echo "$mnt" | awk '{print $2}')"
            echo "Mount Type     : $(echo "$mnt" | awk '{print $3}')"
            echo "Mount Options  : $(echo "$mnt" | awk '{print $4}')"
        else
            echo "Mount Status   : UNMOUNTED"
        fi
    else
        echo "Error: Device /dev/$bname not found in sysfs."
    fi
}

cmd_mount() {
    dev="$1"
    target="$2"
    
    if [ -z "$dev" ]; then
        echo "Error: Device argument required. Usage: disks mount <dev> [target_dir]"
        return 1
    fi
    
    [ -e "$dev" ] || dev="/dev/$dev"
    if [ ! -b "$dev" ]; then
        echo "Error: $dev is not a valid block device."
        return 1
    fi
    
    if [ -z "$target" ]; then
        target="/mnt/ssd"
    fi
    
    mkdir -p "$target"
    
    echo "Attempting safe mount of $dev -> $target..."
    if mount "$dev" "$target" 2>/dev/null; then
        echo "Mounted successfully read-write: $dev on $target"
    elif mount -o ro "$dev" "$target" 2>/dev/null; then
        echo "Mounted successfully read-only (safe fallback): $dev on $target"
    else
        echo "Error: Mount failed. Unsupported filesystem or partition error."
        return 1
    fi
}

cmd_unmount() {
    target="$1"
    if [ -z "$target" ]; then
        echo "Error: Target or device required. Usage: disks unmount <dev|target_dir>"
        return 1
    fi
    
    echo "Syncing disk buffers..."
    sync
    echo "Unmounting $target..."
    if umount "$target" 2>/dev/null; then
        echo "Successfully unmounted $target"
    else
        echo "Error: Failed to unmount $target (device busy?)."
        return 1
    fi
}

cmd_mounts() {
    echo "---------------- ACTIVE MOUNTPOINTS -----------------"
    grep -E '^/dev/' /proc/mounts | awk '{printf "%-16s on %-16s type %-8s (%s)\n", $1, $2, $3, $4}'
}

case "$1" in
    list|"")        cmd_list ;;
    info)           shift; cmd_info "$@" ;;
    mount)          shift; cmd_mount "$@" ;;
    unmount|umount) shift; cmd_unmount "$@" ;;
    mounts)         cmd_mounts ;;
    help|--help|-h) show_help ;;
    *)
        echo "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
