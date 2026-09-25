# Desktop Shell & User Experience

ContinuumShell targets the same clarity and cohesion as macOS's
Aqua/Finder experience, purpose-built rather than themed on top of an
existing desktop environment.

- Global menu bar with a system-wide app menu, clock, and a "Continuum
  Center" status area showing paired Android device(s), battery, signal,
  and continuity feature toggles.
- Dock with running-app indicators, Spaces/virtual-desktop support, and
  a Mission-Control-equivalent overview (gesture- and keyboard-triggered).
- Spotlight-equivalent universal search (files, apps, settings, and —
  uniquely — content on the paired Android phone, such as photos or
  messages, indexed locally and privately).
- Finder-equivalent Files app with a "Nearby Device" sidebar entry
  exposing the paired Android phone's shareable storage, matching macOS
  Finder's iPhone entry.
- System Settings app with a first-class "Continuity" pane covering
  pairing, per-feature toggles, and diagnostics.

## Gaps identified during scaffolding **[added]**

The source plan describes the shell's feature surface but not several
things every shipping desktop shell needs day one:

- **Accessibility affordances** are not mentioned anywhere in the shell
  spec: no screen reader, no switch/scanning input, no captioning, no
  high-contrast/reduced-motion mode. These must be ContinuumKit-level
  APIs (so every app gets them for free), not shell-only features.
  Tracked in `../../cross-cutting/accessibility/README.md`; this is a
  legal requirement (ADA Title III precedent in the US, EN 301 549 in
  the EU) in most launch markets, not just a nice-to-have.
- **Input method editors (IME)** for CJK, Indic, and other non-Latin
  scripts are required for the shell to be usable outside English-first
  markets. Tracked in `../../cross-cutting/localization/README.md`.
- **Notification center** is implied by `continuum-phoned` (Section 8)
  mirroring notifications, but the *native* OS notification model (how
  Continuum-native apps post notifications, do-not-disturb, notification
  history) is not specified independent of the Android-mirroring case.
  Added as a Phase 1 shell-skeleton deliverable.
- **Audio output/input switching UI** (a menu-bar equivalent of macOS's
  volume/output picker) is required for `continuum-phoned` call-audio
  routing to be usable, and isn't mentioned. See
  `../../cross-cutting/audio-stack/README.md`.

Implementation skeleton: `../../phase-1-shell-compositor/shell-skeleton/`.
