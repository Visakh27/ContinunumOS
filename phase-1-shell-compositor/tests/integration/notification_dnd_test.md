# Integration test skeleton: notification center + do-not-disturb

**Level:** integration, no physical hardware.

1. Post a `NativeApp` notification via the ContinuumKit posting API;
   assert it appears in `NotificationCenter::history()`.
2. Enable DND (`set_dnd(true)`); post another notification; assert it's
   suppressed from the visible panel but still recorded in history
   (design intent, not yet explicit in the README — flagged here so the
   implementer notices the gap rather than guessing).
3. Dismiss the first notification; assert a `NotificationObserver` is
   notified with `NotificationSource::NativeApp` — sets up the exact
   hook Phase 4's `continuum-phoned` dismiss-sync will subscribe to,
   tested now against a test-double observer since the real one doesn't
   exist yet.

Not runnable — pending `notification-center/` implementation. Also
files a small spec gap (DND-suppressed-but-still-recorded) discovered
while writing this test — added to
`../../../docs/issues/phase-0-open-items.md`'s sibling file for Phase 1
(see Session 21 exit review, to be created).
