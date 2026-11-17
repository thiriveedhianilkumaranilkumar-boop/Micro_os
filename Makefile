# MicroOS Master Makefile

PYTHON = C:/Users/t_ani/AppData/Local/Programs/Python/Python312/python.exe
PROJECT_ROOT = .

.PHONY: all clean kernel rootfs games iso test-vbox verify-size release

all: kernel rootfs games iso verify-size

kernel:
	@echo "=== Building Kernel Configuration and Artifacts ==="
	$(PYTHON) scripts/build_kernel.py

rootfs:
	@echo "=== Building Root Filesystem ==="
	$(PYTHON) scripts/build_rootfs.py

games:
	@echo "=== Compiling Micro Arcade Games ==="
	$(PYTHON) scripts/build_games.py

iso:
	@echo "=== Generating Bootable Hybrid ISO ==="
	$(PYTHON) scripts/build_iso.py

verify-size:
	@echo "=== Verifying ISO Size Budget (<= 50 MB) ==="
	$(PYTHON) tests/test_iso_size.py

test-vbox:
	@echo "=== Running Oracle VirtualBox Automated Verification ==="
	$(PYTHON) tests/test_vbox_boot.py

clean:
	@echo "=== Cleaning build artifacts ==="
	$(PYTHON) -c "import shutil, os; [shutil.rmtree(d, ignore_errors=True) for d in ['build/rootfs_staging', 'build/iso_staging']]"
