# continuum-camerad

Exposes the phone's camera as a native V4L2 device.

## Planned modules
- `src/v4l2_device/` — virtual V4L2 device node registration/teardown
- `src/stream/` — camera stream negotiation, codec handling
- `src/contention/` — graceful degradation when the phone camera is in
  use by another app (e.g. the Companion's own camera app)

Skeleton only.
