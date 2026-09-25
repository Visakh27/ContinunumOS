# HIL test skeleton: GPU-accelerated multi-monitor

Requires Tier-1 reference hardware with 2+ physical outputs.

1. Boot to desktop with two monitors connected.
2. Assert both outputs are GPU-composited (not software fallback).
3. Move a window across the monitor boundary; assert no visible tearing
   or frame drop beyond the budget (budget TBD).
4. Hot-unplug one monitor; assert graceful reflow, no compositor crash.

Not runnable — pending compositor implementation + HIL lab provisioning.
