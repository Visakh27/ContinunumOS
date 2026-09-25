# External Display / Dock / Thunderbolt-USB4

See `../../docs/architecture/03-driver-hardware-strategy.md`. Distinct
from the compositor's multi-monitor *software* support
(`../../phase-1-shell-compositor/continuumwm/`) — this is the
hardware-enablement side: DisplayPort Alt Mode over USB-C,
Thunderbolt/USB4 tunneling, hub power-delivery negotiation, hot-plug
handling.

## Scope (initial)
- Kernel/udev-level hot-plug detection and mode-setting handoff to
  ContinuumWM.
- Dock power-delivery negotiation (charging through the same port used
  for display/data).
- Its own line item in the Tier-1 certification checklist
  (`../../docs/architecture/03-driver-hardware-strategy.md`), separate
  from internal-display GPU acceleration.

## Status
Not started — begins in Phase 0 hardware-enablement backlog, exercised
end-to-end once the Phase 1 compositor's output-management code exists
(ADR 0005 flags virtual-output design needing to account for this).
