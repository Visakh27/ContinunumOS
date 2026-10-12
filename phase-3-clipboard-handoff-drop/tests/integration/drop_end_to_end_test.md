# Integration test skeleton: drop end to end

1. Send a file from the emulated Companion; assert it is absent from the user folder during transfer.
2. Assert it appears only after digest verification and a clean scan.
3. Send the EICAR test file; assert it never reaches the Files app (see `dropd_malicious_payload_test.md`).
4. Drag a desktop file onto the Files "Nearby Device" entry; assert the Companion receives it.
5. Kill the data path mid-transfer; assert resume per the session-loss policy.
Not runnable: needs implementations and harness.
