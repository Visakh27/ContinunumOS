# HIL test: Secure Boot tamper detection

**Level:** HIL — requires a physical Tier-1 reference device with
Continuum development keys enrolled (`../../boot/secure-boot/enroll-keys.md`).

## Test case 1: unmodified image boots
1. Flash the signed Phase 0 minimal image to the reference device.
2. Power on.
3. **Expect:** device boots to the text console checkpoint within the
   expected boot-time budget (budget TBD, tracked against the Phase 6
   performance KPIs as a baseline).

## Test case 2: tampered bootloader is refused
1. Flash the same image, but with the bootloader binary modified by one
   byte after signing (invalidating the signature).
2. Power on.
3. **Expect:** UEFI Secure Boot refuses to execute the bootloader.
   Device does **not** boot to console; falls through to firmware's own
   Secure Boot failure UI, or (once implemented) to Continuum recovery
   mode per `../../../cross-cutting/recovery-mode/`.
4. **Fail condition:** device boots to console anyway — this is a
   critical security regression, must block the release train
   (`../../ci/pipelines/release-train.yaml`).

## Test case 3: tampered root partition is refused
1. Flash a valid signed bootloader+kernel, but modify one byte of the
   root filesystem after dm-verity hash-tree generation.
2. Power on.
3. **Expect:** bootloader and kernel verify fine (Secure Boot layer
   uncompromised); dm-verity detects the root-partition hash mismatch at
   mount time and refuses to mount read-write-trusted; boot fails
   closed, drops to recovery.
4. **Fail condition:** device boots normally with the tampered root —
   critical regression.

## Status
Not runnable — requires (a) a provisioned Tier-1 reference device with
enrolled dev keys, (b) a built and signed Phase 0 image, (c) HIL lab
remote flash/power tooling. All three are outside this session's scope;
this file specifies the test precisely enough to execute once they
exist.
