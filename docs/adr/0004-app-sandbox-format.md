# ADR 0004: App Sandbox / Package Format

**Status:** Accepted

## Context
Need a primary app-distribution format with capability-scoped
sandboxing (no default filesystem/camera/network access), per
`../architecture/07-security-permission-model.md`.

## Decision
OS-native containerized app format, Flatpak-derived (OSTree-based
distribution + bubblewrap-style sandbox + portal-based capability
brokering), extended with ContinuumKit-specific capabilities (Handoff
API access, continuity-daemon D-Bus surfaces) as additional
grantable/revocable capabilities in the permission broker.

## Consequences
- Reusing Flatpak's OSTree distribution model gives immediate
  compatibility with the existing Flatpak app catalog
  (`../architecture/05-app-ecosystem-compatibility.md`), bootstrapping
  the software catalog from day one instead of starting from zero.
- ContinuumKit-specific capabilities are additive to the Flatpak
  portal model, not a fork of it, so upstream Flatpak security fixes
  remain adoptable.
