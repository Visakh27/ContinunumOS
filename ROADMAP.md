# Continuum OS — Roadmap

Source: `Continuum_OS_Engineering_Plan.docx` v1.0 (Sept 2026), extended
with items identified as missing during scaffolding (marked **[added]**).
Months are relative to project start ("M1" = first month of Phase 0).

| Phase | Name | Timeline | Public milestone | Status |
|---|---|---|---|---|
| 0 | Foundation & Kernel Bring-Up | M1–M6 | Internal: boots to text console, verified boot + OTA working | 🟢 In progress (this repo) |
| 1 | Display, Compositor & Shell Skeleton | M7–M14 | Internal: usable desktop, ported Linux apps | ⬜ Scaffolded |
| 2 | continuumd Core & Pairing | M15–M20 | Internal: phone pairs, shows live "Connected" | ⬜ Scaffolded |
| 3 | Clipboard, Handoff, Drop | M21–M30 | **Public alpha** | ⬜ Scaffolded |
| 4 | Phone, Notifications, Camera | M31–M40 | **Public beta** | ⬜ Scaffolded |
| 5 | Universal Control, Hotspot, Win-app compat | M41–M52 | Internal: feature-complete GA candidate | ⬜ Scaffolded |
| 6 | OEM Certification & GA | M53–M60 | **General Availability** | ⬜ Scaffolded |

## Cross-cutting workstreams **[added]**

These run in parallel with every phase rather than owning a phase of
their own — they are easy to underscope in a plan organized purely by
feature daemon. Tracked under `cross-cutting/`.

| Workstream | Why it's separate | Starts |
|---|---|---|
| Accessibility (screen reader, switch control, captioning, high-contrast) | Legal requirement (ADA/EN 301 549) in most target markets; must be designed into ContinuumKit from Phase 1, not bolted on at Phase 6 | Phase 1 |
| Internationalization / localization | Text direction (RTL), input method editors (CJK, Indic), locale-aware formatting must be in the shell and ContinuumKit APIs from the start | Phase 1 |
| Telemetry, diagnostics & privacy | Crash reporting, opt-in usage telemetry, and a published data-handling policy — needed to make the Phase 13 KPIs (crash-free boot rate, pairing success rate) measurable at all, and required before Phase 3 public alpha | Phase 0 |
| Audio stack | Not mentioned in the source doc; needed for call-audio routing (continuum-phoned), video conferencing, and general desktop use | Phase 1 |
| Print stack (CUPS-equivalent) | Table-stakes desktop OS feature, absent from source doc | Phase 1 |
| External display / Thunderbolt-USB4 | Multi-monitor is mentioned for the compositor; DisplayPort-alt-mode and hot-plug docking are a distinct hardware-enablement stream | Phase 0/5 |
| Backup & restore | No data-loss story exists in the source doc; needed before any public alpha | Phase 2 |
| Enterprise / MDM management | Needed for any OEM or business channel; profiles, remote wipe, compliance policy | Phase 5 |
| Recovery mode / factory reset | Complement to A/B rollback — covers user-initiated recovery, not just failed-update rollback | Phase 0 |
| Developer portal & docs | ContinuumKit SDK is scoped in Phase 1 but needs a public docs site, sample apps, and API stability policy before third parties can build against it | Phase 1 |
| Legal / export compliance & licensing | GPL/kernel license compliance, export control on the crypto stack (Ed25519, Noise, TLS), OEM patent/trademark terms | Phase 0 (review), ongoing |
| Supply chain & build security (SLSA-style provenance, reproducible builds, SBOM) | Verified boot protects the running system; nothing in the source doc protects the *build pipeline* itself | Phase 0 |

## Phase gates

Each phase's `README.md` defines explicit entry criteria (what must be
true to start) and exit criteria (what must be true, and demonstrated in
the phase's `tests/` suite, to call the phase done). No phase starts
before the previous phase's exit criteria are met, per the Section 17.2
document-control review cadence ("revisit phase scope ... at the end of
every phase milestone").
