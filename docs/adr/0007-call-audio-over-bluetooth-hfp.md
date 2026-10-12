# ADR 0007: Call Audio Over Bluetooth HFP

**Status:** Accepted for design; role support on the target stack still to be verified (see Consequences)

## Context
`continuum-phoned` must let the user take phone calls on the desktop. The
phone's call audio has to reach the desktop speakers and microphone.
Options:
- **(a) Companion captures and streams call audio.** Not viable: since
  Android 10 third-party apps cannot capture a phone call's audio; that
  access is reserved for system apps.
- **(b) Desktop acts as a Bluetooth hands-free unit (HFP).** The phone
  already routes call audio to Bluetooth headsets; the desktop presents
  itself as one. This is the mechanism existing desktop-phone integrations
  rely on.
- **(c) Route calls through a VoIP app.** Doesn't cover normal cellular
  calls.

## Decision
Use **(b)**. Call audio and basic call control (answer, reject, hang up,
dial) go over Bluetooth HFP. The continuity session carries only call
*state* (caller, ringing/active/ended) so the shell can show who is
calling. `CallControl` over the session is a fallback when HFP control is
not connected (`continuum-phoned/phoned.proto`).

## Consequences
- **Second pairing.** HFP needs classic Bluetooth (BR/EDR) pairing, separate
  from the BLE-based continuity pairing. Onboarding must guide the user
  through it, and the desktop hardware must have classic Bluetooth. Tier-1
  certification (`docs/architecture/03-driver-hardware-strategy.md`) gains a
  classic-Bluetooth requirement.
- **Audio quality is whatever HFP negotiates.** Narrowband or wideband
  depending on both devices; better codecs need newer profile support.
  Set user expectations in beta docs.
- **Profile behavior risk.** Entering a call may change the Bluetooth
  profile in use on the desktop side. Needs testing on the target
  BlueZ/PipeWire versions.
- **Unverified.** Whether the target BlueZ + PipeWire (or oFono) stack
  supports the desktop in the hands-free role well enough to ship is not
  confirmed. Must be proven on hardware in the Phase 4 HIL pass; if not,
  this ADR is reopened (open item P4-1).
- **Privacy.** Call audio never passes through the Companion or the
  continuity session, so it never touches our code path or telemetry.
