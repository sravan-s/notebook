#!/bin/bash

# https://stanislas.blog/2021/08/firecracker/

# this set of commands creates a  200Mb ext4 filesystem in a file (rootfs.ext4),
# formats it, and mounts it to the /tmp/my-rootfs directory so that it can be
# used like a real filesystem

mkdir -p assets

dd if=/dev/zero of=rootfs.ext4 bs=1M count=200
mkfs.ext4 rootfs.ext4
mkdir -p /tmp/my-rootfs
mount rootfs.ext4 /tmp/my-rootfs

echo "Filesystem mounted at /tmp/my-rootfs"

docker run -i --rm \
    -v /tmp/my-rootfs:/my-rootfs \
    -v "$(pwd)/agent:/usr/local/bin/agent" \
    -v "$(pwd)/openrc-service.sh:/etc/init.d/agent" \
    alpine sh <setup-alpine.sh

umount /tmp/my-rootfs

mv rootfs.ext4 ./assets/rootfs.ext4


