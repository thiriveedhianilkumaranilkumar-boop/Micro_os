#!/usr/bin/env python3
"""
MicroOS Root Filesystem Builder (Optimized CPIO with native symlinks)
Constructs staging directory, system scripts, arcade games, and packages
them into a minimal CPIO initramfs archive (newc format with S_IFLNK symlinks).
"""
import os, sys, shutil, gzip, tarfile

BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
STAGING_DIR = os.path.join(BASE_DIR, "build", "rootfs_staging")
INITRAMFS_OUT = os.path.join(BASE_DIR, "build", "initramfs.cpio.gz")

BUSYBOX_BIN_APPLETS = [
    "sh", "ash", "mount", "umount", "ls", "cat", "echo", "printf",
    "cp", "mv", "rm", "mkdir", "rmdir", "ps", "kill", "killall", "dmesg", "df", "free",
    "uname", "uptime", "hostname", "clear", "reset", "grep", "egrep", "sed", "awk",
    "cut", "tr", "sort", "uniq", "wc", "head", "tail", "find", "xargs", "tar", "gzip",
    "gunzip", "zcat", "cpio", "ip", "ifconfig", "route", "ping", "udhcpc", "netstat",
    "blkid", "vi", "sleep", "true", "false", "env", "which", "test", "[", "[[",
    "chmod", "chown", "sync", "date", "touch", "id", "whoami", "nslookup"
]

