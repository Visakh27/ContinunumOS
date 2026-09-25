# Recovery Mode

See `../../docs/architecture/02-boot-install-update.md#24-recovery-mode-added`
for the architectural rationale (A/B rollback only covers *failed
updates*; this covers user-initiated recovery).

## Entry points
- Boot-time key combination (device-specific, documented per Tier-1
  device in the OEM cert kit).
- "Boot to Recovery" option from the installer USB.
- Triggered automatically after N consecutive boot-health failures on
  *both* A/B slots (extremely rare — normally A/B rollback resolves a
  single-slot failure; both-slots-bad implies a deeper issue).

## Recovery environment requirements
- Lives on a separate small partition, outside the A/B pair, so a
  corrupt main OS can never take recovery down with it.
- Covered by verified boot like the main OS (a common attack target
  otherwise — see `../security/supply-chain.md`).
- Minimal UI: reinstall OS (preserve or wipe user data), manual slot
  rollback, wipe + re-pair continuity trust anchors, export diagnostic
  logs to external USB.

## Structure (planned)
```
recovery-mode/
├── README.md              (this file)
├── recovery-image/        Minimal recovery environment image spec
├── ui/                    Minimal recovery UI skeleton
└── tests/                 Recovery-path test plan (entry, each action, exit)
```
Subdirectories not yet populated — this session established the design
doc; implementation skeleton is a follow-up session per `../../SESSION_LOG.md`.
