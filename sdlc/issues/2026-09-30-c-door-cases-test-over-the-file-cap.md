# The C door's cases test is over the 500-line file cap

Status: open. Found in the ticket 0346 code review.

Kind: debt

Pay when: the next ticket that edits `libraries/c/tests/door/cases.rs`, or before 0.1.

Debt: 027

Severity: low

`libraries/c/tests/door/cases.rs` holds 768 nonblank lines, over the 500-line cap in `CLAUDE.md`. `policy.py` checks the cap only under `crates/` and `conformance/`, so no gate caught it. Keeping it lets the file grow past the size a reviewer can read at once. Split it by case family, and extend the cap check to the bindings' Rust files.
