#!/bin/sh

rm -f pflash.img
rm -f disk.img
make pflash_img
make disk_img

make run A=labs/lab1 LOG=info 2>/dev/null
