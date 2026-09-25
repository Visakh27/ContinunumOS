# Package layout (planned)

- `discovery/` — BLE advertisement/scanning, mDNS (mirrors continuumd's discovery module)
- `pairing/` — Ed25519 handshake + QR/numeric confirmation UI (Phase 2)
- `daemon/` — foreground service hosting the persistent QUIC session +
  feature-daemon protocol handlers, added incrementally per phase:
  clipboard/handoff/drop (Phase 3), phone/notifications/camera (Phase 4),
  control/hotspot (Phase 5)

Kotlin source files not yet written — interface-level planning only.
