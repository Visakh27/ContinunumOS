# Integration test skeleton: scan to desktop

1. Start "Scan from phone" in the Files app; the emulated Companion returns a captured image.
2. Assert it travels as a drop transfer: quarantine, digest, scan, then delivery.
3. Cancel from either side mid-scan; assert nothing is delivered and no partial file remains.
Not runnable.
