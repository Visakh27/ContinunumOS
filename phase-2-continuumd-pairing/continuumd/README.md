# continuumd

Core continuity daemon. See
`../../docs/architecture/06-continuity-subsystem.md#81-continuumd--the-core-continuity-daemon`.

## Planned module layout
- `src/discovery/` — BLE advertisement/scanning, mDNS
- `src/pairing/` — see `../pairing/` for the protocol spec this implements
- `src/session/` — persistent QUIC/WebRTC session management, including
  the session-loss/resume behavior flagged in the architecture doc
- `src/dbus_api/` — local D-Bus surface consumed by feature daemons +
  shell (no continuity feature talks to the network directly — this is
  the sole exception per the daemon design principles)

Not yet implemented — interface skeleton only.
