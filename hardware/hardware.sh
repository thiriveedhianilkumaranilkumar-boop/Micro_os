#!/bin/sh
# MicroOS Hardware Discovery Engine
# Reads real hardware data from /proc and /sys

show_help() {
    echo "===================================================="
    echo " MicroOS Hardware Detection System"
    echo "===================================================="
    echo "Usage: hardware [subcommand]"
    echo ""
    echo "Subcommands:"
    echo "  cpu       Display CPU model, cores, frequency, flags"
    echo "  memory    Display total, free, and available RAM"
    echo "  network   Display wired and wireless network adapters"
    echo "  storage   Display NVMe, SATA, and block devices"
    echo "  usb       Display USB controllers and connected devices"
    echo "  pci       Display PCI and PCIe devices"
    echo "  all       Display full comprehensive hardware report"
    echo "===================================================="
}

detect_cpu() {
    echo "------------------ CPU INFORMATION ------------------"
    if [ -f /proc/cpuinfo ]; then
        model=$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^[ \t]*//')
        cores=$(grep -c '^processor' /proc/cpuinfo)
        arch=$(uname -m)
        mhz=$(grep -m1 'cpu MHz' /proc/cpuinfo | cut -d: -f2 | sed 's/^[ \t]*//')
        echo "Architecture : $arch"
        echo "Model Name   : $model"
        echo "Cores/vCPUs  : $cores"
        [ -n "$mhz" ] && echo "Clock Speed  : $mhz MHz"
        flags=$(grep -m1 'flags' /proc/cpuinfo | cut -d: -f2 | sed 's/^[ \t]*//')
        features=""
        for f in lm vmx svm aes sse4_2 avx avx2; do
            case " $flags " in *" $f "*) features="$features $f";; esac
        done
        echo "Features     :$features"
    else
        echo "Error: /proc/cpuinfo not accessible."
    fi
}

detect_memory() {
    echo "----------------- MEMORY INFORMATION ----------------"
    if [ -f /proc/meminfo ]; then
        total=$(grep -m1 'MemTotal:' /proc/meminfo | awk '{printf "%.2f MB (%d kB)", $2/1024, $2}')
        free=$(grep -m1 'MemFree:' /proc/meminfo | awk '{printf "%.2f MB (%d kB)", $2/1024, $2}')
        avail=$(grep -m1 'MemAvailable:' /proc/meminfo | awk '{printf "%.2f MB (%d kB)", $2/1024, $2}')
        buffers=$(grep -m1 'Buffers:' /proc/meminfo | awk '{printf "%.2f MB", $2/1024}')
        cached=$(grep -m1 '^Cached:' /proc/meminfo | awk '{printf "%.2f MB", $2/1024}')
        echo "Total RAM     : $total"
        echo "Free RAM      : $free"
        echo "Available RAM : $avail"
        echo "Buffers/Cache : $buffers / $cached"
    else
        echo "Error: /proc/meminfo not accessible."
    fi
}

