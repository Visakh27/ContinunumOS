# Menu Bar

Global menu bar: system-wide app menu, clock, and the **Continuum
Center** status area (paired Android device(s), battery, signal,
continuity feature toggles).

## Continuum Center — dependency note
Continuum Center's *content* (device status, feature toggles) depends on
`continuumd`'s local D-Bus API (`../../../phase-2-continuumd-pairing/continuumd/src/dbus_api/`),
which doesn't exist until Phase 2. This Phase 1 session builds:
1. The menu-bar shell itself (system-wide app menu, clock) — fully
   independent of continuity, buildable and testable now.
2. Continuum Center's UI *shape* (layout, toggle-row component, empty/
   "no device paired" state) with a **mock data source** standing in for
   the real D-Bus API, so Phase 2 only has to swap the data source, not
   build the UI from scratch.

## Interface sketch
```rust
pub trait ContinuityStatusSource {
    fn paired_devices(&self) -> Vec<PairedDeviceStatus>;
    fn feature_toggle_state(&self, feature: ContinuityFeature) -> bool;
    fn set_feature_toggle(&mut self, feature: ContinuityFeature, enabled: bool);
}

// Phase 1: MockStatusSource (always reports "no device paired")
// Phase 2: DbusStatusSource, backed by continuumd's real API — same trait
pub struct MockStatusSource; // returns empty paired_devices(), all toggles off/disabled
```

## Status
Menu-bar shell: interface-level design. Continuum Center: UI shape
designed against `ContinuityStatusSource`, mock implementation planned,
real implementation blocked on Phase 2.
