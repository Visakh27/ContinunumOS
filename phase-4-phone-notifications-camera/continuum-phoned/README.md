# continuum-phoned

Notification mirroring, SMS/RCS, call audio routing, surfaced through
the native OS notification center and a built-in Phone app.

## Planned modules
- `src/notifications/` — mirroring, filter/dedup, dismiss-sync (dismiss
  on one side dismisses on the other)
- `src/messaging/` — SMS/RCS sync, ordering under out-of-order delivery
- `src/call_audio/` — routing through `../../cross-cutting/audio-stack/`

Skeleton only.
