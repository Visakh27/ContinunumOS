# ADR 0006: Virtual Camera Approach

**Status:** Proposed (PipeWire primary, V4L2 compatibility optional); needs the open questions below answered

## Context
`continuum-camerad` must make the phone's camera usable by desktop apps as
a webcam. The source plan says "a native V4L2 device". Options:
- **(a) v4l2loopback.** Works with any app that opens `/dev/video*`. To my
  knowledge it is an out-of-tree kernel module, not part of mainline
  Linux (verify against the pinned LTS tree). Our kernel enforces module
  signing (`CONFIG_MODULE_SIG_FORCE=y`), so we would have to build and
  sign it ourselves, and DKMS-style building on the user's machine does
  not fit a read-only, verified root.
- **(b) PipeWire virtual camera node, offered through the camera portal.**
  No extra kernel module. Apps that use the PipeWire/portal camera path
  (sandboxed apps, which is the default in ADR 0004, and modern browsers)
  get it. Apps that open `/dev/video*` directly do not.
- **(c) Both:** (b) as the primary path, (a) as an optional compatibility
  layer.

## Decision
**(c), with (b) first.** Phase 4 delivers the PipeWire node and portal
integration. The V4L2 loopback compatibility device is optional and
**off by default**; whether to ship it at all is decided after the
security review of the module (it adds kernel attack surface, and the
module would sit inside our signed image).

## Consequences
- Our sandbox model already routes camera access through the permission
  broker/portal, so the primary path gets per-app consent for free
  (`Capability::Camera`).
- Legacy apps that open `/dev/video*` directly won't see the phone camera
  unless the compatibility layer ships. This deviates from the source
  plan's wording; flagged to the owner (open item P4-3).
- `phase-0-foundation/kernel/config/continuity.kconfig` lists
  `CONFIG_VIDEO_V4L2LOOPBACK=m`. That symbol is probably not a mainline
  Kconfig option; it is corrected in Session 47 (open item P4-4).
- Open: if the compatibility layer ships, who maintains the out-of-tree
  module across LTS bumps.
