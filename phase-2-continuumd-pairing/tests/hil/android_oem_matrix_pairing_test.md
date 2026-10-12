# HIL test skeleton: Android OEM matrix pairing

Per OEM device (Samsung, Pixel, OnePlus, Xiaomi, Motorola):
1. Fresh-install the Companion APK; pair from the Tier-1 desktop via QR.
2. Assert `ConfirmPairingResponse{confirmed: true}` and trust anchors persisted on both sides.
3. Reboot the phone; assert reconnect uses the stored trust anchor without re-pairing.
4. Record pass/fail per device; feeds the >=98% first-attempt pairing KPI.

Not runnable: needs a Tier-1 device (open item P0-1), built Companion and
`continuumd`, and the physical OEM device matrix.
