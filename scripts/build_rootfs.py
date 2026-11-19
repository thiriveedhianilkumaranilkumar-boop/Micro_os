#!/usr/bin/env python3
"""
MicroOS Rust Edition v2.0 - Root Filesystem Builder
Integrates 100% Rust-compiled binaries into the staging rootfs and
packages them into a compressed CPIO newc initramfs archive.
"""
import os, sys, shutil, gzip, tarfile

BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
STAGING_DIR = os.path.join(BASE_DIR, "build", "rootfs_staging")
INITRAMFS_OUT = os.path.join(BASE_DIR, "build", "initramfs.cpio.gz")
RUST_BIN_DIR = os.path.join(BASE_DIR, "target", "x86_64-unknown-linux-musl", "release")

BUSYBOX_BIN_APPLETS = [
    "mount", "umount", "ls", "cat", "echo", "printf",
    "cp", "mv", "rm", "mkdir", "rmdir", "ps", "kill", "killall", "dmesg", "df", "free",
    "uname", "uptime", "hostname", "clear", "reset", "grep", "egrep", "sed", "awk",
    "cut", "tr", "sort", "uniq", "wc", "head", "tail", "find", "xargs", "tar", "gzip",
    "gunzip", "zcat", "cpio", "ip", "ifconfig", "route", "ping", "udhcpc", "netstat",
    "blkid", "vi", "sleep", "true", "false", "env", "which", "test", "[", "[[",
    "chmod", "chown", "sync", "date", "touch", "id", "whoami", "nslookup"
]

BUSYBOX_SBIN_APPLETS = [
    "reboot", "poweroff", "halt", "mdev", "klogd", "syslogd", "getty"
]

def make_dirs():
    dirs = [
        "bin", "sbin", "usr/bin", "usr/sbin", "dev", "dev/pts", "dev/shm",
        "proc", "sys", "tmp", "run", "mnt", "mnt/ssd", "root", "home",
        "etc", "etc/init.d", "etc/network", "var", "var/log", "var/run"
    ]
    shutil.rmtree(STAGING_DIR, ignore_errors=True)
    for d in dirs:
        os.makedirs(os.path.join(STAGING_DIR, d), exist_ok=True)

def extract_busybox():
    apk_path = os.path.join(BASE_DIR, "rootfs", "busybox-static.apk")
    target_bb = os.path.join(STAGING_DIR, "bin", "busybox")
    print(f"Extracting busybox.static from {apk_path}...")
    with tarfile.open(apk_path, "r:gz") as tar:
        for member in tar.getmembers():
            if member.name.endswith("busybox.static"):
                f = tar.extractfile(member)
                with open(target_bb, "wb") as out:
                    out.write(f.read())
                break
                
    if not os.path.exists(target_bb):
        raise RuntimeError("Failed to extract busybox from APK!")
        
    os.chmod(target_bb, 0o755)
    print(f"Extracted busybox bootstrap: {os.path.getsize(target_bb):,} bytes")

def copy_rust_binaries():
    print("Integrating Rust Edition v2.0 binaries into rootfs...")
    rust_mappings = [
        ("microos-init", "init"),
        ("microos-system-init", "bin/system-init"),
        ("microos-micro-shell", "bin/micro-shell"),
        ("microos-hardware-manager", "bin/hardware"),
        ("microos-storage-manager", "bin/disks"),
        ("microos-network-manager", "bin/netmgr"),
        ("microos-wifi-manager", "bin/wifi"),
        ("microos-package-manager", "bin/apt"),
        ("microos-configuration-manager", "bin/configmgr"),
        ("microos-system-monitor", "bin/sysinfo"),
        ("microos-arcade", "bin/arcade"),
        ("microos-snake", "bin/snake"),
        ("microos-tetris", "bin/tetris"),
        ("microos-pong", "bin/pong"),
        ("microos-minesweeper", "bin/mines"),
        ("microos-game2048", "bin/2048"),
        ("microos-number-guessing", "bin/guess"),
        ("microos-tic-tac-toe", "bin/tictactoe"),
        ("microos-space-invaders", "bin/spaceinvaders"),
    ]

    for src_name, dst_rel in rust_mappings:
        src = os.path.join(RUST_BIN_DIR, src_name)
        dst = os.path.join(STAGING_DIR, dst_rel)
        if not os.path.exists(src):
            raise FileNotFoundError(f"Missing compiled Rust binary: {src}")
        shutil.copyfile(src, dst)
        os.chmod(dst, 0o755)
        print(f"  [RUST] Staged {src_name} -> {dst_rel} ({os.path.getsize(dst):,} B)")

    # Provide /bin/help
    with open(os.path.join(STAGING_DIR, "bin", "help"), "w") as f:
        f.write("#!/bin/micro-shell\nhelp\n")
    os.chmod(os.path.join(STAGING_DIR, "bin", "help"), 0o755)

