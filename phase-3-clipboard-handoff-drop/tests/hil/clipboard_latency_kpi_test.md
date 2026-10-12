# HIL test skeleton: clipboard latency KPI

Target: median <= 500 ms on a local network (plan Section 16).
Run N copies of text, a small image and a near-limit payload per OEM device; record per-event latency from copy to applied. Report median and p95 per device. Feeds `continuity_operation` telemetry.
Blocked on a Tier-1 device (P0-1) and the OEM device matrix.
