#!/bin/sh

elf2uf2-rs $1
sudo mount /dev/disk/by-label/RPI-RP2 ./mnt/
sudo cp ./target/thumbv6m-none-eabi/release/rp2040-project-template.uf2 ./mnt/
