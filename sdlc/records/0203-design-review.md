# Accept the calibration profile design

Status: design accepted, 2026-09-27. Owner: Codex. No runtime fix is claimed.

A fresh read-only Codex reviewer accepted ticket 0203 at `fea8c48a`. The final review found that the ticket covers the observed saved-profile loss paths and pins the R `tt_details` proof to the existing public entry. It also found the boundaries with accepted ADRs 0032 and 0048 and tickets 0148 and 0149 coherent. The queue owner accepts the design within Ian's direction to settle the register. Ian can overturn the ticket's surface-routing and refusal choices; changing the accepted identity or batch rules requires an ADR.

The review cycle corrected the public question-set builder path that could discard a member profile, Python's column shortcut, R's per-row fallback and text-only ordering routes, and Ruby's `rank` and `find` text extraction. The final correction makes R's existing `tt_details(tt_question(file = PATH), evidence)` return the pinned saved-profile digest and mismatch warning. The build must prove these routes through their real public entries, with counted zero-send refusals where specified. Ticket 0148 supplies runtime profile controls for Rust and language wrappers, and 0149 supplies SQL controls. Both remain prerequisites for the full build.

This acceptance covers the ticket design only. No runtime change, cross-surface proof, or gate result is claimed here. The issue remains open until implementation and its proof land.
