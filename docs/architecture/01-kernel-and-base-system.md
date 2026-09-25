# Kernel & Base System Layer

## 1.1 Kernel choice

**Decision:** mainline Linux kernel, LTS branch. Same foundation choice
as ChromeOS, SteamOS, Android, and most independent desktop OS projects.
See `../adr/0001-kernel-choice.md` for the full decision record.

Rationale:
- Immediate driver coverage for essentially all GPUs, Wi-Fi/Bluetooth
  chipsets, USB, NVMe, and peripherals — the largest cost center in any
  from-scratch OS is avoided.
- Inherits a security-patch/CVE pipeline maintained by a large upstream
  community rather than staffing one alone.
- Precedent that kernel choice doesn't dictate UX: Android and ChromeOS
  run wildly different shells on the same kernel — ContinuumShell is a
  userland/compositor decision, not a kernel one.
- A custom microkernel (seL4-style) or fully original kernel is
  evaluated and **rejected for v1**: multi-year driver bring-up cost with
  no UX benefit. Revisit only after Continuum OS has market traction and
  a dedicated kernel team.

## 1.2 Base system layer

- **Init system:** a minimal, security-hardened init (systemd, or a
  purpose-built alternative — see `../adr/0002-init-system.md`) managing
  boot sequence, service supervision, and the continuity daemon suite
  (Section 8 / `06-continuity-subsystem.md`).
- **Filesystem:** ext4 or btrfs with an OS-partition **read-only root**
  (ChromeOS / Fedora Silverblue model) for update reliability and
  rollback safety. See `../adr/0003-filesystem-and-update-model.md`.
- **Update model:** A/B (dual-partition) atomic OS updates — the OS
  updates in the background and swaps on next reboot, with automatic
  rollback on boot failure. Avoids the "update bricked my machine"
  failure mode. Detail in `02-boot-install-update.md`.
- **Package/app model:** a sandboxed, capability-scoped app format
  (OS-native containerized apps, similar in spirit to Flatpak / macOS App
  Sandbox) as the primary distribution format. See
  `../adr/0004-app-sandbox-format.md`.

## Open questions carried into Phase 0

- Final init system pick (systemd vs. purpose-built) — deferred to ADR
  0002, to be closed during Phase 0 kernel bring-up once boot-time and
  attack-surface data is available from the reference image.
- ext4 vs. btrfs for root — btrfs gives snapshot/rollback primitives that
  may simplify the A/B implementation, at the cost of higher complexity
  and a less mature Linux-desktop track record; decision tied to
  `../adr/0003-filesystem-and-update-model.md`.
