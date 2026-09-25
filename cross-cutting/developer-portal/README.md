# Developer Portal & SDK Stability

See `../../docs/architecture/05-app-ecosystem-compatibility.md`.
ContinuumKit is scoped as a Phase 1 deliverable in the source plan, but
nothing addresses how third-party developers actually get access to it
— needed before the Phase 3 public alpha is meaningful to anyone
outside the project.

## Scope (initial)
- Public API reference site + sample apps (a Files-equivalent, a
  Handoff-aware text editor, a ContinuumKit "hello world").
- Explicit API stability policy: what's frozen at alpha vs. still
  churning, so third parties don't build against instability.
- App Store packaging/signing/review pipeline, distinct from the OS's
  own A/B update pipeline (`../../phase-0-foundation/boot/ab-update/`).

## Status
Not started — targeted to land alongside Phase 1 app-runtime work so
it's ready before Phase 3.
