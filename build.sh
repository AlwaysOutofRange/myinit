#!/bin/bash
# Script to build a minimal initramfs with Rust init program

set -e  # Exit on error

function build() {
  echo "Building Rust init program..."
  RUSTFLAGS='-C link-arg=-s' cargo build --release --target x86_64-unknown-linux-musl
}

function create_initramfs() {
  echo "Creating initramfs..."

  # Clean and create directories
  rm -rf initramfs
  mkdir -p initramfs/{dev,proc,sys,bin}

  # Copy the Rust init binary
  if [ ! -f target/x86_64-unknown-linux-musl/release/myinit ]; then
    echo "Error: Binary not found. Run './build-initramfs.sh build' first"
    exit 1
  fi

  cp target/x86_64-unknown-linux-musl/release/myinit initramfs/init
  chmod 755 initramfs/init

  # Add BusyBox
  if command -v busybox >/dev/null 2>&1; then
    cp $(which busybox) initramfs/bin/busybox
    chmod 755 initramfs/bin/busybox

    # Create essential symlinks
    cd initramfs/bin
    ln -sf busybox sh
    ln -sf busybox ls
    ln -sf busybox cat
    ln -sf busybox echo
    cd ../..
  else
    echo "ERROR: BusyBox not found! Please install it."
    exit 1
  fi

  # Create essential device nodes
  sudo mknod -m 666 initramfs/dev/null c 1 3
  sudo mknod -m 622 initramfs/dev/console c 5 1
  sudo mknod -m 666 initramfs/dev/zero c 1 5

  # Show the final structure
  echo "Initramfs structure:"
  find initramfs -type f | sort

  # Create cpio archive
  echo "Creating initramfs archive..."
  (cd initramfs && find . | cpio -H newc -o | gzip -9) > initramfs.cpio.gz

  ls -lh initramfs.cpio.gz
  rm -rf initramfs
  echo "Success: initramfs.cpio.gz created"
}

function run_qemu() {
  echo "Running in QEMU..."
  qemu-system-x86_64 \
    -kernel /boot/vmlinuz-linux \
    -initrd initramfs.cpio.gz \
    -append "console=ttyS0 init=/init panic=5" \
    -nographic \
    -no-reboot
}

function all() {
  build
  create_initramfs
  run_qemu
}

# Process command-line arguments
case "$1" in
  build)
    build
    ;;
  initramfs)
    create_initramfs
    ;;
  qemu)
    run_qemu
    ;;
  all|"")
    all
    ;;
  *)
    echo "Usage: $0 [build|initramfs|qemu|all]"
    exit 1
    ;;
esac

exit 0
