# Reconcile the signal dependencies after 0078

Status: Closed by Quick Fix `qf-signal-deps` on 2026-09-24 (`sdlc/records/qf-signal-deps.md`).

Found by the 0084 and 0095 code review (`sdlc/records/0084-0095-code-review.md`, item 6). Ticket 0078 landed at `1901eebe`. It made `nix` (feature `signal`) a non-optional Unix dependency of the library, and it made `signal-hook` optional and selected by `cli`. `policy.py` now fails if `nix` is missing from the default-features-off graph, or if `signal-hook` is in it.

Two records still carry the old split:

1. `sdlc/tickets/0084-freeze-the-public-rust-contract.md`, the package-proof sentence (line 364 on main at `1901eebe`). It says the proof rejects every package activated only by `cli`, "including `clap`, `csv-core`, and `nix`". It rejects `signal-hook` "only if 0078 made it CLI-only".
2. `sdlc/tickets/0086-expose-the-public-rust-api.md` line 41 on `ticket/0086-public-rust-api`. It says the library graph holds no "`clap`, `csv-core`, `nix`, or other CLI-only normal dependency".

Smallest fix: in both places, allow `nix` as a library dependency and name `signal-hook` as CLI-only. 0084's sentence gets shorter, so its character cap holds. The review says this edit does not reopen full review. It belongs to the post-0078 reconciliation, before 0085 or 0086 starts.
