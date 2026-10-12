# Pairing (Android side)

Implements `phase-2-continuumd-pairing/pairing/pairing.proto`; both
sides generate code from the same schema.

```kotlin
class PairingHandshake(private val keystore: AndroidKeystoreTrustAnchorStore) {
    suspend fun initiate(peer: CandidateDevice): InitiatePairingResponse { TODO() }
    suspend fun confirm(proof: ByteArray): ConfirmPairingResponse { TODO() }
    suspend fun rotateTrustAnchor(deviceId: String): RotateTrustAnchorResponse { TODO() }
    suspend fun revoke(deviceId: String) { TODO() }
}
```

Trust anchors live in the Android Keystore (hardware-backed where
available), the analogue of the desktop TPM keystore. The QR/numeric
confirmation screen is Jetpack Compose and not yet designed.

Status: interface sketch only.
