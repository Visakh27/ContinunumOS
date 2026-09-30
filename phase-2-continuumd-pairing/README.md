# Phase 2 — continuumd Core & Pairing (M15–M20)

## Scope (source plan, Section 11)
- Implement `continuumd`: BLE/mDNS discovery, pairing handshake
  (QR/numeric + Ed25519), persistent QUIC session.
- Build the Continuum Companion Android app with the matching protocol
  implementation.
- Build Continuum Center UI in the shell for device status and feature
  toggles.

## Added to scope
- Backup & restore groundwork (`../cross-cutting/backup-restore/`) — the
  phone becomes a viable backup destination once `continuumd`'s session
  exists.
- Broader telemetry/diagnostics policy beyond the Phase 0 boot-health
  beacon (`../cross-cutting/telemetry-privacy/`), staged to be ready
  before Phase 3 public alpha.

## Entry criteria
Phase 1 exit criteria met: usable desktop shell running.

## Exit criteria
- [ ] A Continuum OS machine pairs with an Android phone and shows a
      live "Connected" status in Continuum Center.
- [ ] Pairing handshake (Ed25519 + QR/numeric confirmation) implemented
      and covered by `tests/protocol/`.
- [ ] `continuumd`'s local D-Bus API implemented and documented for
      consumption by Phase 3+ feature daemons.
- [ ] `tests/` suite passing per `../docs/testing/phase-2-test-plan.md`.

## Structure
```
phase-2-continuumd-pairing/
├── continuumd/                 Core daemon
├── pairing/                    Handshake protocol, trust-anchor storage
├── companion-android-stub/     Interface stub consumed by android-companion/
├── continuum-center-ui/        Shell UI surfacing device status/toggles
└── tests/{unit,integration,protocol}/
```
