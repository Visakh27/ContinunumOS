# Protocol test skeleton: malformed phone input (Phase 4)

Fuzz `phoned.proto` and `camera.proto` parsers, the notification icon PNG
decoder, and the H.264 decode path (the highest-risk target; separate
sandboxed process). Required for every message type and decoder:
- truncated, oversized and wrong-type fields rejected without panic
- length fields larger than the remaining buffer rejected
- no unbounded allocation from attacker-controlled sizes
- decoder crash contained to its sandbox process; stream ends cleanly
Bounded runtime in CI; long runs before beta.
