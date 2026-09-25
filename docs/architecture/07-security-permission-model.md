# Security & Permission Model

- **Verified boot:** every boot stage (bootloader → kernel → root
  filesystem) is cryptographically verified via Secure Boot + dm-verity;
  a modified root partition refuses to boot or drops to a
  recovery/reinstall flow, matching ChromeOS/Android verified-boot
  design. See `../../phase-0-foundation/boot/`.
- **App sandboxing:** all ContinuumKit and Flatpak-distributed apps run
  in capability-scoped sandboxes (no default filesystem, camera, or
  network access) with a macOS-TCC-style permission broker prompting
  per-capability.
- **Continuity daemons** are the only OS components with elevated
  radio/input/virtual-device privileges, and each is individually
  toggleable and independently sandboxed from each other (a compromised
  clipboard daemon cannot reach the input-injection daemon).
- **Pairing security:** mutual Ed25519 key exchange plus rotating
  QR/numeric-comparison confirmation, identical security bar to the
  app-based design, with trust anchors stored in a TPM-backed keystore.
- All continuity network traffic is end-to-end encrypted (Noise Protocol
  / TLS 1.3 mutual auth); a cloud relay is used only for WAN fallback
  pairing/wake, never as a data store.

## Supply chain & build security **[added]**

Verified boot protects the *running* system from tampering after it
leaves the factory. Nothing in the source plan protects the *build
pipeline* that produces the signed images in the first place — a
compromise there (malicious kernel patch, poisoned build dependency,
stolen signing key) would produce a validly-signed, fully verified-boot
compliant, backdoored image. Added as a Phase 0 requirement:

- Reproducible builds for the kernel and core OS image, so a build
  artifact's hash can be independently verified against source.
- SLSA-style build provenance attestation attached to every signed
  release artifact.
- Signing keys held in an HSM, with a documented key-rotation and
  key-compromise runbook.
- A software bill of materials (SBOM) published per release, covering
  the kernel patch set, base system packages, and continuity daemon
  dependencies.

Detail: `../../cross-cutting/security/supply-chain.md`.

## Export control & licensing **[added]**

Not addressed in the source plan. Ed25519, Noise Protocol, and TLS 1.3
are cryptographic exports subject to control regimes (e.g. US EAR) in
some jurisdictions; the Linux kernel and other GPL/LGPL dependencies
carry license-compliance obligations (source-offer requirements,
attribution) that must be satisfied in the OEM pre-install and ISO
distribution paths. Tracked as a Phase 0 legal-review action item,
revisited before every major release.
