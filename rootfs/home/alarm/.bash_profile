if [ "$(tty)" = "/dev/tty1" ]; then
    export SLINT_BACKEND=linuxkms
    exec /usr/local/bin/nowa-dash
fi