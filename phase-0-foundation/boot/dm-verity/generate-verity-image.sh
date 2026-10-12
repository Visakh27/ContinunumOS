#!/usr/bin/env bash
# Continuum OS — dm-verity image generation (build-pipeline skeleton)
set -euo pipefail

ROOTFS_IMAGE="${1:?usage: generate-verity-image.sh <rootfs.img> <output-dir>}"
OUTPUT_DIR="${2:?usage: generate-verity-image.sh <rootfs.img> <output-dir>}"

echo "[skeleton] Would run: veritysetup format ${ROOTFS_IMAGE} ${OUTPUT_DIR}/hash-tree.img"
echo "[skeleton] Would extract root hash, sign it with the db key"
echo "[skeleton] (see ../secure-boot/keys/README.md), and write:"
echo "[skeleton]   ${OUTPUT_DIR}/root-hash.sig"
echo "[skeleton] NOT IMPLEMENTED — structural skeleton only."
exit 0
