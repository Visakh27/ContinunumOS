# Unit test skeletons (Android): pairing and discovery (JUnit + MockK)

PairingHandshake:
- initiate returns accepted=true for a compatible version
- confirm with a wrong proof returns confirmed=false and stores nothing in the Keystore
- rotateTrustAnchor rejects a stale rotation_epoch
- revoke requires a valid signature

Discovery:
- idle -> scanning -> candidateFound -> pairable transitions
- discovery does not run unbounded in the background (Doze/App Standby guard)

Not runnable: pending implementation.
