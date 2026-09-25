# Accessibility

See `../../docs/architecture/04-desktop-shell-ux.md` and
`09-additions-and-gaps.md`. Starts Phase 1 (must be a ContinuumKit API,
not a shell-only bolt-on) and formally gated at Phase 6
(`../../docs/testing/phase-6-test-plan.md`).

## Scope (initial)
- Screen reader (ContinuumKit accessibility-tree API every native app
  gets for free by using standard widgets).
- Switch/scanning input support.
- System-wide captioning hook for `continuum-phoned` call audio (Phase 4).
- High-contrast and reduced-motion modes as ContinuumShell-level
  settings, honored by the compositor (`../../phase-1-shell-compositor/`)
  and exposed to apps via ContinuumKit.

## Status
Design scope only at this stage — implementation begins with the Phase 1
ContinuumKit accessibility-tree API, tracked in
`../../phase-1-shell-compositor/app-runtime/README.md`.
