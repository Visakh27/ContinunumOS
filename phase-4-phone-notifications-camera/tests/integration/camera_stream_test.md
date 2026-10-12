# Integration test skeleton: camera stream through the portal

1. A sandboxed test app requests the camera; assert the portal lists "Phone (name)" and prompts.
2. Grant and start; assert frames reach the app and the shell indicator appears.
3. Simulate camera-busy on the phone; assert placeholder frames and the paused state.
4. Drop the session beyond the grace window; assert the stream ends and does not restart on reconnect.
5. App without a camera grant: assert it cannot see the node.
Not runnable.
