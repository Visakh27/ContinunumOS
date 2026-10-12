# Notifications (Android side)

Implements notification handling from
`phase-4-phone-notifications-camera/continuum-phoned/phoned.proto`.

```kotlin
class ContinuumNotificationListener : NotificationListenerService() {
    override fun onNotificationPosted(sbn: StatusBarNotification) { /* filter -> NotificationPosted */ }
    override fun onNotificationRemoved(sbn: StatusBarNotification, reason: Int) { /* -> NotificationRemoved */ }
    fun dismiss(key: String) { cancelNotification(key) }                  // DismissRequest
    fun invoke(key: String, actionIndex: Int, reply: String?) { /* fire PendingIntent, RemoteInput */ }
}
```
Constraints:
- Needs the user to grant **notification access** in system settings; it is
  a special setting, not a normal permission prompt, and can be revoked at
  any time. Handle `onListenerDisconnected` and request rebind; report
  `permission_revoked` to the desktop.
- Never forward `SECRET` visibility notifications, group summaries, or the
  Companion's own notifications (avoids a feedback loop).
- Per-app allow-list, default off for new apps.
- Aggressive OEM battery management can kill the listener; the HIL OEM
  matrix must include a notification-survives-background check.
Interface sketch only.
