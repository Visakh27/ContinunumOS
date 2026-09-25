# Telemetry, Diagnostics & Privacy

See `../../docs/architecture/09-additions-and-gaps.md`. Needed to make
the source plan's own Section 16 KPIs (crash-free boot rate, pairing
success rate, A/B update success rate) measurable, and required before
Phase 3 public alpha.

## Principles
1. **Opt-in by default for anything beyond boot-health.** The Phase 0
   minimal-image manifest (`../../phase-0-foundation/build-system/manifests/phase-0-minimal-image.manifest`)
   includes only a boot success/failure beacon — required for the A/B
   update mechanism's own staged-rollout gating
   (`../../phase-0-foundation/boot/ab-update/README.md`) to function at
   all, so it is not optional in the way broader usage telemetry is.
2. **Published data-handling policy** before any telemetry beyond
   boot-health ships, covering: what's collected, retention period,
   whether it's identifiable/pseudonymous, and how a user opts out.
3. **No continuity-content telemetry, ever.** Clipboard contents,
   message contents, file contents, camera/mic streams — none of it is
   ever telemetry, regardless of opt-in status. Only operational
   metadata (e.g. "a clipboard sync occurred, took 340ms, succeeded") is
   eligible, and only under the opt-in policy above.

## Schema (boot-health beacon, Phase 0 scope only)
```
{
  "event": "boot_attempt",
  "slot": "A" | "B",
  "outcome": "success" | "failure" | "rolled_back",
  "boot_duration_ms": <int>,
  "os_version": "<semver>",
  "device_class": "<tier1-reference-device-id>",
  "timestamp": "<iso8601>"
}
```
No user-identifying or device-serial-identifying field. Device-class is
a model identifier, not a unique-device identifier — sufficient for
fleet-level crash-free-boot-rate KPI tracking without individual device
tracking.

## Status
Boot-health beacon schema defined (above); the broader
usage/diagnostics telemetry policy (covering later phases' pairing
success, clipboard latency, etc. KPIs) is a Phase 2+ follow-up, staged
to land before the Phase 3 public alpha per
`../../docs/testing/phase-3-test-plan.md`.
