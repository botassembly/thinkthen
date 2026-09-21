# The Rust surface

Lands in Phase B. The acceptance sample, drawn in
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
