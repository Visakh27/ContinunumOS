# continuum-phoned

Mirrors phone notifications into the native notification center, handles
messaging (SMS, RCS and other messengers) through notification replies,
and drives call state/control and call-audio routing. Wire schema:
`phoned.proto`.

## Modules
- `src/notifications/`: mirror, filter, dedup, dismiss-sync, actions.
- `src/messaging/`: message threads and inline reply.
- `src/call_audio/`: call state and routing (see ADR 0007).

## Notifications
Target: `NotificationSource::Mirrored` in
`../../phase-1-shell-compositor/shell-skeleton/notification-center/README.md`.
That variant currently carries only `android_package`. Mapping a desktop
dismissal back to the phone needs the phone's `key`, so `Mirrored` gains
a `remote_key: String` field (additive; native-app path unchanged).

1. **Filtering (phone side, Companion):** skip `SECRET`, group summaries,
   and ongoing/low-priority notifications by default. Per-app allow-list
   in Continuum Center; a new app is off until the user enables it.
2. **Privacy:** `PRIVATE` notifications are redacted on the desktop
   ("New message") unless the user opts in per app. Content is never
   logged and never goes to telemetry.
3. **Dedup:** key = `(remote_key, posted_at)`. An update with the same
   key replaces the existing entry; a repost with an older
   `posted_at` is ignored. After reconnect, `NotificationSyncRequest`
   returns the active set, so nothing is replayed from history.
4. **Dismiss-sync:** a desktop dismissal sends `DismissRequest`; the
   phone cancels it and answers `NotificationRemoved{DISMISSED_ON_DESKTOP}`.
   A phone-side dismissal sends `NotificationRemoved` and the desktop
   removes it. The center's `NotificationObserver::on_dismissed` hook
   (Phase 1) is what triggers the desktop-to-phone direction. Neither
   direction may echo: a removal received from the other side never
   generates a new dismiss request.
5. **Actions:** the desktop cannot hold a phone `PendingIntent`. It sends
   `ActionInvoke{key, action_index, reply_text}` and the Companion fires
   the action. Fails cleanly with `notification_gone` if the phone
   already removed it.
6. **DND:** desktop do-not-disturb suppresses presentation but still
   records history, same rule as native notifications. Phone DND state
   is not mirrored in Phase 4.
7. **Session loss:** state is soft. On `Lost` the desktop keeps showing
   mirrored notifications greyed out for the grace window, then clears
   them; on reconnect it requests a snapshot. Dismiss and action
   requests made while `Lost` fail immediately with a visible error;
   they are not queued, because acting on a stale notification is worse
   than a retry.

## Messaging (SMS, RCS, other messengers)
Android does not let a third-party app read or send RCS, and SMS access
is restricted by Google Play policy (apps need to be the default SMS
handler or qualify for a narrow exception). So Phase 4 mirrors messages
**through message notifications**: the Companion reads the
messaging-style notification and sends the conversation fragment; a
desktop reply goes back as an `ActionInvoke` with `reply_text` on the
notification's reply action. This works for SMS, RCS in Google Messages,
and third-party messengers uniformly.

Limits, stated plainly for beta docs: no message history (only what
arrives while connected), reply works only while the notification is
still present, no sending a brand-new message to an arbitrary number,
and no attachments beyond what the notification exposes. Full
send-anything SMS would need the Companion to become the default SMS
app, a large product decision left to the owner (open item P4-2).

## Calls
Call audio is not handled here; it goes over Bluetooth (ADR 0007).
`CallState` drives the incoming-call notification and the Phone app.
`CallControl` over the session is the fallback when Bluetooth hands-free
control isn't connected.

## Privilege
No network socket, no filesystem beyond its runtime directory, talks only
to `continuumd`'s D-Bus API, the notification center and (for audio) the
audio service. It never touches input injection.

## Status
Design and schema only. No implementation.
