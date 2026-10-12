# continuum-dropd

Wi-Fi Direct / Wi-Fi Aware file transfer, integrated with the Files app
and every app's Share action, with scan-on-receive. Wire schema:
`drop.proto`.

## Modules
- `src/transfer/`: offer/accept, chunking, resume.
- `src/av_scan/`: scan-on-receive before delivery.
- `src/share_integration/`: Share action and Files "Nearby Device" entry.

## Transfer flow
1. Sender sends `TransferOffer` (name, size, SHA-256, chunk size) over
   `continuumd`'s session.
2. Receiver checks policy (feature toggle, size limit, user prompt for
   unknown or large files) and replies `TransferOfferResponse`.
3. Bulk data moves on the high-bandwidth path; the receiver writes to a
   **quarantine directory** and acks with `ChunkAck`.
4. On completion the receiver verifies the SHA-256, runs the scan, and
   only then moves the file to the user's folder.

## Design decisions
1. **Quarantine first.** Nothing received is visible to the Files app or
   any other app until digest verification and scan pass. Required by
   the phase exit criteria.
2. **Scan unavailable policy.** On `SCAN_UNAVAILABLE`, default is to hold
   the file in quarantine and ask the user; never silently deliver.
3. **Filename sanitization.** The received name is never trusted as a
   path: strip separators and control characters, cap length, and
   resolve collisions by suffixing. Prevents path traversal.
4. **Resume.** `ChunkAck.highest_contiguous_offset` lets a transfer
   resume after the session goes `Degraded`/`Lost`, if reconnection is
   inside the grace window (`../../phase-2-continuumd-pairing/continuumd/src/session/README.md`).
   Otherwise the transfer fails with a user-visible error and the
   partial file is deleted. A resume must re-validate the pairing's
   trust anchor epoch: a transfer must not continue under a rotated or
   revoked pairing.
5. **Limits.** Initial maximum file size and a cap on concurrent
   transfers, both configurable; the partial-file quarantine has a disk
   quota and a time-to-live so abandoned transfers don't fill storage.
6. **No network socket of its own.** Control traffic goes through
   `continuumd`, and the bulk data path is opened by `continuumd`'s
   `data_path` service and handed over as an already-connected handle
   (`../../phase-2-continuumd-pairing/continuumd/src/data_path/README.md`,
   changed Session 44; this daemon previously opened it itself).
7. **Hotspot fallback.** Not used for drop. `continuum-hotspotd`
   (Phase 5) covers connectivity, not transfer routing.

## Status
Design and schema only. No implementation.
