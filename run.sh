#!/bin/bash

qemu-system-x86_64 --bios ovmf/OVMF.4m.fd -drive file=fat:rw:esp/,format=raw -d int,cpu_reset -no-reboot -D qemu.log