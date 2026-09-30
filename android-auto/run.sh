#!/bin/bash
set -e

echo "Building aasdk and openauto..."
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AASDK_DIR="$ROOT_DIR/aasdk"
OPENAUTO_DIR="$ROOT_DIR/openauto"
OUTPUT_DIR="${TOP_DIR:-$ROOT_DIR}/output"

AASDK_BUILD_DIR="$OUTPUT_DIR/aasdk"
AASDK_STAGE_DIR="$AASDK_BUILD_DIR/stage"
OPENAUTO_BUILD_DIR="$OUTPUT_DIR/openauto"

mkdir -p "$OUTPUT_DIR" "$OPENAUTO_BUILD_DIR"

cmake -S "$AASDK_DIR" -B "$AASDK_BUILD_DIR" -DCMAKE_INSTALL_PREFIX="$AASDK_STAGE_DIR"
cmake --build "$AASDK_BUILD_DIR" -j"$(nproc)"
cmake --install "$AASDK_BUILD_DIR"

# As seen in `ldd ./android-auto/openauto/bin/autoapp | grep aasdk` these flags should be changed when used in building for the rpi image.
cmake -S "$OPENAUTO_DIR" -B "$OPENAUTO_BUILD_DIR" \
  -DCMAKE_EXE_LINKER_FLAGS="-Wl,--copy-dt-needed-entries" \
  -DCMAKE_PREFIX_PATH="$AASDK_STAGE_DIR"
cmake --build "$OPENAUTO_BUILD_DIR" -j"$(nproc)"

echo "Build complete."
echo "Executing openauto..."
"$OPENAUTO_DIR/bin/autoapp"
