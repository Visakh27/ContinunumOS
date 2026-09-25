# continuum-handoffd

Tracks active-app/document/browser-tab state via a ContinuumKit API apps
opt into; surfaces the resume affordance in the Dock
(`../../phase-1-shell-compositor/shell-skeleton/dock/`).

## Planned modules
- `src/state_tracking/` — active-app/document state from opted-in apps
- `src/continuumkit_api/` — the Handoff API surface third-party apps
  target (feeds `../../cross-cutting/developer-portal/`)
- `src/resume/` — Dock resume-affordance trigger

Skeleton only.
