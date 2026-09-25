# App Ecosystem & Compatibility

- **ContinuumKit** — the native app SDK (UI toolkit, system APIs,
  sandboxing) that first-party and third-party developers target,
  distributed through a Continuum App Store.
- **Linux app compatibility:** since Continuum OS is Linux-kernel-based,
  existing Linux desktop apps (via Flatpak/AppImage) can run with
  minimal porting, immediately bootstrapping the software catalog.
- **Windows app compatibility:** a bundled, sandboxed compatibility layer
  (Wine/Proton-derived) for running common Windows desktop applications,
  similar in spirit to how Proton lets SteamOS run Windows games and
  CrossOver runs Windows apps on macOS/Linux; scoped to "best effort,"
  not full guaranteed compatibility, in v1.
- **Browser as a first-class citizen:** a Chromium-based Continuum
  Browser ships by default, since browser-tab Handoff (`continuum-handoffd`,
  see `06-continuity-subsystem.md`) is one of the flagship continuity
  features.

## Developer portal & SDK stability **[added]**

The source plan scopes ContinuumKit as a Phase 1 deliverable but says
nothing about how third-party developers actually get access to it. For
a public alpha (Phase 3) to be meaningful, developers need, before then:

- A published API reference and sample apps (Files-equivalent, a
  Handoff-aware text editor, a ContinuumKit "hello world").
- An explicit API stability policy (what's frozen at alpha vs. what can
  still break) so third parties don't build against churn.
- A packaging/signing/review pipeline for the App Store distinct from
  the OS's own A/B update pipeline.

Tracked in `../../cross-cutting/developer-portal/README.md`, targeted to
land alongside the Phase 1 app-runtime work so it's ready before Phase 3.
