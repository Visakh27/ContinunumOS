# Phase 5 Test Plan — Universal Control, Hotspot, Windows Compat

**Covers:** `phase-5-control-hotspot-winapp/`

- **Unit:** virtual-display output negotiation, input-event injection
  serialization, hotspot network-path selection logic.
- **Integration:** Universal Control session over the emulated harness
  (input events round-tripping desktop→virtual-display→mocked-phone);
  hotspot fallback triggering on simulated Wi-Fi loss.
- **Security:** this phase's headline gate — independent penetration
  test of `continuum-controld` (compositor-level input injection is the
  single highest-privilege continuity surface in the system) and a
  sandbox-escape test suite for the Windows compatibility layer.
- **HIL:** input-injection latency and correctness on Tier-1 hardware;
  Windows-compat layer tested against the prioritized top-N
  most-requested Windows apps (per the Risk Register mitigation in
  `../../phase-5-control-hotspot-winapp/windows-compat-layer/README.md`).
- **Exit gate:** feature-complete GA candidate (Phase 5 milestone) —
  requires the full security audit and third-party pen test to complete
  with findings triaged, per Section 13.

Skeleton: `phase-5-control-hotspot-winapp/tests/`.
