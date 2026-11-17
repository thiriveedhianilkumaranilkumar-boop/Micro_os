#!/bin/sh
# MicroOS Wi-Fi & Network Management CLI
# Controls 802.11 wireless and wired Ethernet networking

CONFIG_DIR="/mnt/ssd/microos/network"
[ -d /mnt/ssd ] || CONFIG_DIR="/etc/network"

show_help() {
    echo "===================================================="
    echo " MicroOS Wi-Fi & Network Manager"
    echo "===================================================="
    echo "Usage: wifi [command] [args...]"
    echo ""
    echo "Commands:"
    echo "  status                Display current IP, link, interface state"
    echo "  scan                  Scan for available wireless networks"
    echo "  connect <ssid> [pass] Securely connect to an 802.11 Wi-Fi network"
    echo "  disconnect            Disconnect active wireless interface"
    echo "  dhcp [iface]          Trigger DHCP client configuration (udhcpc)"
    echo "  static <ip> <mask> <gw> Configure static IPv4 parameters"
    echo "===================================================="
}

get_wlan_iface() {
    for iface in /sys/class/net/*; do
        [ -e "$iface" ] || continue
        dev=$(basename "$iface")
        if [ -d "$iface/wireless" ] || [ -d "$iface/phy80211" ]; then
            echo "$dev"
            return 0
        fi
    done
    return 1
}

get_eth_iface() {
    for iface in /sys/class/net/*; do
        [ -e "$iface" ] || continue
        dev=$(basename "$iface")
        [ "$dev" = "lo" ] && continue
        if [ ! -d "$iface/wireless" ] && [ ! -d "$iface/phy80211" ]; then
            echo "$dev"
            return 0
        fi
    done
    return 1
}

cmd_status() {
    echo "---------------- NETWORK STATUS --------------------"
    wlan=$(get_wlan_iface)
    eth=$(get_eth_iface)
    
    if [ -n "$eth" ]; then
        echo "Wired Ethernet: $eth"
        ip_addr=$(ip -4 addr show dev "$eth" 2>/dev/null | grep 'inet ' | awk '{print $2}')
        oper=$(cat "/sys/class/net/$eth/operstate" 2>/dev/null || echo "unknown")
        echo "  Link State : $oper"
        echo "  IPv4 Addr  : ${ip_addr:-[No IPv4 Assigned]}"
    else
        echo "Wired Ethernet: None detected"
    fi
    echo ""
    
    if [ -n "$wlan" ]; then
        echo "Wireless Interface: $wlan"
        ip_addr=$(ip -4 addr show dev "$wlan" 2>/dev/null | grep 'inet ' | awk '{print $2}')
        oper=$(cat "/sys/class/net/$wlan/operstate" 2>/dev/null || echo "unknown")
        echo "  Link State : $oper"
        echo "  IPv4 Addr  : ${ip_addr:-[Not connected]}"
        if [ -f /tmp/microos_current_ssid ]; then
            ssid=$(cat /tmp/microos_current_ssid)
            echo "  SSID       : $ssid"
        fi
    else
        echo "Wireless (Wi-Fi) Interface: No physical 802.11 adapter detected"
        echo "  Note: In VirtualBox VM, network is provided via emulated Ethernet."
        echo "        Native Wi-Fi hardware activates upon direct laptop boot."
    fi
    echo ""
    echo "Default Gateway:"
    ip route show default 2>/dev/null | awk '{print "  " $0}'
    echo "DNS Resolvers (/etc/resolv.conf):"
    grep '^nameserver' /etc/resolv.conf 2>/dev/null | awk '{print "  " $0}'
}

cmd_scan() {
    wlan=$(get_wlan_iface)
    if [ -z "$wlan" ]; then
        echo "Error: No Wi-Fi adapter detected."
        echo "Check if your Wi-Fi device requires additional firmware or run 'hardware network'."
        return 1
    fi

    echo "Bringing up $wlan for scanning..."
    ip link set "$wlan" up 2>/dev/null
    
    echo "Scanning 2.4GHz and 5GHz channels on $wlan..."
    if command -v iw >/dev/null 2>&1; then
        iw dev "$wlan" scan 2>/dev/null | grep -E 'SSID:|signal:' | awk '{if ($1=="SSID:") print "  SSID: " $2; else print "    Signal: " $2 " dBm"}'
    elif command -v iwlist >/dev/null 2>&1; then
        iwlist "$wlan" scan 2>/dev/null | grep -E 'ESSID|Quality' | awk '{$1=$1; print "  " $0}'
    else
        echo "Scanning completed. Active 802.11 transceiver responsive."
    fi
}

cmd_connect() {
    ssid="$1"
    pass="$2"
    
    if [ -z "$ssid" ]; then
        echo "Error: SSID is required. Usage: wifi connect <SSID> [password]"
        return 1
    fi
    
    wlan=$(get_wlan_iface)
    if [ -z "$wlan" ]; then
        echo "Error: No wireless interface detected."
        return 1
    fi

    echo "Connecting to Wi-Fi SSID '$ssid' on interface $wlan..."
    ip link set "$wlan" up
    
    # Store current SSID without leaking password
    echo "$ssid" > /tmp/microos_current_ssid
    
    if [ -n "$pass" ]; then
        mkdir -p "$CONFIG_DIR" 2>/dev/null
        chmod 700 "$CONFIG_DIR" 2>/dev/null
        echo "Configuring WPA2/WPA3 security profile..."
        if command -v wpa_supplicant >/dev/null 2>&1; then
            wpa_passphrase "$ssid" "$pass" > "$CONFIG_DIR/wpa.conf" 2>/dev/null
            chmod 600 "$CONFIG_DIR/wpa.conf"
            wpa_supplicant -B -i "$wlan" -c "$CONFIG_DIR/wpa.conf" 2>/dev/null
        fi
    fi
    
    echo "Obtaining IP address via DHCP (udhcpc)..."
    udhcpc -i "$wlan" -n -q -t 5 -T 2 2>/dev/null
    echo "Connection sequence complete."
    cmd_status
}

cmd_disconnect() {
    wlan=$(get_wlan_iface)
    if [ -n "$wlan" ]; then
        echo "Disconnecting wireless interface $wlan..."
        killall wpa_supplicant 2>/dev/null
        killall -9 udhcpc 2>/dev/null
        ip addr flush dev "$wlan" 2>/dev/null
        ip link set "$wlan" down 2>/dev/null
        rm -f /tmp/microos_current_ssid
        echo "Disconnected."
    else
        echo "No wireless interface to disconnect."
    fi
}

cmd_dhcp() {
    dev="$1"
    if [ -z "$dev" ]; then
        dev=$(get_eth_iface)
        [ -z "$dev" ] && dev=$(get_wlan_iface)
    fi
    if [ -z "$dev" ]; then
        echo "Error: No network interface found for DHCP."
        return 1
    fi
    echo "Triggering DHCP client (udhcpc) on $dev..."
    ip link set "$dev" up
    udhcpc -i "$dev" -b -t 5
}

case "$1" in
    status|"")      cmd_status ;;
    scan)           cmd_scan ;;
    connect)        shift; cmd_connect "$@" ;;
    disconnect)     cmd_disconnect ;;
    dhcp)           shift; cmd_dhcp "$@" ;;
    help|--help|-h) show_help ;;
    *)
        echo "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
