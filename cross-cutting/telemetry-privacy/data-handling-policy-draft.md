# Data Handling Policy (draft, closes P2-7 at design level)

Draft for review before public alpha. **Needs legal review before publication.**

1. Boot-health beacon: on by default (required for safe updates); contains no user or unique device identifier.
2. Continuity operational metadata: opt-in, off by default; coarse event data only (see `README.md` schemas).
3. Never collected, under any setting: clipboard, message, file, notification, camera or microphone content; Handoff titles or payloads; file names.
4. Retention: proposed 90 days for operational metadata; boot-health aggregated indefinitely. Both are proposals, not decisions.
5. Opt-out and deletion: one toggle in Continuum Center; deletion by device-class cohort is not possible by design, so explain that clearly.
6. Crash reports: opt-in, scrubbed of content, stack and version only.

Open: retention numbers, jurisdiction handling (GDPR etc.), and who is the data controller.
