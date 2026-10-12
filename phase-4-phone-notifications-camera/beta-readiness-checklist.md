# Public Beta Readiness Checklist

Phase 4's milestone is the public beta. This lists what must be true
before real users get the OS. Items marked **owner** need a decision or
action from outside engineering. Nothing here is done yet; this is the
gate list.

## Must be true (gates)
| # | Gate | Status / dependency |
|---|---|---|
| 1 | Independent security review of the Phase 3 and Phase 4 daemon implementations | Needs real code (P3-2, P4-7) |
| 2 | Data-handling policy approved and published; opt-in telemetry UI in first-run setup | Draft exists (`cross-cutting/telemetry-privacy/data-handling-policy-draft.md`); **owner**: legal review, retention numbers |
| 3 | Legal: export-control review of the crypto stack; GPL/LGPL source-offer compliance for distributed images | Kicked off in Phase 0 as a tracked action; no result recorded in this repo. **owner** |
| 4 | A/B update and rollback proven on real hardware for the beta image | HIL (`phase-0-foundation/tests/hil/`), blocked on P0-1 |
| 5 | `beta` release channel live with staged cohort rollout and halt-on-failure | Channel and `rollout.cohort_percentage` exist in `update-manifest.schema.json`; the server side is not designed (Phase 6 deliverable per the A/B README). Beta needs a minimal version sooner. **owner** |
| 6 | Feedback path: in-OS "Send feedback" with optional diagnostic log export, plus a public issue tracker | Log export exists in the recovery design; the feedback flow does not. **owner**: where reports go |
| 7 | Known-issues list shipped with each release; Tier-2 support matrix published | Matrix template: `tier2-hardware-enablement/README.md` |
| 8 | Companion on a Play Store testing track plus sideloadable APK, with protocol-version compatibility rules published | `pairing.proto` carries version in every message; the compatibility policy is not written |
| 9 | User guide that states limits plainly | See below |

## Limits the user guide must state
- SMS/RCS mirroring works through notifications: no history, reply only while the notification exists, no new conversations (`continuum-phoned/README.md`, P4-2).
- Clipboard from phone to desktop may need a manual action on Android (P3-1).
- Handoff from phone only works for apps that integrate the SDK.
- Calls need classic Bluetooth paired separately, and quality depends on the devices (ADR 0007).
- The phone camera works as a webcam in apps that use the PipeWire/portal camera path; legacy apps that open `/dev/video*` directly may not see it (ADR 0006, P4-3).

## Gaps to decide on, not hide
- **Accessibility:** the screen reader consumer is undesigned (P1-8). Shipping a beta without it excludes blind users; either delay or ship with this stated clearly. **owner**
- **Localization:** the IME engine is unselected (P1-4). Decide whether the beta is English-only. **owner**
- **Support:** who answers users, and how fast. **owner**
- **Hardware:** the Tier-1 device and the Tier-2 sample are still unselected (P0-1, P4-9).

## Beta metrics tracked (not gates)
Boot success per `device_class`, pairing success rate, and the new
operations listed in the telemetry README (notification mirror, call
audio route, camera stream), all coarse and opt-in.
