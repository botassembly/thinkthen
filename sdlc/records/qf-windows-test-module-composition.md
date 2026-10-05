# QuickFix: portable test deadline modules

The final hosted rehearsal 37386703492 failed Windows Clippy because `child` loaded `run.rs` and `wait.rs` again beside their parent modules. The backend check also imported `Listener` outside its Unix-only fixture. Windows qualification 37386706627 reported the root Clippy failure while its remaining steps continued.

`child` now owns the deadline helpers once. The library, backend, public controls, Polars and fork-probe test roots import those modules. `Listener` shares its fixture’s Unix guard. The change retains every test, child deadline, output collection, environment restriction and Windows fixture ACL operation. Removing repeated declarations lowers the Rust source total by 14 nonblank lines to 121858.

One fresh read-only review found an unused fork-probe `wait` import. The correction imports only `child::run`. The review accepted the remaining wiring and behavior.

Focused Linux checks passed with the existing cache locks, explicit offline environment and a systemd scope capped at 12G memory and 1G swap: formatting, policy, source ratchet, root workspace all-target Clippy, fork-probe all-target Clippy, three child-environment regressions, fourteen backend-check tests, twenty-three public-control tests and twenty-four public-environment tests. The latter two suites each retained one ignored child fixture. Native Windows qualification of the corrected source remains pending. The coordinator runs integrated landing and release checks.
