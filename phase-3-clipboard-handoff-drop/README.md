# Phase 3 — Clipboard, Handoff, Drop (M21–M30) — Public Alpha

## Scope (source plan, Section 11)
- Ship `continuum-clipboardd`, `continuum-handoffd` (with a ContinuumKit
  Handoff API for third-party apps), and `continuum-dropd` (Wi-Fi Direct
  transfer, Files app integration).
- Public alpha release to a limited hardware-certified device set.

## Added to scope
- Developer portal / SDK stability policy
  (`../cross-cutting/developer-portal/`) must be ready before this
  phase's public alpha is meaningful to third-party developers.
- Backup/restore (`../cross-cutting/backup-restore/`) should be
  functional before public alpha — first phase where real user data is
  at stake for external users.

## Entry criteria
Phase 2 exit criteria met: live pairing + Continuum Center working.

## Exit criteria
- [ ] Copy/paste, app/document handoff, and drag-and-drop file transfer
      all work end-to-end with an Android phone.
- [ ] Clipboard sync median latency ≤500ms on local network (KPI,
      tracked from this phase even though formally gated at GA).
- [ ] File transfer within 80% of raw Wi-Fi Direct link speed (KPI).
- [ ] AV-scan-on-receive functioning for `continuum-dropd`.
- [ ] Session-loss/resume behavior defined and tested for all three
      daemons (per `../docs/architecture/06-continuity-subsystem.md`).
- [ ] Security review of all three daemons' sandboxing complete —
      required before public alpha, not deferred to Phase 5's audit.
- [ ] `tests/` suite passing per `../docs/testing/phase-3-test-plan.md`.

## Structure
```
phase-3-clipboard-handoff-drop/
├── continuum-clipboardd/
├── continuum-handoffd/
├── continuum-dropd/
└── tests/{unit,integration,protocol}/
```
