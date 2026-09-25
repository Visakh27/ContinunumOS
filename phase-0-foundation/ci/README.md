# CI/CD Pipeline

## Gates (per `../../docs/testing/00-testing-strategy.md`)

| Trigger | Runs | Blocking? |
|---|---|---|
| Every commit (PR) | Unit tests (all crates/modules touched) | Yes |
| Every merge to main | Unit + integration tests | Yes |
| Nightly | HIL boot/driver/continuity regression on Tier-1 fleet + Tier-2 sample | No (alerts on-call; blocks *next* release train, not the merge that caused it) |
| Every kernel patch (`../kernel/patches/`) | Automated boot test across the certified device matrix, before merge | Yes |
| Pre-release (every release train) | Update-path testing: simulated power loss / corrupted update mid-flash against A/B rollback | Yes |
| Pre-major-release | Security: independent pen test, sandbox-escape suite, protocol fuzzing | Yes |

## Pipeline definitions (skeletons)

- `pipelines/unit.yaml` — placeholder CI config (tool-agnostic pseudo-YAML;
  concrete CI system — GitHub Actions/GitLab CI/Buildkite — not yet
  selected, tracked as a Session 6 follow-up decision)
- `pipelines/kernel-patch-gate.yaml` — placeholder, HIL boot test per patch
- `pipelines/nightly-hil.yaml` — placeholder, full fleet regression
- `pipelines/release-train.yaml` — placeholder, update-path + security gates

## HIL lab integration
Remote power control and serial-console log capture for the physical
Tier-1/Tier-2 device lab this CI depends on is a provisioning task
outside repository scope — see
`../../docs/testing/test-environment-setup.md`.
