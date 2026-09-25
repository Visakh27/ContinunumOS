# dm-verity Root Verification

Implements step 2–3 of the boot chain: the bootloader verifies and loads
the kernel+initramfs, and the kernel mounts the **read-only root
partition** only after dm-verity confirms its hash tree matches the
signed root hash (per ADR 0003's read-only-root decision).

## Components

- `generate-verity-image.sh` — build-pipeline skeleton: given a built
  root filesystem, generates the Merkle hash tree and root hash, and
  signs the root hash with the db key.
- `verity-mount-unit.template` — init-system unit template (systemd
  convention, per ADR 0002's working assumption) that fails boot (drops
  to recovery, see `../../../cross-cutting/recovery-mode/`) if
  verification fails, rather than mounting anyway.

## Design notes

- Hash-tree block size and the exact verity format (`veritysetup`
  defaults vs. custom) is a build-system-implementation detail, not an
  architectural decision — deferred to Session 4 follow-up.
- The signed root hash — not the whole hash tree — is what Secure Boot's
  chain of trust ultimately anchors: `secure-boot/` verifies the
  bootloader+kernel+initramfs; the initramfs then verifies the root
  hash's signature before asking dm-verity to enforce it. This closes
  the loop so a modified root partition is caught even though dm-verity
  itself runs *after* Secure Boot's own verification stops.
