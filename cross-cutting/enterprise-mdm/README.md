# Enterprise / MDM Management

Not in the source plan; needed for any OEM or business distribution
channel (`../../docs/architecture/09-additions-and-gaps.md`). Starts
Phase 5, alongside OEM-facing work.

## Scope (initial)
- Device enrollment/profile model (comparable to macOS MDM profiles).
- Remote wipe (interacts with `../recovery-mode/`'s wipe action).
- Policy enforcement hooks: app allow/deny lists, continuity-daemon
  feature toggles locked by policy (e.g. an enterprise disabling
  `continuum-dropd` fleet-wide).
- Compliance reporting (patch level, Secure Boot status) for fleet
  management tools.

## Status
Not started — placeholder scope only, per `../../ROADMAP.md`'s
cross-cutting workstream table (starts Phase 5).
