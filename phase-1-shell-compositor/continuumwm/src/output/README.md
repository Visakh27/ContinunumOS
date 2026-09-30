# Output Management

Owns physical **and virtual** display outputs. The virtual-output type
is a Phase 1 deliverable even though nothing consumes it until Phase 5's
`continuum-controld` (ADR 0005 — retrofitting a virtual output type
after the pipeline is built around physical displays only is expected
to be substantially more expensive than designing it in now).

## Design

```
OutputManager
├── PhysicalOutput   — backed by a real DRM/KMS connector
│   ├── mode setting (resolution, refresh rate)
│   ├── hot-plug detection (feeds ../../../cross-cutting/external-display/)
│   └── GPU-accelerated scanout buffer
└── VirtualOutput    — backed by an off-screen render target, no DRM connector
    ├── same OutputManager API surface as PhysicalOutput (this is the
    │   point: Universal Control's session code in Phase 5 should not
    │   need to know it's talking to a virtual output vs. a real one)
    ├── frame buffer readback path (for continuum-controld to stream to
    │   the phone, Phase 5)
    └── lifecycle: created per Universal Control session, torn down on
        session end — NOT a persistent output like a physical monitor
```

## Interface sketch (Rust, not yet implemented)

```rust
pub trait Output {
    fn id(&self) -> OutputId;
    fn current_mode(&self) -> OutputMode;
    fn set_mode(&mut self, mode: OutputMode) -> Result<(), OutputError>;
    fn scanout_buffer(&self) -> &dyn ScanoutBuffer;
}

pub struct PhysicalOutput { /* DRM connector handle, EDID-derived modes */ }
pub struct VirtualOutput  { /* off-screen render target, no connector */ }

// Both implement Output — continuum-controld (Phase 5) programs against
// this trait, never against PhysicalOutput/VirtualOutput directly.
```

## Multi-monitor
`OutputManager` holds a `Vec<Box<dyn Output>>`; hot-plug events add/remove
physical outputs at runtime. Window placement across outputs is the
window-manager's concern (`../../shell-skeleton/window-manager/`), not
this module's.

## Status
Interface-level design only — no implementation. Feeds
`../../tests/unit/` (see this session's test additions) and is the
direct dependency `continuum-controld` (`../../../phase-5-control-hotspot-winapp/continuum-controld/`)
will build on.
