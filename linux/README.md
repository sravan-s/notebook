#important

Run all scripts in this directory

Download Guest Kernel Image
You can either use `./download_vmlinux.sh` to download them or  manually do it with the following guide


This agent will live inside a 

How to add agent to rootfs ->

Create an ext4 filesystem in a image file
mount it in a Docker container running Alpine
copy the filesystem from the container

See `./make_rootfs.sh`


Docs: https://github.com/firecracker-microvm/firecracker/blob/main/docs/getting-started.md#getting-a-rootfs-and-guest-kernel-image
https://stanislas.blog/2021/08/firecracker/

