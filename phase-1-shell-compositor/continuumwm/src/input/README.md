# Input Routing

Routes input events (pointer, keyboard, touch) to the focused surface
under normal desktop use. **This module is extended, not replaced, in
Phase 5** when `continuum-controld` needs to inject input at the
compositor level for Universal Control sessions
(`../../../phase-5-control-hotspot-winapp/continuum-controld/`).

## Privilege boundary (design constraint, binding on the Phase 5 work)
Per the daemon design principles in
`../../../docs/architecture/06-continuity-subsystem.md`, input
injection must be:
1. **Session-scoped** — an injected event is tagged with the requesting
   Universal Control session ID and can only target that session's
   `VirtualOutput` (`../output/`), never the physical outputs or another
   session.
2. **Revocable instantly** — ending a session (phone disconnect, user
   toggle-off) must synchronously stop event routing for that session's
   injection path; no queued events processed after revocation.
3. **Distinguishable in the input event stream** — every consumer
   downstream of this module (apps, the window manager) must be able to
   tell an injected event from a physically-local one, for any future
   security/audit need, even though most consumers won't care in
   practice.

This module's interface is designed now (Phase 1) so Phase 5 only has
to *implement* the injection path, not redesign the routing core to
retrofit these constraints.

## Interface sketch (not yet implemented)
```rust
pub enum InputSource {
    Local,                         // physical device on this machine
    Injected { session: SessionId }, // continuum-controld, Phase 5
}

pub struct InputEvent {
    pub source: InputSource,
    pub kind: InputEventKind,      // pointer move/click, key press, touch, etc.
    pub target_output: OutputId,   // must be a VirtualOutput for Injected events — enforced, not just convention
}
```

## Status
Interface-level design + privilege-boundary constraints only.
