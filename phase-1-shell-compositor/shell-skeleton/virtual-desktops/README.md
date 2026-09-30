# Virtual Desktops (Spaces)

Multiple desktop workspaces; gesture- and keyboard-triggered switching.

## Design
```rust
pub struct VirtualDesktop {
    id: DesktopId,
    windows: Vec<SurfaceId>,   // membership; a window belongs to exactly one desktop
}

pub struct DesktopManager {
    desktops: Vec<VirtualDesktop>,
    active: DesktopId,
}

impl DesktopManager {
    fn switch_to(&mut self, id: DesktopId) {
        // 1. hide all surfaces on the currently-active desktop
        // 2. show all surfaces on the target desktop
        // 3. update self.active
        // Damage tracking (../../continuumwm/src/surface/) handles the
        // actual recomposite; this just changes visibility state.
    }
}
```

Consumed by:
- `../window-manager/` for per-desktop window membership.
- `../dock/` for the Mission-Control-equivalent overview.

## Status
Interface-level design only.
