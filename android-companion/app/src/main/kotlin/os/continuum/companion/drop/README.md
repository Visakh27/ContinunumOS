# Drop (Android side)

Implements `phase-3-clipboard-handoff-drop/continuum-dropd/drop.proto`.

```kotlin
class DropTransfer(private val session: ContinuumSession, private val dataPath: DataPathFactory) {
    suspend fun offer(file: Uri): TransferOfferResponse { TODO() }
    suspend fun receive(offer: TransferOffer): TransferComplete { TODO() }  // quarantine -> verify -> scan
}
```

Data path candidates: Wi-Fi Direct (`WifiP2pManager`) or Wi-Fi Aware
(`WifiAwareManager`, availability varies by device). Newer Android
versions gate nearby-device access behind a runtime permission, so the
pairing/onboarding flow must request it. Both are device-dependent, which
is why the HIL OEM matrix covers transfer, not just pairing.
Files are read through Storage Access Framework URIs; the Companion never
needs broad storage permission.

Status: interface sketch only.
