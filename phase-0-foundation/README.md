# Phase 0 — Foundation & Kernel Bring-Up (M1–M6)

## Scope (from the source plan, Section 11)
- Select Linux LTS baseline, stand up the build system (Yocto/Buildroot-derived),
  and get a minimal bootable image running on Tier-1 reference hardware.
- Stand up CI/CD, kernel patch-tracking, and the hardware-enablement backlog.
- Implement Secure Boot chain, dm-verity root verification, and the A/B
  partition update mechanism.

## Added to scope during scaffolding
- Recovery mode design (`../cross-cutting/recovery-mode/`)
- Supply-chain / build security baseline (`../cross-cutting/security/supply-chain.md`)
- Telemetry & privacy baseline (`../cross-cutting/telemetry-privacy/`) — needed
  before Phase 3 public alpha, but the schema/policy work starts here so
  crash-free-boot and pairing-success telemetry (Section 16 KPIs) is
  instrumented from the first bootable image, not retrofitted.
- Legal/export-compliance review kickoff (tracked, not a code deliverable)

## Entry criteria
- Architecture decisions in `docs/adr/0001`–`0004` accepted (0002 may
  remain open per its own status).
- Tier-1 reference hardware list drafted (even if provisional) so the
  build system and CI have a concrete target.

## Exit criteria
- [ ] Minimal image boots to a text console on reference hardware.
- [ ] Secure Boot + dm-verity chain verified end-to-end (a tampered
      root partition provably refuses to boot).
- [ ] A/B update mechanism installs, activates, and rolls back on
      simulated boot failure.
- [ ] CI runs unit + integration tests on every commit; HIL boot test
      runs nightly against the reference device.
- [ ] Recovery mode reachable and functional on the reference image.
- [ ] Phase 0 test plan (`../docs/testing/phase-0-test-plan.md`) fully
      passing against the reference image.

None of these are marked complete yet — this phase is scaffolded
(structure, docs, skeletons) but not implemented/run, per current
project scope (running and debugging deferred).

## Structure

```
phase-0-foundation/
├── kernel/            LTS version pin, config fragments, patch tracking
│   ├── config/
│   └── patches/
├── build-system/       Yocto/Buildroot layer skeleton, image manifest
├── boot/
│   ├── secure-boot/    Bootloader signing, UEFI Secure Boot chain
│   ├── dm-verity/      Root filesystem verification
│   └── ab-update/      A/B partition state machine + update client
├── ci/                 Pipeline definitions
└── tests/
    ├── unit/
    ├── integration/
    └── hil/
```
