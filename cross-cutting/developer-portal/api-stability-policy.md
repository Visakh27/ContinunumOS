# ContinuumKit API Stability Policy (draft for alpha)

Needed before the public alpha so third parties don't build on churn.

| Tier | Meaning | Examples |
|---|---|---|
| Stable | Source- and binary-compatible within a major version; removal needs one full major version of deprecation | Capability enum (additive only), `HandoffActivity` trait, accessibility tree trait |
| Preview | May change between minor releases with release-note notice | Notification posting API, print sheet API |
| Internal | No guarantees; not for third parties | Daemon D-Bus interfaces other than those listed as stable |

Rules:
- Protocol schemas (`*.proto`) only add fields; never renumber or reuse tags.
- Anything shipped in the alpha is Preview unless this document lists it as Stable.
- Handoff payload cap and size limits are part of the contract and are announced before they change.

Open: who owns the review process, and the deprecation timeline (needs your input).