def copy_system_files():
    print("Configuring system files and inittab for Rust environment...")
    # /etc/inittab: Launches Rust Micro-Shell as primary console shell
    inittab_content = (
        "::sysinit:/etc/init.d/rcS\n"
        "tty1::respawn:/bin/micro-shell\n"
        "console::respawn:/bin/micro-shell\n"
        "::ctrlaltdel:/sbin/reboot\n"
        "::shutdown:/bin/umount -a -r\n"
    )
    with open(os.path.join(STAGING_DIR, "etc", "inittab"), "w") as f:
        f.write(inittab_content)

    # /etc/init.d/rcS
    rcs_content = (
        "#!/bin/sh\n"
        "hostname microos 2>/dev/null\n"
        "ip link set lo up 2>/dev/null\n"
        "rm -rf /tmp/* /run/* 2>/dev/null\n"
        "mkdir -p /tmp /run /var/log /var/run\n"
        "dmesg -n 1 2>/dev/null\n"
        "echo 'MicroOS Rust Edition v2.0 initialized.'\n"
    )
    with open(os.path.join(STAGING_DIR, "etc", "init.d", "rcS"), "w") as f:
        f.write(rcs_content)
    os.chmod(os.path.join(STAGING_DIR, "etc", "init.d", "rcS"), 0o755)

    # /etc/profile
    shutil.copyfile(os.path.join(BASE_DIR, "shell", "profile"), os.path.join(STAGING_DIR, "etc", "profile"))

    # Passwd, hosts, issue
    with open(os.path.join(STAGING_DIR, "etc", "passwd"), "w") as f:
        f.write("root:x:0:0:root:/root:/bin/micro-shell\nmicro:x:1000:1000:micro:/home:/bin/micro-shell\n")
    with open(os.path.join(STAGING_DIR, "etc", "group"), "w") as f:
        f.write("root:x:0:\nwheel:x:10:root\nmicro:x:1000:\n")
    with open(os.path.join(STAGING_DIR, "etc", "hostname"), "w") as f:
        f.write("microos\n")
    with open(os.path.join(STAGING_DIR, "etc", "hosts"), "w") as f:
        f.write("127.0.0.1 localhost microos\n::1 localhost\n")
    with open(os.path.join(STAGING_DIR, "etc", "issue"), "w") as f:
        f.write("MicroOS Rust Edition v2.0 (x86_64) \\n \\l\n\n")

class CpioArchive:
    """Creates a CPIO archive in newc format with symlink support"""
    def __init__(self, out_file):
        self.out = out_file
        self.ino = 1

    def write_entry(self, name, mode, uid, gid, nlink, mtime, filesize, data):
        namesize = len(name) + 1  # includes null terminator
        header = (
            f"070701"
            f"{self.ino:08X}"
            f"{mode:08X}"
            f"{uid:08X}"
            f"{gid:08X}"
            f"{nlink:08X}"
            f"{mtime:08X}"
            f"{filesize:08X}"
            f"{0:08X}"
            f"{0:08X}"
            f"{0:08X}"
            f"{0:08X}"
            f"{namesize:08X}"
            f"{0:08X}"
        ).encode('ascii')
        self.ino += 1

        self.out.write(header)
        self.out.write(name.encode('utf-8') + b'\x00')
        pad_name = (4 - ((110 + namesize) % 4)) % 4
        if pad_name:
            self.out.write(b'\x00' * pad_name)

        if filesize > 0:
            self.out.write(data)
            pad_data = (4 - (filesize % 4)) % 4
            if pad_data:
                self.out.write(b'\x00' * pad_data)

    def write_trailer(self):
        self.write_entry("TRAILER!!!", 0, 0, 0, 1, 0, 0, b"")

def build_cpio():
    print("Assembling newc CPIO archive with virtual symlinks...")
    raw_cpio_path = os.path.join(BASE_DIR, "build", "initramfs.cpio")
    
    with open(raw_cpio_path, "wb") as f:
        cpio = CpioArchive(f)
        
        # Walk staging dir and write real files and directories
        for root, dirs, files in os.walk(STAGING_DIR):
            for d in sorted(dirs):
                full_path = os.path.join(root, d)
                rel_path = os.path.relpath(full_path, STAGING_DIR).replace("\\", "/")
                cpio.write_entry(rel_path, 0o040755, 0, 0, 2, 0, 0, b"")
                
            for file in sorted(files):
                full_path = os.path.join(root, file)
                rel_path = os.path.relpath(full_path, STAGING_DIR).replace("\\", "/")
                mode = 0o100755 if (rel_path.startswith("bin/") or rel_path.startswith("sbin/") or rel_path == "init") else 0o100644
                with open(full_path, "rb") as content_file:
                    content = content_file.read()
                cpio.write_entry(rel_path, mode, 0, 0, 1, 0, len(content), content)
                
        # Primary shell symlink: /bin/sh -> /bin/micro-shell (100% Rust)
        target = "micro-shell"
        cpio.write_entry("bin/sh", 0o120777, 0, 0, 1, 0, len(target), target.encode('utf-8'))

        # Add virtual symlinks for busybox bootstrap applets
        target = "busybox"
        for applet in sorted(BUSYBOX_BIN_APPLETS):
            if applet != "sh":  # /bin/sh is our Rust micro-shell!
                cpio.write_entry(f"bin/{applet}", 0o120777, 0, 0, 1, 0, len(target), target.encode('utf-8'))
            
        target = "../bin/busybox"
        for applet in sorted(BUSYBOX_SBIN_APPLETS):
            cpio.write_entry(f"sbin/{applet}", 0o120777, 0, 0, 1, 0, len(target), target.encode('utf-8'))
            
        cpio.write_trailer()
        
    print(f"Raw CPIO archive size: {os.path.getsize(raw_cpio_path):,} bytes")
    
    # Compress with gzip maximum compression
    print(f"Compressing into {INITRAMFS_OUT}...")
    with open(raw_cpio_path, "rb") as f_in, gzip.open(INITRAMFS_OUT, "wb", compresslevel=9) as f_out:
        shutil.copyfileobj(f_in, f_out)
        
    os.remove(raw_cpio_path)
    initrd_size = os.path.getsize(INITRAMFS_OUT)
    print(f"Compressed initramfs size: {initrd_size:,} bytes ({initrd_size / (1024*1024):.2f} MB)")

def main():
    print("=== Commencing MicroOS Rust Edition v2.0 Rootfs Build ===")
    make_dirs()
    extract_busybox()
    copy_rust_binaries()
    copy_system_files()
    build_cpio()
    print("=== Root Filesystem Build Complete ===")

if __name__ == "__main__":
    main()
