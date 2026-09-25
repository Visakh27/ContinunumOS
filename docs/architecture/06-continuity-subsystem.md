# Native Android Continuity Subsystem

This is the core differentiator of Continuum OS: instead of a Windows
tray app polling for a phone, these are OS-level daemons that start at
boot alongside networking and display services, with dedicated
kernel-adjacent privileges (raw BLE/Wi-Fi radio access, compositor-level
input injection, virtual display and virtual camera device nodes) that a
sandboxed third-party app could never be granted.

## 8.1 `continuumd` — the core continuity daemon

- Runs as a system service from boot; owns BLE advertisement/scanning,
  mDNS discovery, and the persistent QUIC/WebRTC session to the paired
  phone.
- Exposes a local D-Bus API that every other continuity daemon and the
  desktop shell consume — no continuity feature talks to the network
  directly.

Skeleton: `../../phase-2-continuumd-pairing/continuumd/`.

## 8.2 Feature daemons (each independently toggleable in Continuum Center)

| Daemon | Function | Phase |
|---|---|---|
| `continuum-clipboardd` | Bidirectional clipboard sync (text/image/file), OS-level clipboard hook rather than an app polling loop | 3 |
| `continuum-handoffd` | Tracks active-app/document/browser-tab state via a ContinuumKit API apps opt into; surfaces the resume affordance in the Dock | 3 |
| `continuum-dropd` | Wi-Fi Direct/Wi-Fi Aware file transfer; integrated directly into the Files app and any app's native Share action, with AV-scan-on-receive | 3 |
| `continuum-phoned` | Notification mirroring, SMS/RCS, and call audio routing, surfaced through the native OS notification center and a built-in Phone app | 4 |
| `continuum-camerad` | Exposes the phone's camera as a native V4L2 device, so any app can use it as a webcam | 4 |
| `continuum-controld` | Universal Control: owns a virtual display output for the mirrored phone screen and injects input at the compositor level | 5 |
| `continuum-hotspotd` | Automatic tethering fallback, integrated with NetworkManager/iwd for seamless network-path switching | 5 |

## 8.3 Android-side companion

Because the phone still runs Android, Continuum OS ships (and
open-sources) a **Continuum Companion** service for Android: a Kotlin
foreground service plus the permission surfaces the app-based approach
would need (Notification Listener, Accessibility Service for input,
Telecom/Telephony APIs, CameraX). It speaks the identical wire protocol
as `continuumd`, so from the OS's perspective the phone is just another
Continuum-protocol endpoint.

Skeleton: `../../android-companion/`.

## Daemon design principles (apply to every daemon above)

1. **Least privilege per daemon.** Each continuity daemon gets only the
   specific kernel-adjacent privilege its feature needs (e.g.
   `continuum-camerad` gets a virtual V4L2 device node, not raw input
   injection). A compromised clipboard daemon must not be able to reach
   the input-injection daemon — see `07-security-permission-model.md`.
2. **No direct network access except `continuumd`.** Feature daemons
   talk to the phone only via `continuumd`'s local D-Bus API, never
   opening their own sockets to the network. This bounds the attack
   surface to one daemon's network code.
3. **Independently toggleable and independently sandboxed.** A user can
   disable `continuum-controld` without affecting `continuum-clipboardd`,
   and a crash or compromise in one does not take down another.
4. **Protocol-first.** Every daemon's wire format is a versioned
   protobuf schema reviewed and tested (`tests/protocol/`) before the
   daemon implementation, so the Android Companion can be built against
   a stable contract rather than reverse-engineering daemon behavior.

## Gap identified during scaffolding **[added]**

The source plan doesn't specify what happens when `continuumd`'s
persistent session drops mid-transfer (e.g. a `continuum-dropd` file
transfer, or a `continuum-controld` input-injection session, while the
phone walks out of Wi-Fi range). Each feature daemon's design doc (see
its phase folder) must define a session-loss behavior: resume, fail
cleanly with user notification, or hand off to `continuum-hotspotd`'s
tethering fallback. This is called out explicitly in each daemon's
`tests/protocol/` plan as a required test case.
