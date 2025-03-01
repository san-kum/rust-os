#!/usr/bin/env bash
set -ex

# Build the kernel and create a bootable disk image
cargo bootimage --release

# The bootable disk image is created at target/x86_64-unknown-none/release/bootimage-rust-os.bin
cp target/x86_64-unknown-none/release/bootimage-rust-os.bin myos.bin

# Create ISO (optional if you want to keep using GRUB)
mkdir -p isodir/boot/grub
cp myos.bin isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o myos.iso isodir
