# ContinuumWM

Wayland compositor built on wlroots (ADR 0005:
`../../docs/adr/0005-compositor-protocol.md`).

## Planned module layout
- `src/output/` — physical + **virtual** output management (the virtual
  output is a Phase 1 deliverable per ADR 0005, consumed by
  `continuum-controld` in Phase 5)
- `src/surface/` — window surface lifecycle, damage tracking
- `src/input/` — input event routing (extended in Phase 5 for
  `continuum-controld`'s compositor-level injection — see that phase's
  README for the privilege boundary this must respect)
- `src/protocols/` — Wayland protocol implementations/extensions

Not yet implemented — interface skeleton only, per current project
scope.
