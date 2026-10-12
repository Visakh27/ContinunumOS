# Protocol test skeleton: session-loss behavior per daemon

Required by the phase exit criteria; policy defined in
`../../phase-2-continuumd-pairing/continuumd/src/session/README.md`.

- **clipboardd:** event sent while `Lost` is dropped, not queued or replayed; after reconnect, the next copy syncs normally.
- **handoffd:** advertisements clear on `Lost`; a resume in flight fails with a clear message; the user can retry after reconnect.
- **dropd:** reconnect inside the grace window resumes from the last acked offset; outside it the transfer fails with a user-visible error and the partial file is deleted; a pairing rotated or revoked during the gap blocks resume.

Not runnable: needs the pairing harness and daemon implementations.
