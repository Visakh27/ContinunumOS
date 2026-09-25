# ADR 0003: Root Filesystem & Update Model

**Status:** Accepted (filesystem: provisionally ext4; update model: A/B partitions)

## Context
Need a root filesystem and update model that supports atomic,
rollback-safe OS updates (see `../architecture/02-boot-install-update.md`).

## Decision
- Read-only root partition, A/B (dual-partition) atomic updates,
  ChromeOS/Fedora-Silverblue-style.
- Filesystem: **ext4**, provisionally, for Phase 0. btrfs is the
  candidate alternative (native snapshot/rollback primitives could
  simplify the A/B mechanism) but carries more complexity and a less
  mature track record on Linux desktop workloads at time of writing.
  Revisit once the A/B update mechanism (`phase-0-foundation/boot/ab-update/`)
  has a working ext4 implementation to compare against.

## Consequences
- A/B doubles the OS-partition storage footprint on every device —
  factored into Tier-1 minimum-storage requirements
  (`phase-6-oem-certification-ga/oem-cert-kit/`).
- Read-only root means all app data, user data, and mutable OS state
  must live outside root (a `/var`-equivalent writable partition/overlay)
  — this boundary must be defined precisely before Phase 1 app-runtime
  work begins, since it determines what an app sandbox can and can't
  persist to.
