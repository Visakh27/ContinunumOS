# Backup & Restore

No data-loss story exists in the source plan
(`../../docs/architecture/09-additions-and-gaps.md`). Needed before any
public alpha (Phase 3) — a continuity-focused OS that loses user data on
a failed update is a non-starter for early adopters.

## Scope (initial)
- User-data backup (distinct from the OS-image A/B mechanism, which
  only protects the OS partition, not `/home`-equivalent user data).
- Backup destinations: local external drive, and — leveraging the
  continuity subsystem already being built — the paired Android phone's
  storage as a fallback destination via `continuum-dropd` (Phase 3),
  once that daemon exists.
- Restore path integrated with `../recovery-mode/`'s reinstall flow
  ("reinstall OS, preserving user data" implies a working backup/restore
  mechanism underneath it).

## Status
Design scope only. Targeted to land before Phase 2 exit
(`../../ROADMAP.md`), since Phase 2 introduces the first daemon
(`continuumd`) capable of using the phone as a backup destination.
