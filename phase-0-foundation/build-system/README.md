# Build System

Builds the Continuum OS reference image: kernel + patch set (`../kernel/`)
+ base system layer + boot chain (`../boot/`) into a flashable/bootable
image for Tier-1 reference hardware.

## Choice: Yocto-derived

Rationale: Yocto's layer model maps cleanly onto Continuum's own
layering (kernel layer, base-system layer, continuity-daemon layer added
in Phase 2+, shell layer added in Phase 1), and its image-recipe
approach naturally supports the A/B dual-partition output the update
mechanism (`../boot/ab-update/`) needs. Buildroot was considered as a
simpler alternative; rejected for v1 because its weaker package-layering
story would make it harder to keep the continuity-daemon layer
independently versionable from the base OS layer as the project grows
past Phase 0, per `docs/architecture/02-boot-install-update.md`'s
decoupled-update-cadence goal.

## Layer layout (Yocto convention)

```
build-system/
├── README.md
├── conf/
│   ├── local.conf.template       Build-machine local config template
│   └── bblayers.conf.template    Layer list template
├── layers/
│   ├── meta-continuum-kernel/     Kernel recipe + config/patches from ../kernel/
│   ├── meta-continuum-base/       Base system layer (init, filesystem, boot)
│   └── meta-continuum-image/      Top-level image recipe (Phase 0 minimal image)
└── manifests/
    └── phase-0-minimal-image.manifest   Package/component list for the Phase 0 exit-criteria image
```

`layers/` and `conf/` are placeholders at this stage (empty directories
with a README each) — populating the actual Yocto recipes is a
build-system-implementation task, out of scope for the current
documentation/scaffolding session per `../../SESSION_LOG.md` (Session 3
produced this structure and manifest; recipe implementation is a later,
run-the-build session).

## Phase 0 minimal image manifest

See `manifests/phase-0-minimal-image.manifest` — the exit-criteria image
is deliberately minimal: kernel + init + verified boot chain + A/B update
client + a text console. No compositor, no shell, no continuity daemons
— those are Phase 1+ image layers, added incrementally so each phase's
image is a superset of the previous phase's, never a rewrite.
