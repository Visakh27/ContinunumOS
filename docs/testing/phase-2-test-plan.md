# Phase 2 Test Plan — continuumd Core & Pairing

**Covers:** `phase-2-continuumd-pairing/`

- **Unit:** BLE/mDNS discovery state machine, Ed25519 pairing handshake
  logic, D-Bus API surface (mocked transport).
- **Integration:** two-endpoint pairing over the emulated harness
  (`docs/testing/test-environment-setup.md`), including the negative
  cases — wrong PIN/QR, timeout, re-pair after key rotation.
- **Protocol:** the `continuumd`↔Companion wire schema — version
  negotiation, malformed-frame handling, and the session-loss/resume
  behavior flagged in `../architecture/06-continuity-subsystem.md`.
- **HIL:** physical Android OEM matrix pairing against Tier-1 reference
  hardware, night after every merge that touches the pairing path.
- **Exit gate:** a Continuum OS machine pairs with an Android phone and
  shows a live "Connected" status (Phase 2 milestone), with pairing
  success rate tracked against the KPI target (≥98% first-attempt,
  Section 16) even though the KPI itself is only formally gated at GA.

Skeleton: `phase-2-continuumd-pairing/tests/`.
