#!/usr/bin/env bash
# Continuum OS — bootloader signing (build-pipeline skeleton)
#
# This is a structural skeleton, not a working signer: it documents the
# expected inputs/outputs and integration point for the real HSM-backed
# signing step. Running/wiring to an actual HSM is deferred per current
# project scope.
set -euo pipefail

BOOTLOADER_BINARY="${1:?usage: sign-bootloader.sh <bootloader-binary> <output-signed-binary>}"
OUTPUT_SIGNED="${2:?usage: sign-bootloader.sh <bootloader-binary> <output-signed-binary>}"

echo "[skeleton] Would sign: ${BOOTLOADER_BINARY}"
echo "[skeleton] Signing key: HSM-held Continuum db key (see keys/README.md)"
echo "[skeleton] Would write signed output to: ${OUTPUT_SIGNED}"
echo "[skeleton] NOT IMPLEMENTED — this script documents the interface only."
echo "[skeleton] Real implementation calls out to the HSM signing service"
echo "[skeleton] via the build pipeline's secrets-scoped CI step; see"
echo "[skeleton] ../../ci/README.md for where this slots into the pipeline."
exit 0
