# Instrumented tests (real/emulated Android device)

Planned first tests once the foreground service (`../../main/kotlin/os/continuum/companion/daemon/`)
exists:
- `ForegroundServiceLifecycleTest` — service survives Doze/App Standby
  per Android's background-execution limits (a real-world constraint the
  source plan doesn't mention; flagged here so it isn't discovered late).
- `NotificationListenerPermissionTest` — permission grant/revoke handling.
