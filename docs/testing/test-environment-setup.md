# Test Environment Setup

Reference for what's needed to execute (not just read) the test suites
under each phase's `tests/`. This is written now as a specification;
provisioning is deferred along with running/debugging per current
project scope.

## Toolchains
- Rust stable + the MSRV pinned in `phase-0-foundation/build-system/README.md`, `cargo-nextest` for faster unit runs.
- Kotlin/JVM toolchain + Android SDK/emulator for `android-companion/`.
- `bats-core` for shell/build-script tests.
- `protoc` + `buf` for protocol schema linting and breaking-change detection (protocol tests, all continuity phases).

## Emulated pairing harness (for integration/protocol tests without hardware)
A container-pair harness that runs two `continuumd` instances (or one
`continuumd` + one mocked Companion) on a virtual network namespace, so
pairing, session, and feature-daemon protocol tests can run in CI
without a physical phone. Skeleton: `phase-2-continuumd-pairing/tests/integration/`.

## Hardware-in-the-loop (HIL) lab
Physical Tier-1 reference devices + a rotating Tier-2 sample + a
physical Android OEM matrix (Samsung, Pixel, OnePlus, Xiaomi, Motorola).
Provisioning, remote power control, and serial-console log capture for
this lab are a Phase 0/CI deliverable (`phase-0-foundation/ci/`) but the
lab's physical existence is outside this repository's scope.

## CI gating
See `phase-0-foundation/ci/README.md` for the pipeline that runs unit +
integration on every commit, HIL nightly, and protocol/security suites
pre-release.
