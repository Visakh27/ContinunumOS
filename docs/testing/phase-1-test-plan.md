# Phase 1 Test Plan — Display, Compositor & Shell Skeleton

**Covers:** `phase-1-shell-compositor/`

- **Unit:** compositor output/surface management logic, window-manager
  layout algorithms, Dock/menu-bar state machines — all without a real
  GPU (mocked backend).
- **Integration:** headless compositor session bring-up + a Flatpak
  reference app launching, resizing, closing under ContinuumWM in CI
  using a software renderer (llvmpipe).
- **HIL:** GPU-accelerated multi-monitor bring-up, virtual-desktop
  switching, and the sandboxed app-runtime on Tier-1 reference hardware
  — this is where real GPU driver behavior gets exercised.
- **Exit gate:** boots to a usable desktop with window management and a
  handful of ported Linux apps, per the Phase 1 milestone
  (`ROADMAP.md`), demonstrated by the HIL suite passing on all Tier-1
  devices.

Skeleton: `phase-1-shell-compositor/tests/`.
