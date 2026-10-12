# continuum-camerad

Makes the phone camera usable as a desktop webcam, and provides
scan-to-desktop. Wire schema: `camera.proto`. Approach:
`../../docs/adr/0006-virtual-camera-approach.md` (PipeWire virtual camera
node first; V4L2 loopback compatibility optional and off by default).

## Modules
- `src/virtual_camera/`: creates and removes the PipeWire camera node (and
  the optional V4L2 device if ever enabled); feeds it decoded frames.
- `src/stream/`: mode negotiation, receives encoded H.264 frames from the
  data path, decodes, timestamps, orientation.
- `src/contention/`: handles the phone losing the camera to another app.
- `src/scan/`: scan-to-desktop.

## Streaming
Companion captures with CameraX and encodes H.264 on the phone's hardware
encoder; frames cross the data path (not the protobuf session). Default
720p30, 1080p optional per `CameraCapabilities`. Rotation and mirroring
are applied on the desktop from orientation metadata. The latency budget
is unset until measured (open item P4-6).

## Consent and privacy (the sensitive part)
A remote-controlled camera is the highest-privacy feature in the product.
1. **User-initiated only.** A stream starts only from a desktop user
   action, surfaced as a permission prompt when an app asks for the
   camera (`Capability::Camera`). The phone side can never start a
   stream, and nothing starts one in the background.
2. **Indicators on both devices.** The desktop menu bar shows "Phone
   camera active" with a stop button. The phone shows the foreground-
   service notification Android requires for camera use, plus Android's
   own camera indicator where supported.
3. **No silent resume.** If the session is lost beyond the grace window,
   the stream ends; a new stream needs a new user action (same principle
   as `continuum-controld`).
4. **Never recorded or logged.** Frames are not written to disk by this
   daemon and never reach telemetry.

## Contention (phone camera taken by another app)
Android gives the camera to the foreground app. When the Companion loses
it, it sends `CameraStreamState{PAUSED_CAMERA_BUSY}`. The virtual camera
keeps emitting a "camera unavailable" placeholder frame so desktop apps
don't hang or crash. If the phone camera becomes available again within
30 s (initial value) the stream resumes; otherwise it ends with reason
`timeout`.

## Session loss
`Degraded`: keep streaming, show a quality warning. `Lost`: placeholder
frame during the grace window, then end with `session_lost`. No auto
resume.

## Scan to desktop
Started from the Files app ("Scan from phone") or the Companion. The
phone captures, optionally corrects the document perspective on-device,
and delivers the image through `continuum-dropd`, inheriting its
quarantine and scan checks. Cancellable on both sides. Multi-page
scans and PDF assembly are not in Phase 4.

## Privilege
Create/destroy a camera node and read the data path; no network socket of
its own (the data path comes from `continuumd`), no input injection, no
filesystem beyond its runtime directory.

## Status
Design and schema only. No implementation.
