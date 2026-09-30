# Dock

Running-app indicators, Spaces/virtual-desktop support, Mission-Control-
equivalent overview (gesture- and keyboard-triggered).

## Handoff resume affordance — dependency note
Same pattern as Continuum Center (`../menubar/`): the Dock's Handoff
resume affordance depends on `continuum-handoffd`
(`../../../phase-3-clipboard-handoff-drop/continuum-handoffd/`), which
doesn't exist until Phase 3. Built now against a mock source:

```rust
pub trait HandoffSource {
    fn active_handoff(&self) -> Option<HandoffCandidate>; // None until Phase 3 wires the real source
}
pub struct MockHandoffSource; // always returns None
```

## Running-app indicators
Sourced from the window manager's surface-to-client mapping
(`../window-manager/`) — one indicator per distinct client with at least
one mapped surface, not one per window.

## Mission-Control-equivalent overview
Reuses `../virtual-desktops/`'s desktop-switching state; the overview is
a visual mode of the Dock/compositor (all windows shown at reduced
scale, grouped by virtual desktop), not a separate subsystem.

## Status
Interface-level design + mock Handoff source. No implementation.
