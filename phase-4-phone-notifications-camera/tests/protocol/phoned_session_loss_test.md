# Protocol test skeleton: phoned and camerad session loss

Policy: `../../phase-2-continuumd-pairing/continuumd/src/session/README.md`.
- **Notifications:** on `Lost`, mirrored items grey out for the grace window then clear; on reconnect `NotificationSyncRequest` returns the active set; no history replay.
- **Dismiss/action while `Lost`:** fail immediately with a visible error; nothing queued.
- **Calls:** call state marked stale; HFP audio may continue; session-only controls hidden.
- **Camera:** placeholder during grace, then `ENDED(session_lost)`; no auto resume; a new start needs a new user gesture.
Not runnable: needs the pairing harness and implementations.
