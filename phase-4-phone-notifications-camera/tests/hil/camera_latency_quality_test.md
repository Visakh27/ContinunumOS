# HIL test skeleton: camera latency and quality

Per OEM phone and desktop pair: 720p30 and 1080p30 stream for 10 minutes.
Measure glass-to-glass latency (flash-and-capture method), dropped frames,
phone temperature and battery drain, and whether the hardware encoder
mode was available. Results set the latency budget (P4-6) and the
advertised `CameraCapabilities`.
Blocked on a Tier-1 device and the phone matrix.
