# continuum-handoffd

Tracks the active activity (document, browser tab, etc.) from apps that
opt in through the ContinuumKit Handoff API, advertises it to the paired
phone, and surfaces a resume affordance in the Dock. Wire schema:
`handoff.proto`.

## Modules
- `src/continuumkit_api/`: the API third-party apps call.
- `src/state_tracking/`: holds the current local activity and the
  latest remote one.
- `src/resume/`: drives the Dock affordance and the resume flow.

## ContinuumKit Handoff API (what third parties see)
```rust
pub trait HandoffActivity {
    fn activity_type(&self) -> &str;
    fn title(&self) -> String;
    fn continuation_payload(&self) -> Vec<u8>;   // app-defined state, opaque to the OS
    fn web_fallback_url(&self) -> Option<String>;
}
pub struct HandoffSession; // begin(activity) / update(activity) / end()
// Called from the app; gated by Capability::ContinuityHandoff
// (../../phase-1-shell-compositor/app-runtime/README.md).
// The app implements `resume(payload)` to restore state when picked up.
```
This API is a stability commitment; see
`../../cross-cutting/developer-portal/README.md`.

## Design decisions
1. **Opt-in only.** The OS never infers an activity. An app that doesn't
   call the API is invisible to Handoff.
2. **Opaque payload, capped.** The OS never parses `continuation_payload`
   (initial cap 16 KiB). Large state must go through the app's own
   storage or `continuum-dropd`. This keeps the daemon from becoming a
   data path and bounds its attack surface.
3. **Single current activity per device.** A new advertisement replaces
   the previous; the Dock shows at most one resume affordance per
   remote device.
4. **Privacy.** Titles and payloads are never logged or sent to
   telemetry. A user can disable Handoff per app (capability revoke) or
   globally (Continuum Center toggle).
5. **Fallback.** If the receiving device lacks the app, use
   `web_fallback_url` when present; otherwise reject with
   `app_not_installed` and say so in the UI rather than doing nothing.
6. **Staleness.** An advertisement expires if not refreshed (initial
   TTL 5 minutes) or when the session reaches `Lost`, so the Dock never
   offers a resume for something that is gone.
7. **Session loss.** Advertisements are soft state and simply lapse. A
   resume in flight when the session drops fails with a clear message;
   the user retries.

## Dock integration
Implements `HandoffSource` from
`../../phase-1-shell-compositor/shell-skeleton/dock/README.md`, replacing
`MockHandoffSource`:
```rust
pub struct DaemonHandoffSource { /* D-Bus: os.continuum.Handoffd1 */ }
impl HandoffSource for DaemonHandoffSource {
    fn active_handoff(&self) -> Option<HandoffCandidate> { todo!() }
}
```
No trait change was needed.

## Status
Design, API sketch and schema only. No implementation.
