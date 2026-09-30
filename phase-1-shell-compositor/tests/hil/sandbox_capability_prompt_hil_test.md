# HIL test skeleton: capability prompt on real hardware

Complements `../integration/app_runtime_end_to_end_test.md` with a
physical-device pass — specifically to catch anything the headless
integration test's software renderer might mask (e.g. real GPU-composited
prompt-dialog rendering, real input-device interaction with the prompt).

1. On Tier-1 hardware, launch a reference app requesting camera access.
2. Assert prompt renders correctly at native resolution/refresh rate.
3. Physically interact (trackpad/keyboard) to grant; assert camera
   stream starts within a to-be-set latency budget.

Not runnable — pending Tier-1 device selection and implementations.
