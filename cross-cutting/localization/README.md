# Internationalization & Localization

See `../../docs/architecture/04-desktop-shell-ux.md` and
`09-additions-and-gaps.md`.

## IME (Input Method Editor) framework

System service apps opt into via ContinuumKit, not a per-app
reimplementation — mirrors the pattern already used for the
accessibility-tree API (one system-level service, every ContinuumKit
text-input widget gets it for free).

```rust
pub trait InputMethodEditor {
    fn locale(&self) -> Locale;               // e.g. ja-JP, hi-IN
    fn compose(&mut self, raw_input: KeyEvent) -> ImeComposeResult;
    fn candidates(&self) -> Vec<String>;       // candidate window content
    fn commit(&mut self, candidate_index: usize) -> String; // committed text
}

pub struct ImeManager {
    active_ime: Option<Box<dyn InputMethodEditor>>,
    available: Vec<ImeDescriptor>, // installed IMEs, user-selectable
}
```

Every ContinuumKit text-input widget routes raw key events through
`ImeManager` before treating them as committed text — this is a
ContinuumKit-level integration point
(`../../phase-1-shell-compositor/app-runtime/`), not something each app
wires up itself.

## RTL layout support

ContinuumKit's layout system needs a layout-direction flag threaded
through every container widget:

```rust
pub enum LayoutDirection { Ltr, Rtl }
// Set from Locale; container widgets (stacks, lists) mirror their
// child-ordering and padding/margin interpretation when Rtl.
```

Affects `../../phase-1-shell-compositor/shell-skeleton/` components
directly — the menu bar, Dock, and window-manager's edge-snap logic
(`window-manager/README.md`'s "near_left_edge()"/"near_right_edge()"
checks) all need to be direction-aware rather than hardcoded to
left=start.

## Locale-aware formatting
Date/number/currency formatting as a ContinuumKit utility API, consumed
by first-party apps' string-resource pipeline
(feeds `../../phase-6-oem-certification-ga/localization/`'s eventual
audit).

## Status
IME and RTL interfaces designed at the ContinuumKit integration point.
No concrete IME engine (e.g. a specific CJK input method) selected or
implemented — that's a substantial scope of its own, deferred past this
scaffolding pass. Flagged explicitly rather than silently skipped.
