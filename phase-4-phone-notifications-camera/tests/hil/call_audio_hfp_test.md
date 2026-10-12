# HIL test skeleton: call audio over Bluetooth HFP (tests ADR 0007)

This test decides whether ADR 0007 stands. On each Tier-1 device with
each phone in the OEM matrix:
1. Pair classic Bluetooth; place and receive a real call.
2. Assert call audio plays on desktop speakers and is picked up by the desktop mic, both directions.
3. Assert answer, reject, hang up and dial work via HFP control.
4. Measure audio quality (narrowband or wideband negotiated) and record it.
5. Assert desktop audio profile behavior before and after the call is sane.
Record per-device pass/fail. If the desktop cannot reliably act in the hands-free role on the target stack, reopen ADR 0007 (P4-1).
Blocked on a Tier-1 device (P0-1), the phone matrix and the Bluetooth stack in the image.
