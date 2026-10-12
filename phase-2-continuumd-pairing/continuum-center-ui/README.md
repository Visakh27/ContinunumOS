# Continuum Center UI

Shell surface (lives conceptually in
`../../phase-1-shell-compositor/shell-skeleton/menubar/`'s status area)
showing paired device(s), battery, signal, and per-feature-daemon
toggles. Consumes `../continuumd/src/dbus_api/`. Skeleton only.

## Real data source (Session 25)

Replaces the Phase 1 `MockStatusSource` with a D-Bus-backed source
against `../continuumd/src/dbus_api/README.md`:

```rust
pub struct DbusStatusSource { connection: DbusConnection /* os.continuum.Continuumd1 */ }

impl ContinuityStatusSource for DbusStatusSource {
    fn paired_devices(&self) -> Vec<PairedDeviceStatus> { todo!() }            // GetPairedDevices()
    fn feature_toggle_state(&self, f: ContinuityFeature) -> bool { todo!() }   // GetFeatureEnabled(f)
    fn set_feature_toggle(&mut self, f: ContinuityFeature, enabled: bool) { todo!() } // SetFeatureEnabled(f, enabled)
}
```

It subscribes to `DeviceConnected` / `DeviceDisconnected` /
`FeatureToggled` signals so the "Connected" status is push-based, not
polled. The `ContinuityStatusSource` trait needed no change, so the
Phase 1 design held up against the real API.

Battery and signal come from the Companion over the session;
`continuumd` cannot measure them itself.

Status: designed only. Needs a D-Bus client library in a build manifest
and `continuumd` itself.
