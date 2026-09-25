# Phase 5 Security Audit

Independent penetration test of `continuum-controld` (compositor-level
input injection — the single highest-privilege continuity surface in
the system) and a sandbox-escape test suite for the Windows
compatibility layer. Required before Phase 5 exit and GA.

## Scope
- `continuum-controld` input-injection privilege boundary
- Windows compat layer sandbox escape resistance
- Re-verification of the pairing protocol and all continuity daemons
  (not just those new in this phase) — a full-system audit, per Section
  13 of the source plan, not a Phase-5-only-components audit

## Status
Not started — this is an external engagement, tracked here as a gate,
not implemented in this repository.
