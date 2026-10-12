# continuumd D-Bus API

The local D-Bus surface every other continuity daemon and the shell
consume. No continuity feature talks to the network directly.

```
Interface: os.continuum.Continuumd1
Methods:
  GetPairedDevices() -> Vec<PairedDeviceStatus>
  GetSessionState() -> SessionState
  SetFeatureEnabled(feature, enabled)
  GetFeatureEnabled(feature) -> bool
Signals:
  DeviceConnected(device)
  DeviceDisconnected(device_id, reason)
  FeatureToggled(feature, enabled)
```

Feature daemons register their own interfaces (e.g.
`os.continuum.Clipboardd1`) but query connectivity and toggle state
through this shared, narrow surface, so "a compromised clipboard daemon
cannot reach the input-injection daemon" holds by construction.

## Mapping to the Phase 1 mock
`ContinuityStatusSource` in
`../../../phase-1-shell-compositor/shell-skeleton/menubar/README.md`
maps 1:1: `paired_devices()` -> `GetPairedDevices()`,
`feature_toggle_state()` -> `GetFeatureEnabled()`,
`set_feature_toggle()` -> `SetFeatureEnabled()`. No trait change needed.

## Status
API surface designed. No D-Bus implementation.
