#!/usr/bin/env bash
set -euo pipefail

IMG=$1
MNT=mnt

truncate -s 10G "$IMG"
parted -s "$IMG" mklabel msdos mkpart primary fat32 1MiB 513MiB mkpart primary ext4 513MiB 100%

LOOPDEV=$(losetup --show -fP "$IMG")
mkfs.vfat -F32 "${LOOPDEV}p1"
mkfs.ext4 -F "${LOOPDEV}p2"

mkdir -p "$MNT"
mount "${LOOPDEV}p2" "$MNT"
mkdir "$MNT/boot"
mount -o quiet "${LOOPDEV}p1" "$MNT/boot"

curl -fsSL http://os.archlinuxarm.org/os/ArchLinuxARM-rpi-aarch64-latest.tar.gz |
	tar -xzp -C "$MNT" --numeric-owner

cp -r --remove-destination rootfs/. "$MNT/"
mkdir -p "$MNT/root"
cp -r android-auto "$MNT/root/android-auto"

for fs in proc sys dev dev/pts; domount --bind "/$fs" "$MNT/$fs"; done

install -m 755 build/chroot-setup.sh "$MNT/root/"
chroot "$MNT" /root/chroot-setup.sh
rm "$MNT/root/chroot-setup.sh"

BOOT_UUID=$(blkid -s UUID -o value "${LOOPDEV}p1")
ROOT_UUID=$(blkid -s UUID -o value "${LOOPDEV}p2")
sed -i "s#^/dev/mmcblk0p1.*#UUID=${BOOT_UUID}  /boot  vfat  defaults  0  0#" "$MNT/etc/fstab"
sed -i "s#root=/dev/mmcblk0p2#root=UUID=${ROOT_UUID}#" "$MNT/boot/cmdline.txt"

fstrim -v "$MNT"
fstrim -v "$MNT/boot"
sync
umount -R "$MNT"
losetup -d "$LOOPDEV"
