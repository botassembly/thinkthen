# Libraries are ruled in, and every public name is `thinkthen`

Status: Open

Ruled by Ian on 2026-09-20. ADR 0017 is marked proposed and says it waits for him. His words: "We're definitely doing that. I'm definitely doing Rust, Python, and JavaScript, and I want to own the NPM, crates, and PyPI records for all three." He also ruled on naming: "Everything should have the name ThinkThen. No thinkthen-core or something stupid like that. We should own that term."

## What this changes

1. **ADR 0017 moves from proposed to accepted** for its main ruling: libraries for Python, JavaScript and TypeScript, and Rust, over one Rust core.
2. **The crate naming in ADR 0017 and `sdk-design-study.md` contradicts the naming ruling.** Both name two crates on crates.io: `thinkthen-core` for the pure core and `thinkthen` for the library and binary. crates.io requires every dependency of a published crate to be published too. A separate core crate therefore forces a second public name. One crate named `thinkthen`, with a library target, the binary behind a default feature, and the pure core kept as an inner module, keeps the single name. The cost: the purity rule loses its crate boundary. Today the compiler enforces that the core touches no file, network, clock, or environment, because the core crate simply lacks those dependencies. Inside one crate a lint or a check script has to enforce it. The builders decide how. The ruling decides that the public sees one name.
3. **The names were free on 2026-09-20.** `thinkthen` returned 404 on crates.io, npm, and PyPI. Claiming them is Ian's own act. A todo is filed for him.

## Ian's bar for the interface

He wants the libraries "as clean, grammatical, and literate, or DSL-like, as possible," because a better developer experience makes better documentation. The homepage will show one tabbed sample that flips between Bash, Python, JavaScript, and Rust and does the same three jobs in each. The library has to make those twelve snippets short and honest.

A design opinion was written for him the same day. Its positions, for the builders to weigh against the study:

1. The library reads like the command line. `thinkthen decide` becomes `tt.decide` in Python and JavaScript and `tt::decide` in Rust, under a documented `tt` import.
2. The question comes first and the evidence second, positionally, in every verb. Every command-line option is a keyword argument under the same word. Rust alone gets a builder.
3. A verb is a plain function. The common case constructs no client. The key and the address come from the environment, as in the shell. A `Client` exists for a second backend and for shared concurrency.
4. Unresolved is a value at the call and an error only at the boolean read. Under a single cut, `if tt.decide(q, text):` is total and honest. Under a band, reading the result as a boolean raises, because a band has three outcomes. JavaScript gets a `.yes` that throws under a band. Rust gets no boolean and matches on `Outcome`.
5. Failure is an exception, a rejected promise, or an `Err`. It is never a value, and it never reads as a no.
6. A question file loads as a value you can call. `refund = tt.Question.load("refund.json")`, then `refund(message)`.
7. Record verbs take any iterable and return a lazy sequence in input order. `filter` yields the very objects it was handed.
8. Python is sync by default with an `aio` module beside it. JavaScript is promises. Rust is `async` with a `blocking` module beside it.
9. The names are identical across the three languages apart from each language's casing habit.

Departures from the study: bare functions and a replay scope in place of a client object for the common case, Python sync by default, the boolean rule in item 4, and the single crate.

## Open

- Whether the JavaScript `if` can read better than `if ((await tt.decide(q, text)).yes)`. It is the weakest of the twelve homepage snippets.
- Whether the package on each registry is claimed with a placeholder before the public push. A free name on the day of a launch is a name somebody else can take.
