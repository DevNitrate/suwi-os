#!/bin/bash

cargo build
cp target/x86_64-unknown-none/debug/suwi-os esp/boot/
cp limine.conf esp/boot/limine/
./run.sh