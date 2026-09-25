# Additions & Gaps Identified During Scaffolding

The source engineering plan is a strong, credible v1 architecture but is
— as any single planning document for a project this size will be —
incomplete in places a real build would surface immediately. This doc
consolidates every addition called out inline elsewhere in `docs/architecture/`,
in one place, with where each lives.

| Gap | Why it matters | Tracked at |
|---|---|---|
| Accessibility (screen reader, switch control, captioning, high-contrast, reduced motion) | Legal requirement in most launch markets (ADA precedent, EN 301 549); must be a ContinuumKit API from Phase 1, not retrofitted | `cross-cutting/accessibility/`, `04-desktop-shell-ux.md` |
| Internationalization / localization, IME support | Shell unusable outside English-first markets without it | `cross-cutting/localization/`, `04-desktop-shell-ux.md` |
| Telemetry, crash reporting & privacy policy | Required to measure the plan's own KPIs (crash-free boot rate, pairing success rate); also an alpha-readiness blocker | `cross-cutting/telemetry-privacy/` |
| Audio stack | No audio subsystem specified anywhere, yet `continuum-phoned` requires call-audio routing | `cross-cutting/audio-stack/`, `04-desktop-shell-ux.md` |
| Print stack | Table-stakes desktop feature, absent from source doc | `cross-cutting/print-stack/` |
| External display / dock / Thunderbolt-USB4 hardware enablement | Distinct from compositor multi-monitor support; its own certification stream | `cross-cutting/external-display/`, `03-driver-hardware-strategy.md` |
| Backup & restore | No data-loss story exists before public alpha | `cross-cutting/backup-restore/` |
| Enterprise / MDM management | Needed for OEM/business channel; profiles, remote wipe, compliance | `cross-cutting/enterprise-mdm/` |
| Recovery mode / user-initiated reset | A/B rollback only covers *failed updates*, not user-initiated recovery | `cross-cutting/recovery-mode/`, `02-boot-install-update.md` |
| Developer portal, docs site, SDK stability policy | ContinuumKit needs public-facing docs and a stability contract before third parties build against it | `cross-cutting/developer-portal/`, `05-app-ecosystem-compatibility.md` |
| Legal / export compliance & licensing | Crypto export control, GPL/LGPL compliance obligations, OEM trademark terms | `07-security-permission-model.md` |
| Supply chain & build security (SLSA provenance, reproducible builds, SBOM, HSM-held signing keys) | Verified boot protects the running system, not the build pipeline that produced it | `cross-cutting/security/`, `07-security-permission-model.md` |
| Native OS notification model (independent of Android mirroring) | `continuum-phoned` mirrors Android notifications, but native-app notifications aren't specified | `04-desktop-shell-ux.md` |
| Continuity session-loss / resume behavior | Undefined what happens when `continuumd`'s session drops mid-feature-operation | `06-continuity-subsystem.md` |

None of these change the source plan's phase sequencing or core
architecture decisions; they're additions that slot into the existing
phases (mostly Phase 0–1) as parallel workstreams under `cross-cutting/`,
per `ROADMAP.md`.
