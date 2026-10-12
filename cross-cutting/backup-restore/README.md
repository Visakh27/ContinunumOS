# Backup & Restore

See `../../docs/architecture/09-additions-and-gaps.md`. Needed before
public alpha; Phase 2 introduces the first daemon (`continuumd`) able to
use the phone as a destination.

## Destinations
```rust
pub enum BackupDestination {
    LocalExternalDrive { mount_path: PathBuf },
    PairedPhone { device_id: DeviceId },
    // PairedPhone has no working transport until continuum-dropd (Phase 3).
    // Modeled now so backup config needs no breaking change later.
}
```

## Scope
User-data partition only. The OS partition is never a backup target
(ADR 0003): it is reprovisioned via A/B or a fresh OEM image.

## Restore
Recovery mode's "reinstall OS, preserving user data" action invokes this
module's restore; it does not reimplement it.

Status: destination model only. No scheduling, encryption policy or
restore flow yet.
