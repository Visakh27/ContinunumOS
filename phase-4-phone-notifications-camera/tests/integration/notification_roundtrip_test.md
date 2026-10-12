# Integration test skeleton: notification round trip

1. Emulated Companion posts a notification; assert it shows in the center as `Mirrored` with phone and app labels.
2. Dismiss on desktop; assert the phone gets exactly one `DismissRequest` and answers with the echo; no loop.
3. Post and dismiss on the phone; assert the desktop removes it, and sends nothing back.
4. Post a `SECRET` and a `PRIVATE` notification; assert the first never arrives and the second is redacted.
5. Enable desktop DND; assert presentation is suppressed but history is kept (same rule as native).
Not runnable: needs implementations and harness.
