# Contributing

## Session discipline
This project is built in daily 1-hour sessions tracked in
`SESSION_LOG.md`. Keep each session's change scoped to that session's
"Focus" row — a session that sprawls across multiple rows defeats the
point of pacing the work this way.

## Coding standards (once implementation begins)
- Rust: `rustfmt` + `clippy` clean, `cargo nextest` for tests.
- Kotlin: `ktlint`.
- Every new daemon or module: a README.md following the pattern already
  used throughout this repo (scope, planned module layout, status).
- No daemon gains a new kernel-adjacent privilege without an update to
  `docs/architecture/07-security-permission-model.md` and a
  corresponding test in that phase's `tests/security/` or
  `tests/protocol/`.

## Documentation standards
- Architecture decisions with real trade-offs get an ADR in `docs/adr/`,
  not just a paragraph in a phase README.
- Every phase README states explicit entry/exit criteria — no phase
  exit is just "we ran out of things to build."
