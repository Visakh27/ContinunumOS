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
Interface sketch now exists at the intended hook-in point:
`../../phase-1-shell-compositor/app-runtime/src/accessibility-tree/`
(the `Accessible` trait, implemented automatically by standard
ContinuumKit widgets). No implementation yet — this session established
where the API lives and its shape; the screen reader, switch control,
and captioning consumers of this tree are not yet designed.
