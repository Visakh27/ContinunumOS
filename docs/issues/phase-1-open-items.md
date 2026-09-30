# Phase 1 Open Items (filed at Exit Review, Session 21)

| # | Title | Detail | Status | Target |
|---|---|---|---|---|
| P1-1 | wlroots not integrated into the build | No build-system layer pulls in wlroots yet; `continuumwm/src/protocols/README.md` flags this explicitly. Blocks all compositor implementation work. | Open | Build-system implementation session, needs Phase 0 items P0-6 (Yocto recipes) resolved first |
| P1-2 | No concrete CI system, still | Same root cause as P0-4 — every test in this phase's `tests/` inherits the same "nothing can run" blocker. | Open, duplicate of P0-4 | Same as P0-4 |
| P1-3 | Edge-snap thresholds / corner regions unspecified | `window-manager/README.md` explicitly leaves these as tuning parameters. Low risk — doesn't block other work, but needs a decision before the layout unit tests can assert concrete numbers instead of just branch coverage. | Open, low urgency | Implementation session |
| P1-4 | No concrete IME engine selected | `cross-cutting/localization/README.md` designs the integration point but picks no actual CJK/Indic input method implementation — flagged in that doc as a deliberately-deferred substantial scope of its own. | Open, scoped as deferred (not a Phase 1 blocker) | Later session, dedicated to IME engine selection |
| P1-5 | DND-suppressed-but-still-recorded behavior not in the design doc | Discovered while writing `tests/integration/notification_dnd_test.md` (Session 19). | **Closed this session** — `notification-center/README.md` now states it explicitly | — |
| P1-6 | Boot-time-style "budget TBD" now appears in two more places | `virtual_desktop_switch_perf_test.md` and `sandbox_capability_prompt_hil_test.md` both defer latency budgets, same pattern as P0-9. Bundling here rather than filing two more near-duplicate issues. | Open | Set once Tier-1 device exists and first real measurements are taken |
| P1-7 | PipeWire and CUPS not yet in any build-system manifest | `audio-stack/README.md` and `print-stack/README.md` both note their dependency isn't in the Phase 0 manifest. | **Closed this session** — `phase-0-foundation/build-system/manifests/phase-1-image.manifest` created as the Phase-0-superset manifest | — |
| P1-8 | Accessibility tree has a design partner problem | `accessibility-tree/` designs the *producer* side (widgets expose an `Accessible` tree) but no *consumer* (an actual screen reader) is designed yet — noted already in `cross-cutting/accessibility/README.md`, restated here so it's visible in the Phase 1 exit review rather than only in a cross-cutting doc. | Open, expected — screen reader is substantial scope of its own | Later, dedicated session |

## Exit determination

Same framing as Session 11: no exit criterion in
`phase-1-shell-compositor/README.md` is expected to be checked off yet
— this phase is scaffold/design, not implementation, per current
project scope. This review confirms every exit criterion traces to an
open item above with an unblocked path, and surfaces two multi-phase
carry-overs (P1-1/P1-2 both restate Phase 0 blockers — resolving those
once unblocks both phases' test suites simultaneously) plus two
consciously-deferred substantial scopes (P1-4 IME engine, P1-8 screen
reader) that are correctly *not* being rushed into this pass.

**Recommendation:** the wlroots build-system integration (P1-1) and CI
vendor selection (P1-2/P0-4) remain the two highest-leverage unblocks —
unchanged from the Phase 0 review's recommendation, which tracks: they
were never Phase-0-specific problems, they're whole-project
infrastructure decisions that happen to have been first noticed there.
