# Testing Strategy — Overview

Continuum OS testing spans five levels, mapped to Section 13 of the
source plan plus additions. Every phase folder has a `tests/` directory
implementing the levels relevant to that phase's deliverables, and a
`docs/testing/phase-N-test-plan.md` describing what's covered and why.

## Levels

1. **Unit tests** — per-module, run on every commit, no hardware or
   network required. Rust: `cargo test` per crate. Kotlin (Android
   Companion): JUnit + MockK. Shell/build scripts: `bats`.
2. **Integration tests** — multiple modules together, still no physical
   hardware required (e.g. two `continuumd` instances in emulated
   containers pairing over a virtual network). Run on every merge to
   main.
3. **Protocol tests** — schema and wire-compatibility tests for every
   continuity daemon's protobuf contract: forward/backward compatibility
   across Companion versions, malformed-input fuzzing, session-loss/resume
   behavior (per `../architecture/06-continuity-subsystem.md`). Required
   before any daemon is considered feature-complete.
4. **Hardware-in-the-loop (HIL) tests** — run nightly against the
   physical Tier-1 certified device list plus a rotating Tier-2 sample:
   boot, driver, and continuity regression. Also covers a physical
   Android OEM matrix (Samsung, Pixel, OnePlus, Xiaomi, Motorola) for
   Companion compatibility.
5. **Security tests** — independent penetration testing of the
   verified-boot chain, sandbox-escape testing, and the continuity
   pairing/relay protocol before every major release; continuous fuzzing
   of daemon protocol parsers (folds into level 3 for the daemon-specific
   piece, plus dedicated pen-test engagements pre-release).

## Update-path testing (cross-cutting)

Simulate power loss and corrupted updates mid-flash to validate A/B
rollback reliability before every release train — not phase-specific,
runs against every release candidate from Phase 0 onward once the A/B
mechanism exists.

## What "running and debugging is deferred" means for these test plans

Per current project instruction, test *plans* and test *skeletons*
(file structure, test-case names, fixtures, assertions written against
not-yet-implemented interfaces) are produced now; actually executing
them against a built system is deferred. Each skeleton is written so
that, once the corresponding implementation exists, filling in the
skeleton's body is the only remaining step — the test's intent,
inputs, and pass/fail criteria are already specified.

## Test environment

See `test-environment-setup.md` for what a contributor needs (toolchains,
emulated pairing harness, reference device access) to actually run these
suites once implementations land.

## Per-phase test plans

- `phase-0-test-plan.md`
- `phase-1-test-plan.md`
- `phase-2-test-plan.md`
- `phase-3-test-plan.md`
- `phase-4-test-plan.md`
- `phase-5-test-plan.md`
- `phase-6-test-plan.md`
