# Phase 5 — Universal Control, Hotspot, Windows App Compatibility (M41–M52)

## Scope (source plan, Section 11)
- Ship `continuum-controld` (virtual display + compositor-level input
  injection) and `continuum-hotspotd`.
- Integrate and harden the Wine/Proton-derived Windows app compatibility
  layer.
- Full security audit, third-party penetration test of the pairing
  protocol and continuity daemons.
- Milestone: feature-complete Continuum OS candidate for GA.

## Added to scope
- Enterprise/MDM management groundwork begins here
  (`../cross-cutting/enterprise-mdm/`), alongside other OEM-facing work.

## Entry criteria
Phase 4 exit criteria met: public beta live with phone/notification/camera daemons.

## Exit criteria
- [ ] Universal Control functional: virtual display + input injection
      round-trips correctly, using the virtual-output pipeline designed
      into the compositor back in Phase 1 (ADR 0005).
- [ ] `continuum-hotspotd` automatic tethering fallback functional.
- [ ] Windows compatibility layer covers the prioritized top-N
      most-requested apps (Risk Register mitigation,
      `../docs/architecture/../` source doc Section 15).
- [ ] Independent security audit + pen test of `continuum-controld`
      (highest-privilege continuity surface) complete, zero outstanding
      critical findings.
- [ ] `tests/` suite passing per `../docs/testing/phase-5-test-plan.md`.

## Structure
```
phase-5-control-hotspot-winapp/
├── continuum-controld/
├── continuum-hotspotd/
├── windows-compat-layer/
├── security-audit/
└── tests/{unit,integration,security}/
```
