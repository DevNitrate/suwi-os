#!/bin/bash

sudo mount /dev/sda1 /mnt/usb
sudo cp -r esp/* /mnt/usb
sudo umount /mnt/usb