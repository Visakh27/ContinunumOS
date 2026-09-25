# Phase 1 — Display, Compositor & Shell Skeleton (M7–M14)

## Scope (source plan, Section 11)
- Build ContinuumWM (Wayland compositor) with GPU-accelerated
  multi-monitor support.
- Build the shell skeleton: menu bar, Dock, window management, virtual
  desktops.
- Implement the sandboxed app runtime and get Flatpak-format Linux apps
  running.

## Added to scope
- Virtual-display output support in the compositor's output pipeline,
  designed in now even though `continuum-controld` doesn't consume it
  until Phase 5 (ADR 0005 — retrofitting later is expected to be far
  more expensive).
- Accessibility-tree API in ContinuumKit (`app-runtime/`) — see
  `../cross-cutting/accessibility/`.
- IME framework groundwork — see `../cross-cutting/localization/`.
- General desktop audio stack — see `../cross-cutting/audio-stack/`.
- Native OS notification model — see
  `../docs/architecture/04-desktop-shell-ux.md`.

## Entry criteria
Phase 0 exit criteria met (`../phase-0-foundation/README.md`): verified
boot + A/B update working on a bootable minimal image.

## Exit criteria
- [ ] Boots to a usable desktop with window management.
- [ ] A handful of ported Linux (Flatpak) apps run under the sandboxed
      app runtime.
- [ ] Virtual-desktop switching functional.
- [ ] Compositor output pipeline supports a virtual (non-physical)
      output, even if nothing consumes it yet (Phase 5 dependency).
- [ ] `tests/` suite (unit + integration + HIL) passing per
      `../docs/testing/phase-1-test-plan.md`.

## Structure
```
phase-1-shell-compositor/
├── continuumwm/            Wayland compositor (wlroots-based, ADR 0005)
├── shell-skeleton/
│   ├── menubar/
│   ├── dock/
│   ├── window-manager/
│   └── virtual-desktops/
├── app-runtime/            ContinuumKit sandbox + Flatpak compatibility
└── tests/{unit,integration,hil}/
```
