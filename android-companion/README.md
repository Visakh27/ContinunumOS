# Continuum Companion (Android)

Open-sourced Android companion service. Speaks the identical wire
protocol as `continuumd`
(`../phase-2-continuumd-pairing/continuumd/`), so from the desktop OS's
perspective the phone is just another Continuum-protocol endpoint. See
`../docs/architecture/06-continuity-subsystem.md#83-android-side-companion`.

## Distribution
Google Play + sideloadable open-source APK (per
`../docs/architecture/../` source doc Section 14), since it must live
inside Android's app ecosystem.

## Key permission surfaces (mirrors what the app-based predecessor design needed)
- Notification Listener (for `continuum-phoned` mirroring, Phase 4)
- Accessibility Service, for input handling (for `continuum-controld`,
  Phase 5 — flagged Risk Register item: Google Play policy may restrict
  this; fallback is view-only mirroring without input injection)
- Telecom/Telephony APIs (call audio routing, SMS/RCS, Phase 4)
- CameraX (V4L2 virtual camera source, Phase 4)

## Structure
```
android-companion/
└── app/
    └── src/
        ├── main/kotlin/os/continuum/companion/
        ├── test/           JUnit unit tests
        └── androidTest/    Instrumented tests (real/emulated device)
```

Not yet implemented — module/package skeleton only, matching the
`../phase-2-continuumd-pairing/companion-android-stub/` protocol
contract as it's defined phase by phase.
