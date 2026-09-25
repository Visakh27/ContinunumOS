# Continuum OS — Engineering Repository

Continuum OS is an independent, Linux-kernel-based desktop operating system
with native, kernel-adjacent Android device continuity (clipboard, handoff,
file drop, phone/notifications, universal control, camera, hotspot) built
in as always-on OS services rather than a third-party app.

This repository is the engineering scaffold for the project: architecture
decisions, the phase-by-phase build roadmap, and the folder structure,
code skeletons, and test plans for every phase and every continuity
daemon described in the source engineering plan
(`Continuum_OS_Engineering_Plan.docx`).

## How this repository is organized

```
continuum-os/
├── docs/                       Architecture, ADRs, glossary, test strategy
├── phase-0-foundation/         Kernel bring-up, boot chain, build system, CI
├── phase-1-shell-compositor/   ContinuumWM (Wayland), shell skeleton, app runtime
├── phase-2-continuumd-pairing/ Core continuity daemon + pairing + Continuum Center UI
├── phase-3-clipboard-handoff-drop/  First three feature daemons + public alpha
├── phase-4-phone-notifications-camera/  Phone/notification + camera daemons
├── phase-5-control-hotspot-winapp/  Universal Control, hotspot, Windows compat, security audit
├── phase-6-oem-certification-ga/    OEM cert kit, imaging, localization, accessibility, GA
├── android-companion/          Continuum Companion Android app (Kotlin)
├── cross-cutting/              Concerns that span every phase (see docs/architecture/09-*)
└── tools/                      Shared scripts and CI templates
```

Each phase directory contains its own `README.md` (scope, entry/exit
criteria, deliverables), source subdirectories with `README.md` design
stubs and code skeletons, and a `tests/` directory with a phase test plan
plus test skeletons at the unit / integration / HIL (or protocol/security)
level.

## Working model: daily sessions, no fixed end date

This project is paced as **daily 1-hour engineering sessions**
(10:00–11:00), tracked in [`SESSION_LOG.md`](./SESSION_LOG.md), starting
the first Monday after this repository was created. There is no fixed
completion date — each session ends with the repository in a coherent,
buildable-on-paper state, and the next session picks up exactly where the
log leaves off. See `SESSION_LOG.md` for the running index of what's been
done and what's next.

## What "done" means at this stage

Per the current scope, **this repository is a design-and-scaffold
deliverable, not a running build**: source files are structured,
documented skeletons (interfaces, module boundaries, protocol schemas,
config templates, test plans and test skeletons) rather than compiled,
executed code. Running, integrating, and debugging the actual build is
explicitly deferred to a later stage per your instruction. Every skeleton
is written so that filling in the implementation is the only remaining
step — folder structure, module boundaries, test scaffolding, and
documentation are already in place.

## Start here

1. [`ROADMAP.md`](./ROADMAP.md) — the phase plan, with additions beyond the source doc.
2. [`SESSION_LOG.md`](./SESSION_LOG.md) — daily session index.
3. [`docs/architecture/00-overview.md`](./docs/architecture/00-overview.md) — architecture entry point.
4. [`docs/testing/00-testing-strategy.md`](./docs/testing/00-testing-strategy.md) — overall QA philosophy.
5. [`phase-0-foundation/README.md`](./phase-0-foundation/README.md) — the current active phase.
