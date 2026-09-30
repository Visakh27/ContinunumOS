# Surface Management

Window surface lifecycle and damage tracking — the layer between the
Wayland protocol implementation (`../protocols/`) and what the
window-manager (`../../shell-skeleton/window-manager/`) arranges on
screen.

## Responsibilities
- Surface creation/destruction tied to client (app) connect/disconnect.
- Damage-region tracking so the compositor only re-composites changed
  screen regions, not the full frame every time (performance-critical
  for the Phase 6 battery/thermal KPIs).
- Buffer commit/ack protocol (standard Wayland surface lifecycle).

## Interface sketch (not yet implemented)
```rust
pub struct Surface {
    id: SurfaceId,
    client: ClientId,
    current_buffer: Option<SurfaceBuffer>,
    damage_regions: Vec<Rect>,
}

pub trait SurfaceObserver {
    fn on_commit(&mut self, surface: &Surface);
    fn on_destroy(&mut self, surface_id: SurfaceId);
}
// WindowManager (../../shell-skeleton/window-manager/) implements
// SurfaceObserver to react to commits/destroys.
```

## Status
Interface-level design only.
