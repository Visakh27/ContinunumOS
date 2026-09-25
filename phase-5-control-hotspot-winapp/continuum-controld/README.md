# continuum-controld

Universal Control: owns a virtual display output for the mirrored phone
screen and injects input at the compositor level (a privilege only the
OS itself can safely hold). Highest-privilege continuity daemon in the
system — see
`../../docs/architecture/06-continuity-subsystem.md#daemon-design-principles-apply-to-every-daemon-above`.

## Planned modules
- `src/virtual_display/` — consumes the virtual output pipeline built
  into ContinuumWM in Phase 1 (`../../phase-1-shell-compositor/continuumwm/`,
  ADR 0005)
- `src/input_injection/` — compositor-level input event injection;
  **this module's design must satisfy the least-privilege principle**:
  it injects input only for the active Universal Control session, never
  has standing access to inject input outside that session
- `src/session/` — Universal Control session lifecycle, including
  session-loss handling when the phone leaves range mid-session

Skeleton only. This daemon is the primary subject of the Phase 5
security audit (`../security-audit/`).
