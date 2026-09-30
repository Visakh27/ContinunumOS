# Key material policy

No private signing keys are ever committed to this repository. This
directory documents the key hierarchy and rotation process only.

## Hierarchy
1. **Platform Key (PK)** — root of trust, HSM-held, used only to enroll
   the KEK. Rotation: only on suspected compromise (highest-severity
   incident runbook).
2. **Key Exchange Key (KEK)** — HSM-held, authorizes updates to the
   signature database. Rotation: annual, or on suspected compromise.
3. **Signature Database key (db)** — signs the Continuum bootloader and
   kernel. This is the key used by `../sign-bootloader.sh` in the build
   pipeline. Rotation: per release train or on compromise; old db keys
   remain valid for verifying already-shipped images until their
   supported-lifetime end.

## Compromise runbook (stub)
1. Revoke the compromised key via a signed `dbx` (forbidden signatures
   database) update, pushed through the normal A/B update channel.
2. Re-sign and re-release all affected images under a new db key.
3. Post-incident: rotate the KEK if the db key compromise suggests KEK
   exposure.

Full runbook detail is a Phase 0 security-review deliverable, not yet
written — this stub defines the structure it must follow.
