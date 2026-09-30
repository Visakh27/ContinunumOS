# Integration test skeleton: app runtime end-to-end (sandbox + capability broker + accessibility tree)

**Level:** integration, headless compositor, no physical hardware.
Supersedes/extends `flatpak_app_launch_test.md` (Session 5) by covering
the three `app-runtime/src/` submodules designed in Session 14 working
together, not just the sandbox in isolation.

1. Launch a reference Flatpak app requesting `Capability::Camera`.
2. Assert: app is sandboxed (no filesystem/network access beyond
   default scope) per `flatpak_app_launch_test.md`'s existing assertions.
3. Assert: camera request triggers `CapabilityBroker::request` — no
   camera access granted without it (negative test on the happy path).
4. Simulate user granting the prompt; assert app subsequently receives
   camera access; assert a *second* request for the same capability does
   **not** re-prompt (per `capability_broker_test.md`'s
   `test_granted_capability_not_re_prompted_on_subsequent_request`).
5. Assert the app's standard ContinuumKit button/list widgets appear in
   an `Accessible` tree query without any app-side accessibility code
   (per `accessibility_tree_test.md`'s automatic-implementation guarantee).
6. Close app; assert capability grants for that `AppId` are retained
   across the *session* but the accessibility tree entry is torn down
   (matches `flatpak_app_launch_test.md`'s existing teardown assertion).

Not runnable — pending `app-runtime/src/{sandbox,capability-broker,accessibility-tree}/`
implementations (all Session 14 design-only).
