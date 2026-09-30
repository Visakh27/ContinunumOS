# Wayland Protocol Implementations

Core Wayland protocol + Continuum-specific extensions (e.g. a
`continuum-virtual-output-v1` protocol extension for
`../output/VirtualOutput`, since virtual outputs aren't part of upstream
Wayland). Built on wlroots per ADR 0005 — this module wraps/extends
wlroots' existing protocol implementations rather than reimplementing
core Wayland from scratch.

## Status
Not started — depends on wlroots integration, which is a build-system
dependency (`../../../phase-0-foundation/build-system/`) not yet wired up.
