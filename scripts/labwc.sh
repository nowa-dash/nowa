#!/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG="$ROOT/rootfs/home/alarm/.config"

QS_CONFIG_PATH="$CONFIG/quickshell" exec labwc -C "$CONFIG/labwc"