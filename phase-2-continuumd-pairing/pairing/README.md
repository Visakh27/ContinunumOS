# Pairing Protocol

Mutual Ed25519 key exchange plus rotating QR/numeric-comparison
confirmation; trust anchors in a TPM-backed keystore
(`../../docs/architecture/07-security-permission-model.md`).

## Protocol schema (planned)
`pairing.proto` (to be added) — request/response messages for:
`InitiatePairing`, `ConfirmPairing` (QR or numeric path), `RotateTrustAnchor`,
`RevokePairing`.

Skeleton only.
