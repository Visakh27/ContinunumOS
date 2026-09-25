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
