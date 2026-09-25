# continuum-clipboardd

Bidirectional clipboard sync (text/image/file), OS-level clipboard hook.
See `../../docs/architecture/06-continuity-subsystem.md`.

## Planned modules
- `src/hook/` — OS-level clipboard hook (not a polling loop)
- `src/sync/` — content-type serialization + sync over `continuumd`'s
  D-Bus API (`../../phase-2-continuumd-pairing/continuumd/`)
- `src/content_types/` — text/image/file handling, size limits, MIME
  negotiation

Skeleton only.
