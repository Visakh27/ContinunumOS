# A/B Update State Machine — Transition Detail

Complements the diagram in `README.md`. Table form for implementers.

| From | Event | To | Notes |
|---|---|---|---|
| IDLE | manifest fetched & signature valid | DOWNLOADING | Manifest schema: `update-manifest.schema.json` |
| DOWNLOADING | manifest signature invalid | IDLE | Reject, log, no partial state persisted |
| DOWNLOADING | payload downloaded, hash mismatch | IDLE | Retry policy TBD; must not proceed to VERIFIED |
| DOWNLOADING | payload downloaded, hash matches | VERIFIED | Inactive slot now holds a verified image |
| VERIFIED | reboot triggered | BOOTING_NEW | Bootloader flips active-slot pointer atomically |
| BOOTING_NEW | boot-health checkpoint reached within N attempts | ACTIVE | N default 3, configurable; old slot retained one more cycle before reuse |
| BOOTING_NEW | N attempts exceeded without checkpoint | ROLLING_BACK | Bootloader-level, not OS-level — must work even if the new OS never gets far enough to run its own rollback logic |
| ROLLING_BACK | slot pointer flipped back | IDLE | Update marked failed; telemetry event emitted (`cross-cutting/telemetry-privacy/`) |
