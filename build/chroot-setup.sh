#!/bin/bash
set -euo pipefail

pacman-key --init
pacman-key --populate archlinuxarm

# Uninstall old kernel
pacman -Rns --noconfirm linux-aarch64 uboot-raspberrypi

# Install everything
pacman -Syu --noconfirm --needed \
	linux-rpi \
	sudo \
	exfatprogs \
	networkmanager \
	android-udev \
	mesa \
	libinput \
	libxkbcommon \
	alsa-lib \
	fontconfig \
	ttf-dejavu \
	pipewire \
	pipewire-pulse \
	pipewire-alsa \
	wireplumber \
	bluez \
	bluez-utils

# Crash the workflow if there are missing runtime dependencies
if ldd /usr/local/bin/nowa-dash | grep 'not found'; then exit 1; fi

# Disable systemd-networkd and enable networkmanager
systemctl disable systemd-networkd
systemctl enable NetworkManager

# Enable pipewire
systemctl --global enable pipewire pipewire-pulse wireplumber

# Enable bluetooth
systemctl enable bluetooth

# Enable the bluez mpris proxy
systemctl --global enable mpris-proxy.service

# Hardware video decoder (/dev/video*) for Android Auto
usermod -aG video alarm

find /var/cache/pacman/pkg -mindepth 1 -delete

# Fix home dir permissions
chown -R alarm:alarm /home/alarm

# Generate en_US.UTF-8 locale
locale-gen

# mkdir -p /storage/music
# chmod -R 777 /storage
