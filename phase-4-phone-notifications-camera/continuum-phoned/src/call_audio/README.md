# Call audio

State machine for a call as the desktop sees it, driven by `CallState`
from the Companion and HFP events from the Bluetooth stack:

```
Idle -> Ringing -> Active -> Ended -> Idle
              \-> Ended (rejected / missed)
```
- Answer/reject/hang-up use HFP call control when connected, else
  `CallControl` over the session.
- On `Active`, ask the audio service to apply `RoutingPriority::ActiveCall`
  (`../../../../cross-cutting/audio-stack/README.md`).
- If the session is lost mid-call, the audio may continue over Bluetooth
  but call *state* goes stale: show "call in progress (connection lost)"
  and do not offer controls that need the session.
Design only.
