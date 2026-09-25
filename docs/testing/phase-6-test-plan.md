# Phase 6 Test Plan — OEM Certification & GA

**Covers:** `phase-6-oem-certification-ga/`

- **Certification:** the full OEM hardware-compliance suite
  (`../architecture/03-driver-hardware-strategy.md` checklist) run
  against every Tier-1 device before GA listing.
- **Performance:** battery, thermal, and boot-time hardening pass across
  the certified device matrix — regression-tested against Phase 0–5
  baselines to catch drift.
- **Accessibility:** full accessibility audit (screen reader, switch
  control, contrast/motion) against the `cross-cutting/accessibility/`
  spec — this is the formal gate for the accessibility workstream that's
  been building since Phase 1.
- **Localization:** string coverage, RTL layout, and IME functional
  testing across the GA-supported locale list.
- **Exit gate / KPIs (Section 16), all required before GA:**
  - 100% of Tier-1 device list passes hardware certification.
  - Pairing success rate ≥98% first-attempt across the Android OEM
    matrix.
  - A/B update success rate ≥99.9%, 100% simulated-failure rollback
    recovery.
  - Clipboard sync ≤500ms median; file transfer ≥80% of raw link speed.
  - Crash-free boot rate ≥99.5% across the certified fleet.
  - Verified-boot and sandbox security audits: zero outstanding critical
    findings.

Skeleton: `phase-6-oem-certification-ga/tests/`.
