# App Runtime (ContinuumKit Sandbox)

Sandboxed, capability-scoped app runtime per ADR 0004
(`../../docs/adr/0004-app-sandbox-format.md`). Gets Flatpak-format Linux
apps running with minimal porting.

## Submodules
- `src/sandbox/` — the Flatpak-derived (OSTree + bubblewrap-style)
  process sandbox itself.
- `src/capability-broker/` — macOS-TCC-style per-capability permission
  prompts and grant storage (`../../docs/architecture/07-security-permission-model.md`).
- `src/accessibility-tree/` — the ContinuumKit accessibility API (see
  `../../cross-cutting/accessibility/README.md`) — built here, in the
  runtime every app goes through, so accessibility is free for any app
  using standard ContinuumKit widgets rather than a per-app
  reimplementation.

## Capability model sketch
```rust
pub enum Capability {
    Filesystem(FsScope),      // no access by default; explicit path scope on grant
    Camera,
    Microphone,
    Network,
    ContinuityHandoff,        // ContinuumKit Handoff API (Phase 3 dependency, additive)
    ContinuityDbusSurface(ContinuityFeature), // e.g. clipboard read/write, gated per-feature
}

pub struct CapabilityBroker {
    grants: HashMap<(AppId, Capability), GrantState>, // Granted | Denied | NotYetAsked
}

impl CapabilityBroker {
    fn request(&mut self, app: AppId, cap: Capability) -> GrantState {
        // NotYetAsked -> show prompt -> record Granted/Denied -> return it
        // Granted/Denied (already decided) -> return immediately, no re-prompt
        todo!()
    }
}
```

Continuity-related capabilities (`ContinuityHandoff`,
`ContinuityDbusSurface`) are modeled now, even though the daemons they
gate (`continuum-handoffd` et al.) don't exist until Phase 3+ — so the
broker's capability enum doesn't need a breaking change later, only
additions.

## Accessibility-tree API sketch
```rust
pub trait Accessible {
    fn role(&self) -> AccessibilityRole;       // button, text field, list, etc.
    fn label(&self) -> String;
    fn children(&self) -> Vec<Box<dyn Accessible>>;
}
// Standard ContinuumKit widgets (button, list, text field, ...) implement
// this automatically; a custom-drawn widget must implement it manually —
// documented as a required step in the (future) ContinuumKit dev docs,
// feeds ../../../cross-cutting/developer-portal/.
```

## Status
Interface-level design for all three submodules. No implementation.
