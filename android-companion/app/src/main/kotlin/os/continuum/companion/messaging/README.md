# Messaging (Android side)

Messages are carried by message-style notifications (see
`continuum-phoned/README.md`, Messaging). No SMS or RCS permission is
requested.

```kotlin
fun toChatMessages(n: Notification): List<ChatMessage>   // from MessagingStyle extras
fun replyTo(key: String, text: String): ActionResult      // RemoteInput on the reply action
```
Not possible this way: history, new conversations to arbitrary numbers,
attachments beyond the notification. Becoming the default SMS app would
lift these but is a large product and policy decision (open item P4-2).
Interface sketch only.
