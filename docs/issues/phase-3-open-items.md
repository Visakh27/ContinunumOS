# Phase 3 Open Items (Exit Review, Session 41)

| # | Title | Detail | Status | Target |
|---|---|---|---|---|
| P3-1 | Android background clipboard read is restricted | Android 10+ blocks background clipboard reads (Android 12+ also toasts), so phone-to-desktop sync can't be fully automatic. Options (foreground action, accessibility service, IME) are unevaluated; the accessibility route carries Play policy risk. | Open, high risk | Before alpha scoping is final |
| P3-2 | Independent implementation security review not possible yet | `cross-cutting/security/phase-3-daemon-hardening.md` is design-time only. The exit criterion needs a review of real code. | Open, alpha gate | Once daemons exist |
| P3-3 | Scanner choice and signature updates undecided | ClamAV-class vs platform service; updating signatures on a read-only root is unsolved. | Open | Before dropd implementation |
| P3-4 | Handoff coverage on Android is narrow | Works only for apps that integrate the Continuum SDK, plus browser tabs via Continuum Browser. Alpha docs must say so. | Open, expected | Developer portal docs |
| P3-5 | Files app has no design doc | The Files app is referenced by Drop, Share and the shell UX doc but has no folder or spec. | Open, gap found this phase | Phase 1/3 follow-up |
| P3-6 | Policy drafts need owners | `data-handling-policy-draft.md` needs legal review and retention decisions; `api-stability-policy.md` needs a review owner and deprecation timeline. | Open, owner decision | Before alpha |
| P3-7 | Limits are placeholders | 1 MiB clipboard inline cap, 16 KiB handoff payload, 5 min handoff TTL, transfer and quarantine quotas: none measured. | Open | After first hardware run |
| P3-8 | Data path availability varies | Wi-Fi Aware support differs by device; fallback to Wi-Fi Direct vs. LAN is not decided. | Open | OEM matrix results |
| P3-9 | Image clipboard decoder attack surface | Whether to sanitize or re-encode synced images is undecided. | Open | Before clipboardd implementation |
| P3-10 | Carry-overs | Tier-1 device (P0-1), CI vendor (P0-4), wlroots (P1-1), pairing harness (P2-3). | Open, owner decisions | Unchanged |

## Exit determination
Not exited (scaffold-only scope). Design and schemas exist for all three
daemons, the Android modules, the Dock `HandoffSource`, capability
grants, and the session-loss tests required by the exit criteria. Two
exit criteria cannot be met by design work alone: the implementation
security review (P3-2) and the KPI measurements (need hardware). P3-1 is
the biggest feasibility risk found so far.
