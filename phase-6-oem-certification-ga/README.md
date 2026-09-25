# Phase 6 — OEM Certification & General Availability (M53–M60)

## Scope (source plan, Section 11)
- Finalize the OEM hardware-certification program and reference-device
  imaging kit.
- Performance, battery, and thermal hardening pass across the certified
  device matrix.
- Localization, accessibility audit, and documentation.
- GA launch with a certified Tier-1 device lineup and downloadable ISO.

## Entry criteria
Phase 5 exit criteria met: feature-complete GA candidate, security audit
clean.

## Exit criteria (= Section 16 KPIs, all required)
- [ ] 100% of Tier-1 device list passes hardware certification.
- [ ] Pairing success rate ≥98% first-attempt across the Android OEM matrix.
- [ ] A/B update success rate ≥99.9%, 100% simulated-failure rollback recovery.
- [ ] Clipboard sync ≤500ms median; file transfer ≥80% of raw link speed.
- [ ] Crash-free boot rate ≥99.5% across the certified fleet.
- [ ] Verified-boot and sandbox security audits: zero outstanding
      critical findings.
- [ ] Accessibility audit passed (`accessibility/`).
- [ ] Localization coverage complete for the GA-supported locale list
      (`localization/`).
- [ ] `tests/` suite passing per `../docs/testing/phase-6-test-plan.md`.

## Structure
```
phase-6-oem-certification-ga/
├── oem-cert-kit/       Hardware compliance suite, reference imaging kit
├── imaging/            OEM factory imaging process
├── localization/       Locale coverage, RTL/IME functional testing
├── accessibility/      Formal accessibility audit
└── tests/{certification,performance,accessibility}/
```
