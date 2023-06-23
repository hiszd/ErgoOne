#!/bin/sh

sudo mount /dev/disk/by-label/RPI-RP2 ./mnt/
sudo elf2uf2-rs -d $1
# sudo cp ./target/thumbv6m-none-eabi/release/rp2040-project-template.uf2 ./mnt/
