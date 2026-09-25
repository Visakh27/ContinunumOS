# ADR 0005: Compositor Protocol & Base

**Status:** Accepted

## Context
Need a Wayland compositor with GPU-accelerated multi-monitor support,
plus a virtual-display output for Universal Control (`continuum-controld`,
Phase 5).

## Decision
Build "ContinuumWM" on **wlroots** rather than a from-scratch Wayland
compositor implementation or an existing full desktop environment's
compositor (e.g. KWin, Mutter) reused as-is.

## Consequences
- wlroots handles low-level protocol compliance and GPU buffer
  management, letting Phase 1 engineering focus on ContinuumShell's
  actual differentiation (menu bar, Dock, virtual desktops) rather than
  re-solving solved problems.
- The virtual-display output required for `continuum-controld` (Phase 5)
  must be designed into the compositor's output-management code in
  Phase 1, even though it isn't consumed until Phase 5 — retrofitting a
  virtual output type after the compositor's output pipeline is built
  around physical displays is expected to be substantially more
  expensive. Flagged as a Phase 1 requirement in
  `phase-1-shell-compositor/continuumwm/README.md`.
