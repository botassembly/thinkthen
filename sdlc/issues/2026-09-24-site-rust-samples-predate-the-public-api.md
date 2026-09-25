# The site's Rust samples predate the public API

Filed 2026-09-24 by Claude during ticket 0093.

The eleven `site/src/data/examples/*__rust.json` files hold the deck's drawn Rust samples, marked `"status": "drawn"`. They were written against the retired stand-in contract, and none compiles against the 0086 public API. Examples:

- `Question::choose("…", &teams)?.build()?` takes the options as a slice. The public builder takes typed options from `choices!`, one `option` call each.
- `tt.decide(ask, text)` passes a string as the question. `decide` takes a `Question` or a `BandedQuestion`.
- `found.index`, `one.index`, `.value`, `Recognize::new().kinds([…])`, and `Relate::new().either(…)` do not exist.

`libraries/rust/examples/` now holds one compiled, tested program per function with the same intent. A site ticket can draw its Rust tab from those files and mark each sample as run. This issue authorizes no work.
