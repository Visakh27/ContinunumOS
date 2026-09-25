// Continuum OS — continuum-ab-updated
//
// STATUS: interface skeleton only. Implementation deferred per current
// project scope (running/debugging deferred to a later stage).
//
// See ../README.md for the state machine this CLI/daemon implements.

fn main() {
    // Planned subcommands:
    //   continuum-ab-updated status    -> print current slot, update state
    //   continuum-ab-updated check     -> query update service for a new manifest
    //   continuum-ab-updated apply     -> download+verify+write to inactive slot
    //   continuum-ab-updated rollback  -> force-flip back to the previously-active slot
    //
    // Not implemented yet. See state_machine.rs (planned) for the core
    // logic this will call into, and ../../tests/unit/ for the test
    // skeletons this implementation must satisfy.
    unimplemented!(
        "continuum-ab-updated: skeleton only, see phase-0-foundation/boot/ab-update/README.md"
    );
}
