# HIL test: A/B rollback survives power loss mid-flash

**Level:** HIL. Complements the CI-level update-path testing in
`../../ci/pipelines/release-train.yaml` with a *physical* power-loss
test (CI simulation can't fully replicate real power-rail behavior).

## Procedure
1. Device on slot A (active), healthy.
2. Begin an update write to slot B (inactive).
3. Physically cut power at a randomized point during the slot-B write
   (repeat across multiple trials at different points in the write).
4. Restore power.
5. **Expect:** device boots from slot A (untouched, still the active
   slot pointer) every time — a partial/corrupt slot-B write must never
   be able to affect slot A or get promoted to active.
6. **Fail condition:** device fails to boot at all, or boots into a
   corrupted slot B — either is a critical finding, blocks release.

## Status
Not runnable — requires physical device + controllable power delivery
(e.g. a scriptable PDU) in the HIL lab. Test procedure specified now so
lab provisioning has a concrete requirement to build against.
