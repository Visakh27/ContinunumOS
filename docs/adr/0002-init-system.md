# ADR 0002: Init System Selection

**Status:** Proposed — open until Phase 0 boot-time/attack-surface data available

## Context
Need an init system managing boot sequence, service supervision, and the
continuity daemon suite. Options: systemd (mature, widely supported,
larger attack surface) vs. a purpose-built minimal init (smaller attack
surface, more engineering cost, less ecosystem tooling).

## Decision
**Deferred.** Phase 0 will prototype both on the Tier-1 reference image
and compare boot time, attack surface (LOC in PID 1, CVE history), and
integration cost with NetworkManager/udev/D-Bus before closing this ADR.
Default assumption for scaffolding purposes: systemd, as the
lower-initial-risk option, revisited at Phase 0 exit review
(`SESSION_LOG.md` session 11).

## Consequences of deferring
Downstream skeletons (`phase-0-foundation/boot/`, daemon unit files)
are written against systemd unit-file conventions as the working
assumption, isolated behind a thin service-definition abstraction so a
later switch doesn't require rewriting every daemon's service
description from scratch.
