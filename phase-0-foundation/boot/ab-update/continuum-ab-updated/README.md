# continuum-ab-updated

Rust daemon/CLI implementing the A/B update state machine in
`../README.md`. Interface skeleton only — no implementation logic yet.

## Planned crate layout
- `src/main.rs` — CLI entry point (`continuum-ab-updated status|check|apply|rollback`)
- `src/state_machine.rs` — the state machine itself, unit-tested in isolation from I/O
- `src/manifest.rs` — signed-manifest parsing/verification against `../update-manifest.schema.json`
- `src/slot.rs` — partition-slot abstraction (which slot is active/inactive, flip operation)
- `src/boot_health.rs` — boot-health checkpoint reporting, consumed by the bootloader's rollback trigger
