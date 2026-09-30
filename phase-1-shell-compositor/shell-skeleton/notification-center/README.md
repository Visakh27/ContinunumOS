# Notification Center

Native OS notification model — how Continuum-native apps post
notifications, do-not-disturb state, and notification history. Flagged
as a gap in `../../../docs/architecture/04-desktop-shell-ux.md`: the
source plan only specifies `continuum-phoned` *mirroring* Android
notifications (Phase 4), never the native model that mirroring plugs
into. Built now, independent of Phase 4, so mirroring has something to
plug into rather than needing to invent the native model itself under
schedule pressure.

## Design

```rust
pub struct Notification {
    pub id: NotificationId,
    pub source: NotificationSource,
    pub title: String,
    pub body: String,
    pub actions: Vec<NotificationAction>,
    pub posted_at: Timestamp,
}

pub enum NotificationSource {
    NativeApp(AppId),                    // Continuum-native app via ContinuumKit
    Mirrored { android_package: String }, // continuum-phoned, Phase 4 — added as a
                                          // variant now so Phase 4 doesn't modify
                                          // this enum's existing native-app path
}

pub trait NotificationCenter {
    fn post(&mut self, n: Notification);
    fn dismiss(&mut self, id: NotificationId);
    fn history(&self, window: TimeRange) -> Vec<Notification>;
    fn dnd_active(&self) -> bool;
    fn set_dnd(&mut self, active: bool);
}
```

**DND behavior (clarified Session 21):** a notification posted while DND
is active is suppressed from the visible panel but **is still recorded**
in `history()` — DND affects presentation, not retention. Flagged as
open item P1-5 after `tests/integration/notification_dnd_test.md`
(Session 19) assumed this without the design doc stating it explicitly.

## Dismiss-sync dependency (Phase 4)
`continuum-phoned`'s "dismiss on one side dismisses on the other"
requirement (`../../../phase-4-phone-notifications-camera/continuum-phoned/README.md`)
needs this center's `dismiss()` to emit an event Phase 4 can subscribe
to — modeled as an observer hook now:

```rust
pub trait NotificationObserver {
    fn on_dismissed(&mut self, id: NotificationId, source: NotificationSource);
}
```

## ContinuumKit posting API
Native apps post notifications through a ContinuumKit call, not directly
against `NotificationCenter` — keeps the capability-broker
(`../../app-runtime/src/capability-broker/`) able to gate notification
posting like any other capability, rather than it being an ungated
system call.

## Status
Interface-level design. No implementation, no UI (the panel/tray
rendering of notifications) built yet.
