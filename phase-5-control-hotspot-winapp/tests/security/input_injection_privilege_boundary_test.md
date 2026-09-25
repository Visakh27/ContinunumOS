# Security test skeleton: input-injection privilege boundary

- `test_controld_cannot_inject_input_without_active_session`
- `test_controld_input_injection_scoped_to_session_virtual_display_only`
- `test_compromised_clipboardd_cannot_reach_controld_dbus_surface`
  (cross-daemon isolation, per daemon design principle #3)
- `test_session_ends_cleanly_on_phone_disconnect_no_dangling_injection_grant`

Not runnable — pending `../../continuum-controld/` implementation; this
is also the specification handed to the external pen-test engagement
(`../../security-audit/`).
