# Integration test skeleton: Handoff resume affordance in the Dock

1. Emulated Companion advertises an `ActivityAdvertisement`.
2. Assert `DaemonHandoffSource::active_handoff()` returns it and the Dock shows one affordance.
3. Advertise a second activity; assert it replaces the first.
4. Withdraw it; assert the affordance disappears.
5. Drop the session to `Lost`; assert the affordance clears.
Not runnable: needs daemon, Dock and harness.