detect_network() {
    echo "---------------- NETWORK ADAPTERS ------------------"
    if [ -d /sys/class/net ]; then
        for iface in /sys/class/net/*; do
            [ -e "$iface" ] || continue
            dev=$(basename "$iface")
            [ "$dev" = "lo" ] && continue
            
            type="Ethernet"
            if [ -d "$iface/wireless" ] || [ -d "$iface/phy80211" ]; then
                type="Wi-Fi (802.11)"
            fi
            
            mac=""
            [ -f "$iface/address" ] && mac=$(cat "$iface/address")
            state="DOWN"
            [ -f "$iface/operstate" ] && state=$(cat "$iface/operstate")
            
            driver=""
            if [ -L "$iface/device/driver" ]; then
                driver=$(basename "$(readlink "$iface/device/driver")")
            fi
            
            echo "Device: $dev"
            echo "  Type        : $type"
            echo "  State       : $state"
            [ -n "$mac" ] && echo "  MAC Address : $mac"
            [ -n "$driver" ] && echo "  Driver      : $driver"
        done
    else
        echo "No network interfaces found."
    fi
}

detect_storage() {
    echo "----------------- STORAGE DEVICES ------------------"
    if [ -d /sys/block ]; then
        found=0
        for b in /sys/block/*; do
            [ -e "$b" ] || continue
            dev=$(basename "$b")
            case "$dev" in
                loop*|ram*|sr*|zram*) continue ;;
            esac
            found=1
            size_sectors=$(cat "$b/size" 2>/dev/null || echo 0)
            size_mb=$((size_sectors * 512 / 1024 / 1024))
            
            bus="Unknown"
            if [ -d "$b/device" ]; then
                dev_path=$(readlink -f "$b/device" 2>/dev/null || echo "")
                case "$dev_path" in
                    *nvme*) bus="NVMe PCIe" ;;
                    *ata*|*scsi*|*ahci*) bus="SATA / AHCI" ;;
                    *usb*) bus="USB Mass Storage" ;;
                    *virtio*) bus="VirtIO" ;;
                esac
            fi
            
            model=$(cat "$b/device/model" 2>/dev/null | sed 's/^[ \t]*//')
            [ -z "$model" ] && model=$(cat "$b/device/name" 2>/dev/null)
            [ -z "$model" ] && model="Standard Block Device"
            
            echo "Drive: /dev/$dev"
            echo "  Model       : $model"
            echo "  Bus/Type    : $bus"
            echo "  Capacity    : $size_mb MB ($((size_mb/1024)) GB)"
            
            # List partitions
            for part in /sys/block/$dev/${dev}*; do
                if [ -d "$part" ] && [ "$part" != "$b" ]; then
                    pname=$(basename "$part")
                    psize=$(cat "$part/size" 2>/dev/null || echo 0)
                    pmb=$((psize * 512 / 1024 / 1024))
                    echo "    Partition : /dev/$pname ($pmb MB)"
                fi
            done
        done
        [ $found -eq 0 ] && echo "No physical storage drives detected."
    else
        echo "Error: /sys/block not accessible."
    fi
}

detect_usb() {
    echo "------------------- USB DEVICES --------------------"
    if [ -d /sys/bus/usb/devices ]; then
        found=0
        for dev in /sys/bus/usb/devices/*; do
            [ -e "$dev" ] || continue
            if [ -f "$dev/idVendor" ]; then
                found=1
                vendor=$(cat "$dev/idVendor" 2>/dev/null)
                product=$(cat "$dev/idProduct" 2>/dev/null)
                mfr=$(cat "$dev/manufacturer" 2>/dev/null)
                prod=$(cat "$dev/product" 2>/dev/null)
                bname=$(basename "$dev")
                echo "USB Device [$bname] ID: ${vendor}:${product}"
                [ -n "$mfr" ] && echo "  Manufacturer: $mfr"
                [ -n "$prod" ] && echo "  Product     : $prod"
            fi
        done
        [ $found -eq 0 ] && echo "No external USB devices detected."
    else
        echo "Error: /sys/bus/usb/devices not accessible."
    fi
}

detect_pci() {
    echo "------------------- PCI DEVICES --------------------"
    if [ -d /sys/bus/pci/devices ]; then
        count=0
        for dev in /sys/bus/pci/devices/*; do
            [ -e "$dev" ] || continue
            count=$((count + 1))
            slot=$(basename "$dev")
            vendor=$(cat "$dev/vendor" 2>/dev/null)
            device=$(cat "$dev/device" 2>/dev/null)
            class=$(cat "$dev/class" 2>/dev/null)
            echo "PCI [$slot] Class: $class ID: ${vendor}:${device}"
        done
        echo "Total PCI/PCIe devices: $count"
    else
        echo "Error: /sys/bus/pci/devices not accessible."
    fi
}

case "$1" in
    cpu)     detect_cpu ;;
    mem*|memory) detect_memory ;;
    net*|network) detect_network ;;
    stor*|storage) detect_storage ;;
    usb)     detect_usb ;;
    pci)     detect_pci ;;
    all|"")
        echo "===================================================="
        echo "           MICROOS HARDWARE DISCOVERY REPORT        "
        echo "===================================================="
        detect_cpu
        echo ""
        detect_memory
        echo ""
        detect_storage
        echo ""
        detect_network
        echo ""
        detect_usb
        echo ""
        detect_pci
        echo "===================================================="
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Unknown hardware target: '$1'"
        show_help
        exit 1
        ;;
esac
