# Audio Stack

PipeWire-based audio/video stack. See
`../../docs/architecture/04-desktop-shell-ux.md` and
`09-additions-and-gaps.md` for why this exists at all (absent from the
source plan, yet `continuum-phoned` needs call-audio routing and any
desktop needs general audio).

## Output/input switching UI — hooks into menu bar
Design lands in `../../phase-1-shell-compositor/shell-skeleton/menubar/`
as a status-area item alongside Continuum Center, following the same
mock-source pattern already established there for continuity data:

```rust
pub trait AudioDeviceSource {
    fn output_devices(&self) -> Vec<AudioDevice>;
    fn input_devices(&self) -> Vec<AudioDevice>;
    fn active_output(&self) -> AudioDeviceId;
    fn set_active_output(&mut self, id: AudioDeviceId);
}
// Phase 1: PipeWireDeviceSource, backed by real PipeWire enumeration —
// unlike the continuity mocks, this has no daemon dependency, so it can
// be a real implementation now rather than a mock.
```

## Call-audio routing (Phase 4 dependency)
`continuum-phoned` (`../../phase-4-phone-notifications-camera/continuum-phoned/`)
will need to route phone-call audio through desktop speakers/mic without
manual device switching. Modeled now as a routing-priority concept this
module owns, so Phase 4 adds a *routing source* rather than redesigning
audio output selection:

```rust
pub enum RoutingPriority {
    UserSelected,           // explicit output/input picked in the menu-bar UI
    ActiveCall,             // continuum-phoned call in progress — overrides UserSelected
                             // for the duration of the call, restores after
}
```

## Status
`AudioDeviceSource` interface + `RoutingPriority` concept designed. No
PipeWire integration code written yet — that's a build-system dependency
(PipeWire needs to be in the base image,
`../../phase-0-foundation/build-system/manifests/`) not yet added to the
Phase 0 minimal-image manifest (intentionally — Phase 0's manifest is
deliberately minimal, audio is a Phase 1 image-layer addition per the
"superset, never a rewrite" principle).

## Call routing (Session 43, per ADR 0007)

Call audio arrives as a Bluetooth hands-free audio device. The audio
service applies `RoutingPriority::ActiveCall` like this:
1. **Ringing:** play the ring through the notification center's sound; do
   not change audio routing yet.
2. **Active:** when `CallState` is `ACTIVE` and the HFP device is
   connected, switch default output and input to it. Remember the
   previous devices.
3. **Manual change during a call:** if the user picks another device in
   the menu bar, that choice (`UserSelected`) wins for the rest of the
   call and is not overridden.
4. **Ended:** restore the remembered devices, unless the user changed them
   during the call.
5. **Speakerphone echo:** use PipeWire's echo-cancel processing when the
   call runs through desktop speakers and microphone.
6. **No HFP connection:** do not pretend the call is on the desktop. Show
   the call as phone-only and offer to connect Bluetooth.

Captioning of call audio (accessibility) would hook here; not designed yet.
