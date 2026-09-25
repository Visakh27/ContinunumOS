# A/B Atomic Update Mechanism

Implements `../../../docs/architecture/02-boot-install-update.md`'s
update strategy: background, delta-compressed, signed-manifest-verified
updates to the inactive partition, activated only on next reboot, with
automatic rollback on boot failure.

## State machine

```
        ┌─────────────┐
        │   IDLE       │  (slot A active, slot B inactive)
        └──────┬───────┘
               │ update available (signed manifest fetched)
               ▼
        ┌─────────────┐
        │ DOWNLOADING  │  delta-compressed payload → inactive slot
        └──────┬───────┘
               │ payload verified against signed manifest
               ▼
        ┌─────────────┐
        │  VERIFIED    │  inactive slot has a verified, unbooted image
        └──────┬───────┘
               │ reboot triggered (user or scheduled)
               ▼
        ┌─────────────┐
        │  BOOTING_NEW │  bootloader flips active slot, boot attempted
        └──────┬───────┘
       success  │  failure (crash-loop / boot-health check fails)
       ┌────────┴────────┐
       ▼                 ▼
┌─────────────┐   ┌──────────────┐
│  ACTIVE      │   │ ROLLING_BACK │  bootloader flips back to old slot
│ (new slot)   │   └──────┬───────┘
└─────────────┘          ▼
                   ┌─────────────┐
                   │   IDLE       │  (back on old slot; update marked failed)
                   └─────────────┘
```

## Boot-health check
A new slot is only considered successfully booted (and the old slot
released for the *next* update cycle) after it reports healthy past a
defined checkpoint (e.g. shell/compositor reached login, or — for the
Phase 0 minimal image — getty reachable) within a bounded number of boot
attempts (default: 3, configurable). This is what makes rollback
*automatic* rather than requiring user intervention.

## Components
- `state-machine.md` — the above, plus the exact transition triggers and
  timeout values (values TBD, structure fixed).
- `continuum-ab-updated/` — CLI/daemon skeleton (Rust) implementing the
  state machine; not yet implemented, interface-only.
- `update-manifest.schema.json` — signed-manifest schema (version, delta
  payload hash, minimum-compatible-version, release notes reference).

## Staged rollout (Section 4.3)
Rollout percentage gating (1%→10%→50%→100%) and the crash/boot-failure
telemetry halt condition are a server-side (update-service) concern, not
part of the on-device state machine above — tracked as a Phase 6
distribution-infrastructure deliverable
(`../../../phase-6-oem-certification-ga/`), consuming the same boot-health
telemetry defined in `../../../cross-cutting/telemetry-privacy/`.
