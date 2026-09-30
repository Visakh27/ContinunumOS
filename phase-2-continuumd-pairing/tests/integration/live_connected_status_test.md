# Integration test skeleton: live "Connected" status

1. Pair emulated Companion instance with `continuumd` over the harness.
2. Assert Continuum Center UI (`../../continuum-center-ui/`) reflects
   "Connected" within N ms of handshake completion.
3. Kill the emulated Companion process; assert UI reflects
   "Disconnected" within the session-timeout window
   (`../../continuumd/README.md`'s session-loss behavior).

Not runnable — pending implementations.
