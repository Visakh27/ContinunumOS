# Secure Boot Chain

Implements step 1–2 of `../../../docs/architecture/02-boot-install-update.md`'s
boot chain: UEFI Secure Boot verifies the Continuum bootloader signature;
the bootloader verifies and loads the kernel+initramfs via dm-verity
(see `../dm-verity/`).

## Components

- `keys/` — key-generation and rotation runbook (no private key material
  ever committed to this repo; production keys live in an HSM per
  `../../../docs/architecture/07-security-permission-model.md`'s
  supply-chain-security requirement).
- `sign-bootloader.sh` — skeleton signing script (build-pipeline
  integration point, not yet wired to a real HSM).
- `enroll-keys.md` — process for enrolling Continuum's Secure Boot
  signing key (or shim-style chain) on Tier-1 reference hardware and, on
  the OEM path, at the factory.

## Design notes

- Bootloader candidate: `systemd-boot` or GRUB, both Continuum-signed
  (per source plan Section 4.1). Final pick tracked alongside ADR 0002
  (init system) since `systemd-boot` vs GRUB interacts with the
  systemd-vs-custom-init decision.
- A tampered/unsigned bootloader or kernel must **fail closed** — refuse
  to boot rather than warn-and-continue — this is validated by
  `../../tests/hil/test_secure_boot_tamper_detection.md`.
