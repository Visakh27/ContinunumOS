# ADR 0001: Adopt Mainline Linux LTS as the Continuum OS Kernel

**Status:** Accepted
**Date:** Phase 0, Session 1

## Context
An independent desktop OS needs a kernel. Options considered: (a) a
fully original kernel, (b) a custom microkernel (seL4-style), (c)
mainline Linux LTS with a Continuum hardware-enablement patch set.

## Decision
Adopt mainline Linux LTS (c). See `../architecture/01-kernel-and-base-system.md`.

## Consequences
- **Positive:** immediate driver coverage, inherited CVE/security
  pipeline, precedent (Android, ChromeOS) that kernel choice doesn't
  constrain UX.
- **Negative:** GPL obligations on kernel-space code and patch
  upstreaming overhead; inherits Linux kernel's occasional ABI churn
  between LTS versions, which the Hardware Enablement team must track.
- **Rejected alternatives:** (a) and (b) both carry multi-year driver
  bring-up cost with no UX benefit for a v1 product — revisit only after
  market traction and a dedicated kernel team exist.
