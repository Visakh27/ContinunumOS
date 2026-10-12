# Camera (Android side)

Implements `phase-4-phone-notifications-camera/continuum-camerad/camera.proto`.

```kotlin
class CameraStreamService : Service() {          // foreground service, type = camera
    fun start(req: StartCameraStream): StartCameraStreamResponse { TODO() } // CameraX + MediaCodec H.264
    fun onCameraTakenByOtherApp() { /* -> PAUSED_CAMERA_BUSY, retry up to 30 s */ }
    fun stop(id: String) { TODO() }
}
class ScanCapture { fun capture(req: ScanRequest, documentMode: Boolean): ScanComplete { TODO() } }
```
- Runs as a foreground service with a visible notification; recent
  Android versions require the camera foreground-service permission.
- Refuses to start unless the request arrives from a paired, connected
  desktop with the feature enabled; rejects with `permission_denied` if
  the camera permission was revoked.
- Hardware H.264 encoder availability and supported modes differ by
  device; advertise only modes verified to work in `CameraCapabilities`.
- Scan results go through the drop path, not a separate channel.
Interface sketch only.
