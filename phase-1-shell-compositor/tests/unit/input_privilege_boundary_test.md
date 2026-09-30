# Unit test skeleton: input routing privilege boundary

Implementation under test (not yet written):
`../../continuumwm/src/input/`

- `test_injected_event_only_reaches_its_own_session_virtual_output`
- `test_injected_event_cannot_target_a_physical_output`
  (enforced, not just convention — see design doc's constraint #1)
- `test_revoking_session_stops_event_routing_synchronously`
  (constraint #2 — no queued-event processing after revocation)
- `test_local_and_injected_events_are_distinguishable_downstream`
  (constraint #3)

This test file exists in Phase 1 even though `continuum-controld` (the
only real *producer* of `Injected` events) isn't built until Phase 5 —
it specifies the contract Phase 5 must satisfy, written against the
Phase 1 interface design so the constraint doesn't get discovered late.
Not runnable — pending both `src/input/` and, for full end-to-end
coverage, Phase 5's daemon.
