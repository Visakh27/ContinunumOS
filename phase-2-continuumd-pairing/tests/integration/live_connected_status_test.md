# Integration test skeleton: live "Connected" status

1. Pair emulated Companion instance with `continuumd` over the harness.
2. Assert Continuum Center UI (`../../continuum-center-ui/`) reflects
   "Connected" within N ms of handshake completion.
3. Kill the emulated Companion process; assert UI reflects
   "Disconnected" within the session-timeout window
   (`../../continuumd/README.md`'s session-loss behavior).

Not runnable — pending implementations.

## Expanded chain (Session 28)
Exercises discovery -> `pairing.proto` handshake -> D-Bus `DeviceConnected`
-> `DbusStatusSource` in Continuum Center.
1. Pair an emulated Companion with `continuumd` using the real pairing messages.
2. Assert `DeviceConnected` fires within N ms of `ConfirmPairingResponse{confirmed: true}`.
3. Assert the UI shows "Connected" via the signal, with a call-count bound on `GetPairedDevices()` proving it is push-based.
4. Kill the Companion; assert Connected -> Degraded -> Lost, and that the UI shows "Disconnected" only after Lost, not on Degraded.
