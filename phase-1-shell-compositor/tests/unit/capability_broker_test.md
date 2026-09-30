# Unit test skeleton: capability broker

Implementation under test (not yet written):
`../../app-runtime/src/capability-broker/`

- `test_app_denied_filesystem_access_by_default`
- `test_granted_capability_not_re_prompted_on_subsequent_request`
- `test_denied_capability_not_re_prompted_but_stays_denied`
- `test_capability_grant_scoped_to_requesting_app_only`
  (app A's grant must not leak to app B)
- `test_continuity_dbus_surface_capability_gated_per_feature`
  (granting clipboard access must not implicitly grant handoff access)

Not runnable — pending `src/capability-broker/` implementation.
