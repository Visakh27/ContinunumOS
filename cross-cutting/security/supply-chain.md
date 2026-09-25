# Supply Chain & Build Security

See `../../docs/architecture/07-security-permission-model.md#supply-chain--build-security-added`
for rationale.

## Requirements (Phase 0 baseline)

1. **Reproducible builds.** The kernel and core OS image build must be
   bit-for-bit reproducible from pinned source + pinned toolchain, so an
   independent party can verify a released artifact's hash matches a
   from-source rebuild. Tracked as a build-system requirement
   (`../../phase-0-foundation/build-system/`); enforcement mechanism
   (e.g. comparing CI-produced vs. independently-produced hashes) TBD.
2. **Build provenance attestation.** Every signed release artifact
   carries a SLSA-style provenance statement: what source commit, what
   toolchain version, what CI pipeline run produced it. Attached
   alongside the existing db-key signature (`../../phase-0-foundation/boot/secure-boot/`),
   not a replacement for it.
3. **HSM-held signing keys.** No signing key (PK/KEK/db, per
   `../../phase-0-foundation/boot/secure-boot/keys/README.md`) is ever
   present outside an HSM in plaintext form, including in CI.
4. **SBOM per release.** Covers the kernel patch set, base system
   packages, and (from Phase 2 onward) continuity daemon dependencies.
   Published alongside release notes.

## Threat this addresses
Verified boot (Secure Boot + dm-verity) protects a device from running a
*tampered* image. It says nothing about whether the *signed* image
itself was produced honestly — a compromised build pipeline, poisoned
dependency, or stolen signing key would produce a validly-signed,
verified-boot-compliant, backdoored image that this threat model doesn't
catch. Items 1–4 above are what closes that gap.

## Status
Design-level requirements only at this stage; CI enforcement
(`../../phase-0-foundation/ci/`) doesn't yet implement provenance
attestation or reproducibility verification — flagged as a Phase 0
follow-up, not yet in `SESSION_LOG.md`'s numbered sequence.
