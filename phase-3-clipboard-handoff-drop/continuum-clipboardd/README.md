# continuum-clipboardd

Bidirectional clipboard sync (text, image, file reference) using an
OS-level clipboard hook, not a polling loop. Wire schema: `clipboard.proto`.
Architecture: `../../docs/architecture/06-continuity-subsystem.md`.

## Modules
- `src/hook/`: subscribes to the compositor's clipboard/selection events
  (Wayland data-control). Event-driven, so no polling latency and no
  wasted wakeups (battery KPI).
- `src/sync/`: serializes events, sends via `continuumd`'s D-Bus API
  (`../../phase-2-continuumd-pairing/continuumd/src/dbus_api/`), applies
  inbound events, handles acks.
- `src/content_types/`: per-kind handling and limits.

## Design decisions
1. **Loop prevention.** Applying a remote clipboard write triggers the
   local hook. Every applied remote event records its `event_id`; the
   hook ignores a change whose content matches the last applied
   `event_id`. Without this, two devices echo each other forever.
2. **Ordering.** Near-simultaneous copies on both devices resolve by
   `lamport_clock`, ties broken by `origin_device_id`. Last-writer-wins,
   no merging.
3. **Size limits.** Inline cap for TEXT and IMAGE (initial value 1 MiB,
   configurable). Anything larger becomes a `FILE_REFERENCE` and moves
   via `continuum-dropd`; a clipboard sync must never block the session
   with a large payload.
4. **Sensitive content.** If the source app marks the selection
   concealed (password managers), the event is not synced unless the
   user enables it, and is never logged or put in telemetry. Telemetry
   carries only the operational metadata in
   `../../cross-cutting/telemetry-privacy/`.
5. **Session loss.** Fire-and-forget per event. Nothing to resume; an
   event dropped while `Lost` is not replayed, since a stale clipboard
   overwriting a newer local copy is worse than a missed sync.
6. **Privilege.** Needs clipboard read/write only. No input injection,
   no network socket, no filesystem beyond its runtime directory.

## Status
Design and schema only. No implementation.
