# Window Manager

Window placement, focus, resize/snap behavior on top of ContinuumWM's
surface layer (`../../continuumwm/src/surface/`). Implements
`SurfaceObserver` (see that module's interface sketch) to react to
client commits/destroys.

## Responsibilities
- Placement: default spawn position, multi-monitor-aware (queries
  `../../continuumwm/src/output/` for available outputs).
- Focus: click-to-focus by default (not focus-follows-hover), matching
  the macOS-parallel UX goal in `../../../docs/architecture/04-desktop-shell-ux.md`.
- Snap/tile: drag-to-edge half/quarter snapping.
- Per-virtual-desktop window membership (coordinates with
  `../virtual-desktops/`).

## Layout algorithm sketch
```
on_drag_end(window, cursor_position):
    if cursor_position.near_left_edge():
        snap(window, LeftHalf)
    elif cursor_position.near_right_edge():
        snap(window, RightHalf)
    elif cursor_position.near_top_edge():
        maximize(window)
    elif cursor_position.near_corner():
        snap(window, Quarter(corner))
    else:
        # free placement, no snap
        commit_position(window, cursor_position)
```
Exact edge-detection thresholds and corner-snap regions are a UX-tuning
detail, not an architecture decision — left as implementation parameters,
not specified further here.

## RTL awareness (added Session 17)
`near_left_edge()`/`near_right_edge()` above are placeholder names
written before layout-direction was designed
(`../../../cross-cutting/localization/README.md`). Real implementation
must resolve against `LayoutDirection` — "snap to start edge" vs. "snap
to end edge" — not hardcode left=start.

## Status
Interface-level design + layout algorithm sketch only. Matches the
existing unit test skeleton `../../tests/unit/window_manager_layout_test.md`
written in an earlier session — that file's four test names
(`test_snap_left_half_on_drag_to_left_edge`, etc.) map directly onto the
branches in the sketch above.
