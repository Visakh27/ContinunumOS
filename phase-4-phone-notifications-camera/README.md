# Phase 4 — Phone, Notifications, Camera (M31–M40) — Public Beta

## Scope (source plan, Section 11)
- Ship `continuum-phoned` (notifications, SMS/RCS, call audio routing)
  and `continuum-camerad` (V4L2 virtual camera + scan-to-desktop).
- Expand hardware-enablement to Tier-2 best-effort device support.
- Public beta release.

## Added to scope
- Native OS notification model must exist before `continuum-phoned` can
  mirror into it meaningfully (`../docs/architecture/04-desktop-shell-ux.md`).
- Audio stack (`../cross-cutting/audio-stack/`) call-audio routing
  integration is this phase's dependency, not a nice-to-have.

## Entry criteria
Phase 3 exit criteria met: clipboard/handoff/drop working, public alpha live.

## Exit criteria
- [ ] Notification mirroring, SMS/RCS, call-audio routing functional
      end-to-end.
- [ ] Phone camera exposed as a native V4L2 device, usable by
      representative third-party apps (video call, screenshot tool).
- [ ] Tier-2 best-effort device support demonstrated via HIL pass on a
      Tier-2 sample, not just Tier-1.
- [ ] `tests/` suite passing per `../docs/testing/phase-4-test-plan.md`.

## Structure
```
phase-4-phone-notifications-camera/
├── continuum-phoned/
├── continuum-camerad/
└── tests/{unit,integration,protocol}/
```
