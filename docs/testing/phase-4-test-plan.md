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

## Skeleton index (Session 50)

| File (under `phase-4-phone-notifications-camera/tests/`) | Level | Covers |
|---|---|---|
| `unit/phoned_notifications_test.rs` | unit | Filtering, redaction, dedup, no-echo dismiss, rate limit, icon safety, no content in logs |
| `unit/phoned_calls_test.rs` | unit | Call state, audio routing rules, stale state on session loss, dial only from user action |
| `unit/camerad_test.rs` | unit | User-gesture start, contention placeholder, no auto resume, indicator source, no disk writes |
| `protocol/notification_dismiss_sync_test.md` | protocol | Dismiss both ways, dedup, SMS ordering |
| `protocol/phoned_session_loss_test.md` | protocol | Session-loss behavior for notifications, calls, camera |
| `protocol/fuzz_inputs_test.md` | protocol | `phoned.proto`, `camera.proto`, PNG icon and H.264 decode fuzzing |
| `integration/notification_roundtrip_test.md` | integration | Mirror, dismiss both ways, redaction, DND |
| `integration/message_reply_test.md` | integration | Reply through notification action, failure when notification is gone |
| `integration/call_audio_routing_test.md` | integration | Routing rules with mocked Bluetooth and audio |
| `integration/camera_stream_test.md` | integration | Portal listing, consent, indicator, contention, no auto resume |
| `integration/scan_to_desktop_test.md` | integration | Scan through the drop path, cancel cleanup |
| `hil/call_audio_hfp_test.md` | HIL | Real calls over HFP; decides whether ADR 0007 stands |
| `hil/camera_latency_quality_test.md` | HIL | Latency, drops, thermals; sets the latency budget |
| `hil/notification_survives_background_test.md` | HIL | OEM battery management vs. the notification listener |
| `hil/tier2_sample_pass_test.md` | HIL | Tier-2 sample pass and support matrix |

Also: `cross-cutting/security/phase-4-daemon-hardening.md` (design-time
threat model) and `phase-4-phone-notifications-camera/beta-readiness-checklist.md`.
All entries are skeletons; protocol, integration and HIL entries are
blocked on the pairing harness, Tier-1 hardware (P0-1) and the OEM phone matrix.
