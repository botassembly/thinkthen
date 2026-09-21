# The Rust surface

Landed 2026-09-21. The crate `thinkthen`: an `Engine` value from the
environment, every verb in the ruled shape, blocking calls returning
`Result`, and the stand-in behind one dependency line. Run `./check.sh`
for the null suite, the conformance slice, and the wire suite when the
stub is up on 8213. Findings and quirks are in `NOTES.md`; the slide's
one-character finding is filed there for the slide owner.

The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the library:

```rust
use thinkthen::{Answer, Engine, Question};

let tt = Engine::from_env()?;
let refund = Question::decide("...").band(0.2, 0.8)?;
match tt.decide(&refund, &ticket.body)? {
    Answer::Yes => refunds.push(ticket),
    Answer::No => {}
    Answer::Unsure => review.push(ticket),
}
let complaints = tt.filter("Is this a complaint?", &reviews)?;
```

The engine itself, with no binding in between. Blocking calls, and no async
runtime comes with it. `Answer::Unsure` is a checked arm.
