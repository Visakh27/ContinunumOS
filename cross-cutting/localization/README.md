# Internationalization & Localization

See `../../docs/architecture/04-desktop-shell-ux.md` and
`09-additions-and-gaps.md`. Starts Phase 1, formally gated at Phase 6.

## Scope (initial)
- RTL layout support in the ContinuumKit UI toolkit and ContinuumShell.
- Input method editor (IME) framework for CJK/Indic/other non-Latin
  scripts, exposed as a system service apps opt into via ContinuumKit,
  not a per-app reimplementation.
- Locale-aware date/number/currency formatting in ContinuumKit.
- String-resource pipeline for first-party apps (shell, Settings, Files,
  Phone) feeding the Phase 6 localization audit
  (`../../phase-6-oem-certification-ga/localization/`).

## Status
Design scope only — implementation begins alongside Phase 1 shell work.
