# Kernel Baseline

Decision record: `../../docs/adr/0001-kernel-choice.md`.

## Version policy
- Track the current mainline **Linux LTS** branch. Pin the exact version
  in `VERSION_PIN` (this directory) once Phase 0 build-system work
  selects a concrete baseline — left as a placeholder here since pinning
  before the build system exists would go stale immediately.
- Rebase the Continuum patch set onto each new LTS point release monthly;
  onto a new LTS major only after a full HIL regression pass.

## Patch-tracking process
1. Every Continuum-specific kernel change lives in `patches/` as a
   quilt-style numbered series (`NNNN-description.patch`), never as a
   direct edit to a vendored kernel tree, so the patch set stays
   rebaseable.
2. Each patch's commit message states: what upstream doesn't yet do,
   why Continuum needs it now, and the tracking issue for upstreaming it
   (Hardware Enablement team owns upstreaming per
   `../../docs/architecture/03-driver-hardware-strategy.md`).
3. A patch lands only with a corresponding HIL boot-test pass (see
   `../ci/README.md`) on at least one Tier-1 reference device.

## Config fragments
`config/` holds Kconfig fragments (not a full monolithic `.config`) split
by concern, merged at build time by the build system:
- `config/base.kconfig` — minimum viable kernel (console, ext4, basic net)
- `config/security.kconfig` — dm-verity, Secure Boot chain requirements,
  seccomp, module signing
- `config/continuity.kconfig` — BLE, Wi-Fi Direct/Aware, V4L2 virtual
  device support (`uvcvideo`/`v4l2loopback`-class options) needed by the
  continuity daemons from Phase 2 onward, enabled early so hardware
  bring-up isn't blocked later
- `config/graphics.kconfig` — DRM/KMS, GPU driver options for the
  compositor (Phase 1)

## Open item
Exact LTS version and the base/security/continuity/graphics fragment
contents are a build-system-integration task (Session 3,
`../../SESSION_LOG.md`) — this README defines the *process*, the
fragments below are structural placeholders.
