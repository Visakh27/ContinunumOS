# Phone app (built-in desktop app)

The desktop app surfacing `continuum-phoned`: calls, messages and phone
status. A ContinuumKit app like any other, so it uses the capability
broker, accessibility tree and RTL layout rules.

## Screens
- **Calls:** dial pad, incoming/active call view with answer, reject, hang
  up, mute. Shows "phone-only" and a "Connect Bluetooth" prompt when HFP
  isn't connected (ADR 0007). Recents list holds only calls seen while
  connected, because the Companion does not read the phone's call log
  (see Android notes).
- **Messages:** conversation view built from messaging notifications
  (`continuum-phoned/README.md`, Messaging). Reply box enabled only when
  the notification's reply action is still available; otherwise shows why.
- **Status:** battery, signal, and connection state from `continuumd`.

## Decisions
1. **Message retention is off by default.** Fragments live in memory for
   the session. An optional local history (encrypted at rest) is a later,
   explicit opt-in; messages are among the most private data the OS will
   touch.
2. **No contact sync in Phase 4.** Caller names come from the phone's own
   caller-ID text when available, else the number. Contact sync is a
   separate, privacy-heavy feature not in this phase.
3. **Notifications and the app stay consistent:** dismissing a message
   notification does not delete the conversation view, but replying after
   the phone removed the notification fails with a clear message.

Design only.
