apk add --no-cache openrc
apk add --no-cache util-linux
apk add --no-cache --update nodejs nodejs-npm

ln -s agetty /etc/init.d/agetty.ttyS0
echo ttyS0 >/etc/securetty
rc-update add agetty.ttyS0 default

echo "root:root" | chpasswd

echo "nameserver 1.1.1.1" >>/etc/resolv.conf

addgroup -g 1000 -S notebook && adduser -u 1000 -S notebook -G notebook

rc-update add devfs boot
rc-update add procfs boot
rc-update add sysfs boot

rc-update add agent boot

for d in bin etc lib root sbin usr; do tar c "/$d" | tar x -C /my-rootfs; done
for dir in dev proc run sys var tmp; do mkdir /my-rootfs/${dir}; done

chmod 1777 /my-rootfs/tmp
mkdir -p /my-rootfs/home/notebook/
chown 1000:1000 /my-rootfs/home/notebook/

