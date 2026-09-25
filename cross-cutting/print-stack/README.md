# Print Stack

Table-stakes desktop OS feature, absent from the source plan
(`../../docs/architecture/09-additions-and-gaps.md`).

## Scope (initial)
- CUPS-based printing (IPP Everywhere / driverless printing as the
  primary path, matching the "certified hardware, best-effort beyond
  that" philosophy already used for Tier-1/Tier-2 device support).
- Print dialog integrated into ContinuumKit as a standard system sheet,
  not a per-app reimplementation.

## Status
Not started — starts Phase 1, low priority relative to compositor/shell
core work; must land before Phase 3 public alpha for the OS to be usable
as a daily driver.
