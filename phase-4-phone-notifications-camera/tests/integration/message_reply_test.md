# Integration test skeleton: message reply

1. Emulated Companion posts a messaging-style notification with a reply action.
2. Phone app shows the conversation fragment; reply box enabled.
3. Send a reply; assert `ActionInvoke` carries the text and the phone-side stub receives it.
4. Remove the notification on the phone; assert the reply box disables with an explanation, and a late reply returns `notification_gone`.
Not runnable.
