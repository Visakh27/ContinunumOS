# HIL test skeleton: notification listener survives OEM battery management

Per OEM phone: pair, enable mirroring, lock the phone, leave idle 2 hours,
send notifications. Assert they arrive and the listener was not killed.
Record the OEM setting that was needed (battery exemption, autostart) so
onboarding can guide the user. Known to vary widely by vendor.
Blocked on the OEM phone matrix.
