#!/bin/bash

cargo build
cp target/x86_64-unknown-none/debug/suwi-os esp/boot/
cp limine.conf esp/boot/limine/
qemu-system-x86_64 --bios ovmf/OVMF.4m.fd -drive file=fat:rw:esp/,format=raw