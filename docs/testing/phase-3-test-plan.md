# Phase 3 Test Plan — Clipboard, Handoff, Drop (Public Alpha)

**Covers:** `phase-3-clipboard-handoff-drop/`

- **Unit:** clipboard content-type serialization, Handoff state
  tracking, Drop transfer chunking/resume logic.
- **Integration:** end-to-end clipboard round-trip and file-drop over
  the emulated harness; Handoff resume affordance appearing in the Dock
  after a simulated phone-side app-state broadcast.
- **Protocol:** AV-scan-on-receive path for `continuum-dropd` (must
  reject a malicious payload before it reaches the Files app); large-file
  transfer resume after simulated connection loss.
- **HIL:** clipboard sync latency measurement against the KPI (≤500ms
  median on local network, Section 16); file-transfer throughput against
  the ≥80% of raw Wi-Fi Direct link speed KPI.
- **Exit gate:** copy/paste, app/document handoff, and drag-and-drop file
  transfer all work end-to-end with an Android phone (Phase 3
  milestone) — this is also the public alpha gate, so security review of
  the three daemons' sandboxing is required before exit, not just
  functional passing.

Skeleton: `phase-3-clipboard-handoff-drop/tests/`.

## Skeleton index (Session 40)

| File (under `phase-3-clipboard-handoff-drop/tests/`) | Level | Covers |
|---|---|---|
| `unit/clipboardd_test.rs` | unit | Loop guard, ordering, size limit, sensitive content, session loss |
| `unit/handoffd_test.rs` | unit | Replace/expire/clear, payload cap and opacity, capability gate, fallback |
| `unit/dropd_test.rs` | unit | Filename sanitization, quarantine, digest, scan policy, resume, quotas |
| `protocol/session_loss_per_daemon_test.md` | protocol | Session-loss behavior for all three daemons (exit criterion) |
| `protocol/fuzz_inputs_test.md` | protocol | Malformed phone input against the three schemas |
| `protocol/dropd_malicious_payload_test.md` | protocol | EICAR, oversize, resume, no resume past a rotated trust anchor |
| `integration/clipboard_roundtrip_test.md` | integration | Round trip within the 500 ms budget, type negotiation |
| `integration/handoff_dock_resume_test.md` | integration | Dock affordance lifecycle |
| `integration/drop_end_to_end_test.md` | integration | Quarantine-first delivery, drag-and-drop, resume |
| `hil/clipboard_latency_kpi_test.md` | HIL | Median <= 500 ms KPI per device |
| `hil/drop_throughput_kpi_test.md` | HIL | >= 80% of raw link speed KPI per device pair |

Also: `cross-cutting/security/phase-3-daemon-hardening.md` (design-time
threat model). All entries are skeletons. Integration, protocol and HIL
entries are blocked on the pairing harness and Tier-1 hardware (P0-1).
