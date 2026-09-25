# Phase 0 Test Suite

See `../../docs/testing/00-testing-strategy.md` for the level
definitions and `../../docs/testing/phase-0-test-plan.md` (to be added
in this same session, see below) for what Phase 0 specifically covers.

- `unit/` — state-machine and parsing logic, no hardware/network.
- `integration/` — build-system output validation, no physical hardware.
- `hil/` — physical Tier-1 reference device required.

All test files below are **skeletons**: test names, fixtures, and
assertions are specified against the interfaces defined in
`../kernel/`, `../boot/`, and `../build-system/`; bodies are
`unimplemented!`/`TODO` pending the corresponding implementation, per
current project scope (running/debugging deferred).
