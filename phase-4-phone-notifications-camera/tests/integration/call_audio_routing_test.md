# Integration test skeleton: call audio routing

With a mocked Bluetooth hands-free device and mocked audio service:
1. Incoming call: ring via the notification center; routing unchanged.
2. Answer: assert routing switches to the call device.
3. Change output manually mid-call; hang up; assert the manual choice is kept.
4. Repeat with HFP absent; assert "phone-only" and a Connect Bluetooth prompt.
Not runnable. The real behavior is proven by the HIL test.
