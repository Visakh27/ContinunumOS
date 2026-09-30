# HIL test skeleton: virtual desktop switch performance

Requires Tier-1 reference hardware (blocked on
`../../../phase-0-foundation/TIER1_REFERENCE_HARDWARE.md` selection,
open item P0-1).

1. Open 5+ windows across 3 virtual desktops.
2. Switch desktops via keyboard shortcut; measure time from input event
   to compositor-reported frame-complete.
3. **Expect:** switch completes within a to-be-set budget (same
   "budget TBD, set once a reference device exists" pattern as
   `../../../phase-0-foundation/tests/hil/test_secure_boot_tamper_detection.md`
   — open item P0-9's sibling for this phase).
4. Repeat with `LayoutDirection::Rtl` active
   (`../../../cross-cutting/localization/README.md`); assert no
   additional latency from direction-aware layout logic.

Not runnable — pending Tier-1 device selection and
`virtual-desktops/`/`continuumwm/` implementations.
