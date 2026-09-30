#!/bin/bash
set -e

pacman-key --init
pacman-key --populate archlinuxarm
pacman -Sy --noconfirm

# Uninstall old kernel
pacman -Rns --noconfirm linux-aarch64 uboot-raspberrypi

# Install rpi-linux kernel
pacman -S --noconfirm linux-rpi linux-rpi-headers
pacman -Syu --noconfirm

# Install sudo
pacman -S --noconfirm sudo

# makepkg refuses to run as root, create a temporary build user with passwordless sudo
useradd -m builder
echo "builder ALL=(ALL) NOPASSWD: ALL" >/etc/sudoers.d/builder

# Install everything
pacman -S --noconfirm --needed \
  git \
  base-devel \
  cmake \
  ninja \
  boost \
  libusb \
  protobuf \
  openssl \
  qt5-base \
  qt5-multimedia \
  rtaudio \
  exfatprogs \
  networkmanager \
  labwc \
  nemo \
  quickshell \
  swaybg \
  foot \
  android-udev \
  qt5-wayland \
  pipewire \
  pipewire-pulse \
  pipewire-alsa \
  wireplumber \
  pipewire-jack \
  gstreamer \
  gst-libav \
  gst-plugins-base \
  gst-plugins-bad \
  gst-plugins-good

# Enable pipewire
systemctl --global enable pipewire pipewire-pulse wireplumber

# Build aasdk
ANDROID_AUTO_SRC="/root/android-auto"
BUILD_DIR="/root/build"
INSTALL_PREFIX="/usr"

mkdir -p "$BUILD_DIR"
chown -R builder:builder "$ANDROID_AUTO_SRC" "$BUILD_DIR"
chmod 755 /root

sudo -u builder bash -c "
  set -e
  cmake -S '$ANDROID_AUTO_SRC/aasdk' -B '$BUILD_DIR/aasdk' \
    -GNinja \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX='$INSTALL_PREFIX'
  cmake --build '$BUILD_DIR/aasdk' -j\$(nproc)
"
cmake --install "$BUILD_DIR/aasdk"

ldconfig

# Build openauto
sudo -u builder bash -c "
  set -e
  cmake -S '$ANDROID_AUTO_SRC/openauto' -B '$BUILD_DIR/openauto' \
    -GNinja \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX='$INSTALL_PREFIX'
  cmake --build '$BUILD_DIR/openauto' -j\$(nproc)
"
cmake --install "$BUILD_DIR/openauto"

# Clean up build tree and source checkout — not needed on the final image
rm -rf "$BUILD_DIR" "$ANDROID_AUTO_SRC"

pacman -Rns --noconfirm git cmake base-devel ninja boost
pacman -S --noconfirm --needed boost-libs # Reinstalling because removing boost also removes boost-libs

pacman -Scc --noconfirm || true
rm -rf /var/cache/pacman/pkg/*
rm -rf /var/cache/pacman/pkg/.[!.]*
rm -rf /var/cache/pacman/pkg/..?*

# Clean up the temporary build user and its sudo grant
userdel -r builder
rm /etc/sudoers.d/builder

systemctl enable NetworkManager

# Fix home dir permissions
chown -R alarm:alarm /home/alarm

# Generate en_US.UTF-8 locale
locale-gen

# mkdir -p /storage/music
# chmod -R 777 /storage
