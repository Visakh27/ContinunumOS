# Enrolling Continuum Secure Boot Keys

## Reference-hardware development enrollment (Phase 0)
1. Boot the Tier-1 reference device into UEFI firmware setup.
2. Enter Secure Boot key management (varies by firmware vendor).
3. Enroll the Continuum development PK/KEK/db (test keys, distinct from
   production HSM keys — see `keys/README.md`).
4. Re-enable Secure Boot enforcement.
5. Boot the Continuum reference image; confirm it boots. Confirm a
   deliberately-corrupted image (see `../../tests/hil/test_secure_boot_tamper_detection.md`)
   is refused.

## OEM factory enrollment (Phase 6)
Production key enrollment at OEM factories is part of the OEM
certification kit, not this doc — see
`../../../phase-6-oem-certification-ga/oem-cert-kit/README.md`.
