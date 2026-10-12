# Phase 2 Open Items (Exit Review, Session 31)

| # | Title | Detail | Status | Target |
|---|---|---|---|---|
| P2-1 | Phase READMEs on `develop` cut down to test-suite stubs | Commit `aa2798e` ("Refactor README files") left 10 phase/component READMEs (phases 0-6, `build-system`, `ab-update`) as short stubs, e.g. `phase-2-continuumd-pairing/README.md` is 2 lines. Scope, entry/exit criteria and structure sections are gone. This review had no Phase 2 exit criteria to check against and used the original from `82e2ebf`. | Open, needs owner decision | Before the next phase |
| P2-2 | No D-Bus client library or protobuf toolchain in any manifest | `DbusStatusSource` and `pairing.proto` codegen (Rust and Kotlin) need build-system support. | Open | Build-system implementation session |
| P2-3 | Emulated pairing harness does not exist | All protocol and integration tests depend on it. | Open | Before any Phase 2 test can run |
| P2-4 | Android QR/numeric confirmation UI undesigned | Protocol is specified; the Compose screen is not. | Open | Android implementation session |
| P2-5 | Grace-window value and timeouts unvalidated | 30s default is a placeholder pending real-device measurements. | Open, needs P0-1 | After first hardware run |
| P2-6 | `PairedPhone` backup destination has no transport | Intentional until `continuum-dropd` (Phase 3). | Open, expected | Phase 3 |
| P2-7 | Data-handling policy document not written | Schemas exist; the published policy must land before Phase 3 alpha. | Open | Before Phase 3 public alpha |
| P2-8 | Carry-overs from Phase 0/1 | CI vendor (P0-4), Tier-1 device (P0-1), wlroots integration (P1-1). | Open, owner decisions | Unchanged |
| P2-9 | `SESSION_LOG.md` on `develop` had a duplicated Phase 3 heading | Fixed in this branch. | Closed | n/a |

## Exit determination
Not exited, same scaffold-only framing as Phases 0 and 1. The Phase 1
mock `ContinuityStatusSource` mapped 1:1 onto the real D-Bus API with no
trait change, and the session-loss gap in the architecture doc is
resolved. Every item above has a path; P2-1 and P2-8 need your decision.
