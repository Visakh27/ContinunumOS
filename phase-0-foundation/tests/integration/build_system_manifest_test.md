# Integration test: Phase 0 image manifest completeness

**Level:** integration (build-system output validation, no physical
hardware required — validates the *manifest*, not a flashed device;
device-level validation is the HIL suite below).

## Test: every manifest component has a corresponding recipe layer entry
- **Given:** `phase-0-foundation/build-system/manifests/phase-0-minimal-image.manifest`
- **When:** cross-referenced against `phase-0-foundation/build-system/layers/*/README.md`
- **Then:** every component listed in the manifest (kernel, init,
  filesystem, boot-chain, update-client, recovery, diagnostics, console)
  has an owning layer.
- **Status:** not runnable yet — no automated manifest-to-layer linter
  exists; this is a specification for one, to be implemented alongside
  the Yocto recipes themselves (build-system-implementation session).

## Test: image is a strict superset going into Phase 1
- **Given:** the Phase 0 manifest and the (future) Phase 1 manifest
- **When:** compared
- **Then:** every Phase 0 component is still present in Phase 1's
  manifest (per the "superset, never a rewrite" principle in
  `../../build-system/README.md`)
- **Status:** blocked on Phase 1 manifest existing; test spec recorded
  here so it isn't forgotten when Phase 1 build-system work starts.