BUSYBOX_SBIN_APPLETS = [
    "init", "reboot", "poweroff", "halt", "mdev", "klogd", "syslogd", "getty"
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
    print(f"Extracted busybox binary: {os.path.getsize(target_bb):,} bytes")

def copy_system_files():
    print("Copying MicroOS system scripts and configurations...")
    # /init
    shutil.copyfile(os.path.join(BASE_DIR, "init", "init.sh"), os.path.join(STAGING_DIR, "init"))
    os.chmod(os.path.join(STAGING_DIR, "init"), 0o755)
    
    # /etc/inittab & rcS
    shutil.copyfile(os.path.join(BASE_DIR, "init", "inittab"), os.path.join(STAGING_DIR, "etc", "inittab"))
    shutil.copyfile(os.path.join(BASE_DIR, "init", "rcS"), os.path.join(STAGING_DIR, "etc", "init.d", "rcS"))
    os.chmod(os.path.join(STAGING_DIR, "etc", "init.d", "rcS"), 0o755)
    
    # /etc/profile
    shutil.copyfile(os.path.join(BASE_DIR, "shell", "profile"), os.path.join(STAGING_DIR, "etc", "profile"))
    
    # MicroOS command-line utilities
    utils = [
        ("shell/sysinfo.sh", "bin/sysinfo"),
        ("shell/help.sh", "bin/help"),
        ("hardware/hardware.sh", "bin/hardware"),
        ("networking/wifi.sh", "bin/wifi"),
        ("storage/disks.sh", "bin/disks"),
        ("package-manager/apt.sh", "bin/apt")
    ]
    for src, dst in utils:
        target = os.path.join(STAGING_DIR, dst)
        shutil.copyfile(os.path.join(BASE_DIR, src), target)
        os.chmod(target, 0o755)
        
    # Micro Arcade games
    games = ["snake", "tetris", "pong", "mines", "2048", "guess", "tictactoe", "spaceinvaders"]
    for g in games:
        g_src = os.path.join(BASE_DIR, "games", g)
        g_dst = os.path.join(STAGING_DIR, "bin", g)
        shutil.copyfile(g_src, g_dst)
        os.chmod(g_dst, 0o755)
        
    arcade_src = os.path.join(BASE_DIR, "arcade", "arcade")
    arcade_dst = os.path.join(STAGING_DIR, "bin", "arcade")
    shutil.copyfile(arcade_src, arcade_dst)
    os.chmod(arcade_dst, 0o755)
    
    # Static config files
    with open(os.path.join(STAGING_DIR, "etc", "passwd"), "w") as f:
        f.write("root:x:0:0:root:/root:/bin/sh\nmicro:x:1000:1000:micro:/home:/bin/sh\n")
    with open(os.path.join(STAGING_DIR, "etc", "group"), "w") as f:
        f.write("root:x:0:\nwheel:x:10:root\nmicro:x:1000:\n")
    with open(os.path.join(STAGING_DIR, "etc", "hostname"), "w") as f:
        f.write("microos\n")
    with open(os.path.join(STAGING_DIR, "etc", "hosts"), "w") as f:
        f.write("127.0.0.1 localhost microos\n::1 localhost\n")
    with open(os.path.join(STAGING_DIR, "etc", "issue"), "w") as f:
        f.write("MicroOS v1.0.0 (x86_64) \\n \\l\n\n")

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

    def write_symlink(self, name, target):
        target_bytes = target.encode('utf-8')
        # S_IFLNK = 0120000, 0777 permissions -> 0120777
        self.write_entry(name, 0o120777, 0, 0, 1, 0, len(target_bytes), target_bytes)

    def close(self):
        self.write_entry("TRAILER!!!", 0, 0, 0, 1, 0, 0, b"")
        pos = self.out.tell()
        pad = (512 - (pos % 512)) % 512
        if pad:
            self.out.write(b'\x00' * pad)

def build_initramfs():
    print(f"Creating initramfs archive at {INITRAMFS_OUT}...")
    cpio_temp = os.path.join(BASE_DIR, "build", "initramfs.cpio")
    with open(cpio_temp, "wb") as f_out:
        archive = CpioArchive(f_out)
        
        # 1. Walk staging directory and write real files and directories
        for root, dirs, files in os.walk(STAGING_DIR):
            dirs.sort()
            files.sort()
            for d in dirs:
                full_path = os.path.join(root, d)
                rel_path = os.path.relpath(full_path, STAGING_DIR).replace("\\", "/")
                archive.write_entry(rel_path, 0o040755, 0, 0, 2, 0, 0, b"")
                
            for file in files:
                full_path = os.path.join(root, file)
                rel_path = os.path.relpath(full_path, STAGING_DIR).replace("\\", "/")
                with open(full_path, "rb") as f_in:
                    content = f_in.read()
                if rel_path == "init" or rel_path == "etc/init.d/rcS" or rel_path.startswith("bin/") or rel_path.startswith("sbin/") or rel_path.endswith(".sh"):
                    mode = 0o100755
                else:
                    mode = 0o100644
                archive.write_entry(rel_path, mode, 0, 0, 1, 0, len(content), content)
                
        # 2. Add BusyBox symlinks (virtual symlinks into CPIO)
        for applet in BUSYBOX_BIN_APPLETS:
            archive.write_symlink(f"bin/{applet}", "busybox")
            archive.write_symlink(f"usr/bin/{applet}", "/bin/busybox")
            
        for applet in BUSYBOX_SBIN_APPLETS:
            archive.write_symlink(f"sbin/{applet}", "/bin/busybox")
            archive.write_symlink(f"usr/sbin/{applet}", "/bin/busybox")
            
        archive.close()
        
    cpio_size = os.path.getsize(cpio_temp)
    print(f"Uncompressed CPIO size: {cpio_size:,} bytes ({cpio_size / 1024 / 1024:.2f} MB)")
    
    # Compress with gzip level 9
    with open(cpio_temp, "rb") as f_in:
        with gzip.open(INITRAMFS_OUT, "wb", compresslevel=9) as f_out:
            shutil.copyfileobj(f_in, f_out)
            
    os.remove(cpio_temp)
    gz_size = os.path.getsize(INITRAMFS_OUT)
    print(f"[OK] Compressed initramfs: {gz_size:,} bytes ({gz_size / (1024*1024):.2f} MB)")

def main():
    print("=== Building MicroOS Minimal Root Filesystem ===")
    make_dirs()
    extract_busybox()
    copy_system_files()
    build_initramfs()
    print("Root filesystem and initramfs generation complete!")

if __name__ == "__main__":
    main()
