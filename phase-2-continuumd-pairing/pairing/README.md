# Pairing Protocol

Wire schema: `pairing.proto`. Security requirements it implements
(`../../docs/architecture/07-security-permission-model.md`): mutual
Ed25519 key exchange, rotating QR/numeric-comparison confirmation,
TPM-backed trust anchors.

## Flow
1. `InitiatePairing` <-> `InitiatePairingResponse`: ephemeral key
   exchange and confirmation-method negotiation.
2. Out-of-band confirmation (QR scan or numeric compare). Deliberately
   NOT carried over this protocol: an out-of-band channel is what makes
   MITM detection meaningful.
3. `ConfirmPairing` <-> `ConfirmPairingResponse`: proves both sides saw
   the same code; issues the persisted `TrustAnchor`.
4. Later: `RotateTrustAnchor` and `RevokePairing` (also used by
   recovery mode's "wipe + re-pair").

## Status
Schema complete. No implementation.
