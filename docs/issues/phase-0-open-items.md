# Phase 0 Open Items (filed at Exit Review, Session 11)

Numbered issues, in the style this repo will use going forward for
tracked gaps (a real issue tracker is a later infrastructure choice —
this file is the interim log). Each has a status and the session/phase
expected to close it.

| # | Title | Detail | Status | Target |
|---|---|---|---|---|
| P0-1 | No Tier-1 reference device selected | `phase-0-foundation/kernel/VERSION_PIN`, the build-system MACHINE variable, and every HIL test file reference "Tier-1 reference hardware" without a concrete device. Blocks all HIL execution. | Open → partially addressed this session, see `../../phase-0-foundation/TIER1_REFERENCE_HARDWARE.md` (provisional draft) | Needs real hardware selection before any HIL test can run |
| P0-2 | ADR 0002 (init system) still open | systemd assumed as working default; not formally closed. Low risk to keep open into Phase 1 since daemon unit-file abstraction already isolates the decision, but should close before Phase 2 daemons multiply the number of unit files written against the assumption. | Open | Before Phase 2 |
| P0-3 | ext4 vs btrfs (ADR 0003) provisional | ext4 is the working choice; btrfs snapshot primitives not evaluated against a working A/B implementation because none exists yet. | Open, low urgency | Revisit once `continuum-ab-updated` has a working prototype |
| P0-4 | No CI system selected | `ci/README.md` and all `ci/pipelines/*.yaml` are tool-agnostic placeholders. Nothing can actually run until a concrete system (GitHub Actions/GitLab CI/Buildkite) is chosen. | Open | Before any test in `tests/` can execute |
| P0-5 | Kconfig fragment merge tooling doesn't exist | `tests/unit/kconfig_fragment_merge_test.bats` is entirely `skip`. No script merges `kernel/config/*.kconfig` into a buildable `.config`. | Open | Build-system implementation session |
| P0-6 | Yocto recipes not written | `build-system/layers/*/` are READMEs only, no `.bb` recipe files. The manifest (`phase-0-minimal-image.manifest`) has nothing that consumes it yet. | Open | Build-system implementation session |
| P0-7 | `continuum-ab-updated` is `unimplemented!()` | Compiles to a single panic. All 8 unit test cases in `tests/unit/ab_update_state_machine_test.rs` are `todo!()`. | Open, expected at this stage | Implementation session, post-scaffold |
| P0-8 | Secure Boot / dm-verity scripts are stubs | `sign-bootloader.sh` and `generate-verity-image.sh` print their intended action and exit 0 — no real signing or hash-tree generation happens. HSM integration undesigned beyond the key-hierarchy doc. | Open, expected at this stage | Implementation session, needs HSM vendor decision first |
| P0-9 | Boot-time budget undefined | `tests/hil/test_secure_boot_tamper_detection.md` case 1 references an "expected boot-time budget (budget TBD)" with no number. Needed to make the test's pass/fail line objective. | Open | Set once a reference device exists (P0-1) and a first real boot is measured |
| P0-10 | Recovery-mode implementation not started | `cross-cutting/recovery-mode/README.md` defines the design; `recovery-image/`, `ui/`, `tests/` subdirectories referenced in its "Structure (planned)" section don't exist yet. | Open | Follow-up session, flagged in that README already |
| P0-11 | Broader telemetry schema (beyond boot-health beacon) not started | `cross-cutting/telemetry-privacy/README.md` explicitly scopes this as a Phase 2+ follow-up; noting here so it isn't lost between phase boundaries. | Open, on track (not a Phase 0 exit blocker per its own README) | Before Phase 3 alpha |

## Exit determination

Per `phase-0-foundation/README.md`'s own framing, **none of the exit
criteria are expected to be checked off at this stage** — the phase was
explicitly scoped as design/scaffold, not implementation, for this
project. This review's job is narrower: confirm every exit criterion has
a concrete, unblocked path to completion, and that nothing was
forgotten. Result: all six exit criteria trace to open items above (or
combinations of them) with no orphaned criterion and no criterion
blocked by something outside this repo's control except P0-1 (hardware
selection, a decision for you, not an engineering task) and P0-4/HSM
vendor choice (tooling decisions, also yours to make).

**Recommendation:** P0-1 (Tier-1 device) and P0-4 (CI system) are the
two decisions that unblock the most downstream work — every other open
item either depends on one of them or is independently actionable.
Addressed partially this session (see
`../../phase-0-foundation/TIER1_REFERENCE_HARDWARE.md`); flagging both
explicitly rather than guessing a CI vendor or hardware SKU on your
behalf.
