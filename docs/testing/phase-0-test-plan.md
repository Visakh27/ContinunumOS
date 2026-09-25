# Phase 0 Test Plan — Foundation & Kernel Bring-Up

**Covers:** `phase-0-foundation/`

- **Unit** (`phase-0-foundation/tests/unit/`): A/B update state machine
  transitions (all 8 transitions in `boot/ab-update/state-machine.md`
  covered individually); kernel config fragment merge correctness.
- **Integration** (`.../tests/integration/`): build-system image
  manifest completeness and superset-across-phases checks — no physical
  hardware.
- **HIL** (`.../tests/hil/`): Secure Boot tamper detection (3 cases:
  clean boot, tampered bootloader, tampered root partition); A/B
  rollback survival under physical power loss mid-flash.
- **Update-path testing** (cross-cutting, per `00-testing-strategy.md`):
  simulated power loss / corrupted update, both in CI
  (`phase-0-foundation/ci/pipelines/release-train.yaml`) and physically
  (`phase-0-foundation/tests/hil/test_ab_rollback_on_power_loss.md`).

## Exit gate
All boxes in `phase-0-foundation/README.md`'s exit-criteria checklist,
with every test above passing (not just written) against the reference
image. Current status: **all test files are skeletons** — none are
runnable yet, since the implementations they test don't exist yet. See
`phase-0-foundation/tests/README.md`.

## Traceability
Every HIL test case in `phase-0-foundation/tests/hil/` maps to a Section
13 (source plan) requirement:
- Tamper detection → "independent penetration test of the verified-boot
  chain" (pre-release requirement, exercised continuously in Phase 0 as
  a regression guard, not just pre-release).
- Power-loss rollback → "simulate power loss and corrupted updates
  mid-flash to validate A/B rollback reliability before every release
  train."
