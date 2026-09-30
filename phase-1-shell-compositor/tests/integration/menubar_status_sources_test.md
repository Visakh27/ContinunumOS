# Integration test skeleton: menu-bar status sources (audio + Continuum Center)

**Level:** integration, no physical hardware (audio via a mocked
PipeWire backend for CI determinism, even though the real
`AudioDeviceSource` implementation is not itself a mock per
`../../../cross-cutting/audio-stack/README.md`).

1. Menu bar renders with `MockStatusSource` (Continuum Center) and a
   test-double `AudioDeviceSource` both wired in.
2. Assert Continuum Center shows "no device paired" state — the correct
   behavior of the Session 1–15 mock, confirming the UI doesn't assume a
   paired device exists.
3. Simulate an `ActiveCall` routing-priority event
   (`RoutingPriority::ActiveCall`, per the audio-stack design) with no
   real `continuum-phoned` present — assert the menu bar's audio status
   item reflects the override without crashing, i.e. the routing-priority
   concept is wired into the UI even though its Phase-4 producer doesn't
   exist yet.
4. Swap `MockStatusSource` for a fixture reporting one paired device with
   clipboard sync enabled, handoff disabled; assert both toggle states
   render correctly and are independently interactive (regression guard
   for "independently toggleable" per
   `../../../docs/architecture/06-continuity-subsystem.md`'s daemon
   design principles, exercised here at the UI layer even before real
   daemons exist).

Not runnable — pending `menubar/` and `audio-stack/` implementations.
