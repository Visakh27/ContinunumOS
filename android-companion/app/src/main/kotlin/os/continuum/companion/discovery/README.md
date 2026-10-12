# Discovery (Android side)

Kotlin mirror of `phase-2-continuumd-pairing/continuumd/src/discovery/`,
using `BluetoothLeAdvertiser`/`BluetoothLeScanner` and `NsdManager`.

```kotlin
sealed class DiscoveryState {
    object Idle : DiscoveryState()
    object Scanning : DiscoveryState()
    data class CandidateFound(val candidate: CandidateDevice) : DiscoveryState()
    data class Pairable(val candidate: CandidateDevice) : DiscoveryState()
}
interface DiscoveryBackend { fun startBleAdvertising(); fun startNsdDiscovery(); fun stop() }
```

**Android-only constraint:** background BLE scanning is throttled under
Doze/App Standby, so un-paired discovery needs the app foregrounded or
user-initiated pairing (notification or quick-settings tile). It cannot
scan unbounded in the background like the desktop side.

Status: interface sketch only.
