# Session

Persistent QUIC/WebRTC session to the paired phone, including the
**session-loss/resume policy** that closes the gap flagged in
`../../../docs/architecture/06-continuity-subsystem.md`.

## States
```rust
pub enum SessionState {
    Disconnected,
    Connecting,
    Connected { session_id: SessionId },
    Degraded { session_id: SessionId, reason: DegradeReason },
    Lost { last_session_id: SessionId, lost_at: Timestamp },
}
pub enum DegradeReason { WeakSignal, HighLatency, IntermittentPacketLoss }
```

## Policy
On `Connected -> Lost`, `continuumd` attempts silent reconnection for a
bounded grace window (default 30s, configurable) before moving to
`Disconnected` and emitting `DeviceDisconnected` on the D-Bus API.
Each feature daemon decides its own in-flight behavior from the shared
`Degraded`/`Lost` signal:
- `continuum-clipboardd`: fire-and-forget per event; nothing to resume.
- `continuum-dropd`: must resume an in-flight transfer if reconnection
  lands inside the grace window, else fail with a clear user-facing error.
- `continuum-controld`: must NOT resume silently — input injection
  resuming without explicit reconnection confirmation is a
  security-relevant surprise. `continuum-hotspotd` tethering fallback is
  the alternative path, not silent resume.

## Status
State machine + policy designed. No QUIC/WebRTC implementation.
