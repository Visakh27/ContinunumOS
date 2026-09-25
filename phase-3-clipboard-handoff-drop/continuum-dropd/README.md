# continuum-dropd

Wi-Fi Direct/Wi-Fi Aware file transfer; integrated into the Files app
and any app's native Share action, with AV-scan-on-receive.

## Planned modules
- `src/transfer/` — chunking, resume-after-interruption (session-loss
  behavior per `../../docs/architecture/06-continuity-subsystem.md`)
- `src/av_scan/` — scan-on-receive before a transferred file is exposed
  to the Files app or any other app
- `src/share_integration/` — native Share-action integration point

Skeleton only.
