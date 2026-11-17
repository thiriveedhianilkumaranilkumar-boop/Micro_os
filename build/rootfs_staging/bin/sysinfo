#!/bin/sh
# MicroOS System Overview Utility
# High-contrast, lightweight system summary

echo -e "\033[1;36m"
cat << 'EOF'
 __  __ _             ___  ____  
|  \/  (_) ___ _ __ / _ \/ ___| 
| |\/| | |/ __| '__| | | \___ \ 
| |  | | | (__| |  | |_| |___) |
|_|  |_|_|\___|_|   \___/|____/ 
EOF
echo -e "\033[0m"
echo -e "\033[1;37m  Minimal Linux. Maximum control.\033[0m"
echo "----------------------------------------------------"

# OS & Kernel
kernel=$(uname -r)
arch=$(uname -m)
echo -e "  \033[1;34mOS\033[0m          : MicroOS v1.0.0 ($arch)"
echo -e "  \033[1;34mKernel\033[0m      : Linux $kernel"

# Uptime
if [ -f /proc/uptime ]; then
    secs=$(cut -d. -f1 /proc/uptime)
    mins=$((secs / 60))
    hours=$((mins / 60))
    days=$((hours / 24))
    echo -e "  \033[1;34mUptime\033[0m      : ${days}d ${hours%24}h ${mins%60}m"
fi

# CPU
if [ -f /proc/cpuinfo ]; then
    cpu=$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^[ \t]*//')
    cores=$(grep -c '^processor' /proc/cpuinfo)
    echo -e "  \033[1;34mCPU\033[0m         : $cpu (${cores} vCPUs)"
fi

# Memory
if [ -f /proc/meminfo ]; then
    total=$(grep -m1 'MemTotal:' /proc/meminfo | awk '{printf "%.1f MB", $2/1024}')
    free=$(grep -m1 'MemAvailable:' /proc/meminfo | awk '{printf "%.1f MB", $2/1024}')
    echo -e "  \033[1;34mMemory\033[0m      : $free free / $total total"
fi

# Storage / Mounts
if [ -f /proc/mounts ]; then
    root_use=$(df -h / 2>/dev/null | tail -n1 | awk '{print $3 " / " $2 " (" $5 ")"}')
    echo -e "  \033[1;34mRootfs (RAM)\033[0m: $root_use"
    if grep -q '/mnt/ssd' /proc/mounts; then
        ssd_use=$(df -h /mnt/ssd 2>/dev/null | tail -n1 | awk '{print $3 " / " $2 " (" $5 ")"}')
        echo -e "  \033[1;32mSSD Storage\033[0m : $ssd_use (Persistent)"
    else
        echo -e "  \033[1;33mSSD Storage\033[0m : Not mounted (Run 'disks mount <dev> /mnt/ssd')"
    fi
fi

# Network IP
ip_addr=$(ip -4 addr show 2>/dev/null | grep 'inet ' | grep -v '127.0.0.1' | awk '{print $2}' | head -n1)
echo -e "  \033[1;34mNetwork IP\033[0m  : ${ip_addr:-[Offline]}"

echo "----------------------------------------------------"
echo -e "Type \033[1;32mhelp\033[0m for command reference or \033[1;33marcade\033[0m for games."
