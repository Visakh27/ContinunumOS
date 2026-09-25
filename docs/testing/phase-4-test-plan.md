# Phase 4 Test Plan — Phone, Notifications, Camera (Public Beta)

**Covers:** `phase-4-phone-notifications-camera/`

- **Unit:** notification-mirroring filter/dedup logic, V4L2 virtual
  device registration/teardown.
- **Integration:** notification round-trip (Android → mirrored →
  dismissed on desktop → dismissed on phone) over the emulated harness;
  call-audio routing path with a mocked audio backend.
- **Protocol:** SMS/RCS message sync ordering and dedup under
  out-of-order delivery; camera stream negotiation and graceful
  degradation when the phone camera is in use by another app.
- **HIL:** real call-audio routing latency and quality on Tier-1
  hardware; V4L2 camera stream tested against a representative set of
  apps (video call app, screenshot/photo tool).
- **Exit gate:** Phase 4 milestone (public beta) — plus Tier-2
  best-effort hardware support expanded per `../architecture/03-driver-hardware-strategy.md`,
  verified by a Tier-2 HIL pass, not just Tier-1.

Skeleton: `phase-4-phone-notifications-camera/tests/`.
