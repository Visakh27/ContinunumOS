# Audio Stack

Not specified anywhere in the source plan, yet `continuum-phoned`
(Phase 4) requires call-audio routing, and any general-purpose desktop
needs system audio. See `../../docs/architecture/04-desktop-shell-ux.md`
and `09-additions-and-gaps.md`.

## Scope (initial)
- PipeWire-based audio/video stack (handles both audio routing and the
  video-capture path `continuum-camerad`, Phase 4, will need).
- Output/input device switching UI in ContinuumShell's menu bar
  (identified gap in `04-desktop-shell-ux.md`).
- Call-audio routing API for `continuum-phoned` to route phone-call
  audio through desktop speakers/mic without the user manually switching
  devices.

## Status
Not started — starts Phase 1 (general desktop audio), extended in Phase 4
for call-audio routing.
