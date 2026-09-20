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

## Update, later on 2026-09-20

Ian ruled on the crates. His words: "I don't mind maintaining two crates if I have to, but I don't mind getting over the purity thing. The core could be ThinkThen, right?" Yes. The recommendation is one crate named `thinkthen` with a library target and the binary behind a default `cli` feature. `cargo install thinkthen` gives the command. `cargo add thinkthen --no-default-features` gives the library without the command-line dependencies. The purity rule becomes a lint or a check script over the core module. The fallback he also accepts is two crates, with the library named `thinkthen` and a binary crate named `thinkthen-cli` that installs a binary called `thinkthen`.

Ian also called the JavaScript snippet `if ((await tt.decide(q, text)).yes)` ugly and asked for the cleanest code possible. The opinion changed to fix it, and the change replaces position 4 above:

- **Mirror the command line: a bare answer by default, details on request.** The command line prints a bare `true`, a bare label, a bare number, or the user's own records, and `--details` swaps in the full object. The libraries do the same. `decide` under a single cut returns a plain boolean. `choose` returns the label or the language's null. `score` returns a number. The record verbs return the caller's own records. A `details` option returns the full result object.
- The homepage lines become `if tt.decide(q, command):` in Python, `if (await tt.decide(q, command)) runIt();` in JavaScript, and `if tt::decide(q, &command).await? { run_it() }` in Rust.
- **A band stays honest.** A band has three outcomes, so `decide` with a band returns a three-valued `Outcome` and never a boolean or a null that could read as false. TypeScript expresses it with overloads: a number gives `Promise<boolean>` and a `[low, high]` tuple gives `Promise<Outcome>`. Rust gets a `.band(low, high)` step that returns an enum the compiler forces the caller to match.
- **The JavaScript hazard is named in the docs.** A forgotten `await` makes `if (tt.decide(...))` always true. The TypeScript lint rules `no-floating-promises` and `no-misused-promises` catch it.
- **Types can carry the choices.** A TypeScript `as const` tuple gives a literal union and an exhaustive `switch`. Python takes a `Literal` or an `Enum`. Rust takes an enum with a derive.
- **A question can be named once and called like a predicate.** `asks_for_refund = tt.question("The customer asks for a refund.")`, then `if asks_for_refund(message):`.

This fights `sdk-design-study.md` in two places. The study returns a rich result object from every verb, and it has Python's boolean read always raise.

Ian asked whether Ruby can work. It can. The `magnus` crate with `rb-sys` wraps a Rust core into a Ruby gem, and RubyGems and Bundler build Rust extensions natively. Ruby reads best of all: `if ThinkThen.decide?("The command only reads files.", command)`. The gem name `thinkthen` returned 404 on rubygems.org on 2026-09-20. Ruby is not ruled in.

## Open

- Whether the package on each registry is claimed with a placeholder before the public push. A free name on the day of a launch is a name somebody else can take.
- Whether Ruby joins the first three languages.
