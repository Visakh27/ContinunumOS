# Phase 4 Open Items (Exit Review, Session 51)

| # | Title | Detail | Status | Target |
|---|---|---|---|---|
| P4-1 | Desktop as Bluetooth hands-free device is unproven | ADR 0007 rests on BlueZ + PipeWire (or oFono) supporting the desktop in the HFP hands-free role well enough. If not, the call feature has no fallback: Android does not allow call-audio capture. | Open, **highest risk** | First hardware pass (`hil/call_audio_hfp_test.md`) |
| P4-2 | SMS/RCS is notification-only | No history, reply-only, no new conversations. Full SMS would need the Companion to be the default SMS app (large product and Play policy decision). | Open, **owner decision** | Before beta docs |
| P4-3 | Virtual camera deviates from the plan | ADR 0006 makes PipeWire primary; legacy `/dev/video*` apps won't see it unless the optional loopback layer ships. | Open, **owner decision** | After security review of the module |
| P4-4 | Invalid kernel option removed | `CONFIG_VIDEO_V4L2LOOPBACK` removed in Session 47. Still to confirm against the pinned LTS tree that the symbol never existed. | Closed, pending LTS pin (P0-1) | n/a |
| P4-5 | Call control without default-dialer status varies by Android version | Which commands work through the session fallback is unverified. | Open | Android implementation |
| P4-6 | Camera latency budget unset | Needs measurements. | Open | First hardware pass |
| P4-7 | Independent implementation security review | Design-time profile only (`phase-4-daemon-hardening.md`). Required before beta (checklist gate 1). | Open, beta gate | Once daemons exist |
| P4-8 | Rate limits and dial-confirmation UX undecided | Notification flood limits; confirmation for unusual numbers. | Open | Phone app design pass |
| P4-9 | Tier-2 sample not selected | Number of devices and who provides them; the Phase 4 exit criterion needs a real sample. | Open, **owner decision** | Before Phase 4 can exit |
| P4-10 | H.264 decoder choice | Hardened software vs. hardware decode given the attack surface. | Open | Before camerad implementation |
| P4-11 | OEM battery management can kill the notification listener | Varies by vendor; onboarding must guide the user. | Open, known risk | OEM matrix results |
| P4-12 | Phase 3 inconsistency fixed | `continuum-dropd` opened its own data path, contradicting "only `continuumd` opens sockets". Now `continuumd`'s `data_path` service. | Closed | n/a |
| P4-13 | Beta gates need owners | Legal review, update-channel server, feedback path, accessibility and localization decisions, support. See `beta-readiness-checklist.md`. | Open, **owner decisions** | Before beta |
| P4-14 | Carry-overs | Tier-1 device (P0-1), CI vendor (P0-4), wlroots (P1-1), pairing harness (P2-3), Android clipboard (P3-1). | Open, owner decisions | Unchanged |

## Exit determination
Not exited (scaffold-only scope). Designs, schemas, tests and a
design-time security profile exist for notifications, messaging, calls and
the camera. Two exit criteria cannot be met by design work: functional
end-to-end behavior and the Tier-2 HIL pass. The biggest finding is P4-1:
the calling feature depends on an assumption about the Bluetooth stack
that only hardware can confirm. P4-2 and P4-3 change what the product
delivers relative to the original plan and need your decision.
