# specification/

The contract for `thinkthen`. Code follows these documents. A behavior that is absent here is absent from the tool. Ian reads the design here, and a ticket cites the section it builds. ADR 0007 fixes the surface, and changing a Settled section takes a new ADR.

Version one is ten commands: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `annotate`, `find`, `recognize`, and `relate`. ADR 0010 removed named address profiles and their configuration command. ADR 0033 later added the smaller read-only cache configuration. `roadmap.md` holds what remains out.

The names table in [`../CONTRIBUTING.md`](../CONTRIBUTING.md#the-four-names) fixes the four names: question file, transform, how-to, and pipeline. A question file holds one question, and a question set holds several named questions. The [glossary](../CONTRIBUTING.md#calls-requests-and-decisions) defines call, request and decision.

`spec/` holds executable pages that describe the code that has landed. `specification/` is the contract the code is moving to. Slice 3 of `sdlc/planning/plan.md` closes the gap between the two. The fixtures under `fixtures/` follow the landed code until a ticket changes them together with `spec/`.

| Document | What it fixes | Status |
| --- | --- | --- |
| [channels.md](channels.md) | Arguments, the five channels, exit codes, `--quiet`, `--raw`, `--plan`, option placement | Settled |
| [threshold.md](threshold.md) | The one threshold rule, its two forms, and which verbs take which | Settled |
| [question-file.md](question-file.md) | The two homes of every setting, the question file grammar, precedence, and the question digest | Settled |
| [result.md](result.md) | Bare views, five answer kinds, result/2 IDs, provenance, transport and inactive proxy types; landed result/1 distinguished | Settled; 0.2 adoption pending |
| [files.md](files.md) | Explicit text/image readers, located carriers and spans; ordered native/CLI image inputs | Settled for 0.2 |
| [records.md](records.md) | Reading a stream of records: framing, pointers, order, failure, resume, `--cache`, `--jobs` | Settled |
| [backends.md](backends.md) | One wire shape, the key, the address, the request, retries, the `systemone` adapter | Settled, with Draft sections |
| [sdk-boundary.md](sdk-boundary.md) | One configured route per engine, retained caller controls and proxy business policy | Settled |
| [recording.md](recording.md) | `--record` and `--replay`: offline answers, usage/privacy and bounded optional timing history | Settled; timing adoption pending |
| [cache.md](cache.md) | Versioned question keys, offline validation/migration, model freshness and bounded Cache-Control storage policy | Settled for 0.2; adoption pending |
| [decide.md](decide.md) | `decide` | Settled |
| [choose.md](choose.md) | `choose` | Settled |
| [tag.md](tag.md) | `tag` | Settled |
| [score.md](score.md) | `score` | Settled |
| [filter.md](filter.md) | `filter` | Settled |
| [rank.md](rank.md) | `rank` | Settled |
| [annotate.md](annotate.md) | `annotate` and the saved question set | Settled |
| [find.md](find.md) | `find`, and the `none` option that says nothing fits | Settled |
| [recognize.md](recognize.md) | `recognize` and its beta relation output | Settled |
| [relate.md](relate.md) | `relate`, complete entity sets, relation plans, and edges | Settled |
| [transform.md](transform.md) | `transform list` and `transform show`, the read-only catalog of built-in `jq` transforms | Settled |
| [audit.md](audit.md) | `runs audit`, which grades the saved answers of all ten commands against an answer key | Settled |
| [diff.md](diff.md) | `runs diff`, which shows the saved answers that changed between two runs or two cuts | Settled |
| [check.md](check.md) | `backends check`, which sends four rich probes and minimal calls through ten functions to a named backend and reports whether it works with this tool | Settled |
| [settings.md](settings.md) | Reference: every setting, its default, and its spelling on each surface, with a link to the page that fixes it | Settled |
| [fixtures/](fixtures/) | Request and response files. Tests read them, and another implementer can test against them | Settled |

[roadmap.md](roadmap.md) lists every held verb and option with the reason it is held. The roadmap is not a contract. It carries no status word.

[ADR 0120](../sdlc/planning/adr/0120-sdk-result-and-cache-contract.md) fixes the coordinated result/2 and cache/2 target. The generated result schema and existing examples still describe landed result/1 until the owning implementation changes serializers, corpus and complete-result readers together. The target preserves bare/scalar compatibility, never claims a cache hit or answered model without observations, and reserves proxy types without activating them. It depends on 0449's one-route boundary under ADR 0119; 0443/0444/0445/0450 and the typed carrier tickets own adoption.

## Status words

Each section carries one of three words. **Settled** means code may be built against it. **Draft** means the shape is proposed and Ian has not read it. **Open** means a question in it waits on a ruling.

## The commands

| Command | One line |
| --- | --- |
| `decide QUESTION` | Answers yes, no, or not sure, and sets the exit code |
| `choose QUESTION OPTION...` | Picks one label from a fixed list |
| `tag QUESTION LABEL...` | Returns every applicable label |
| `score QUESTION LEVEL...` | Places the evidence on named levels and prints a number |
| `filter QUESTION` | Keeps the records that reach the mark |
| `rank QUESTION` | Prints records by yes probability, by a saved score value, or by turns across an ordered decide set |
| `annotate FILE` | Asks a saved question set and adds one field per question |
| `find QUESTION` | Picks the unit that best answers a question, out of a set the model sees at once |
| `recognize [KIND]...` | Finds every name in one text and gives each one a kind |
| `relate RELATION...` | Finds named relationships in one complete entity set |
| `runs audit RESULTS KEY` | Grades saved answers against an answer key and suggests a bar |
| `runs diff A [B]` | Shows the saved answers that changed between two runs or two cuts |
| `transform list`, `transform show NAME` | Lists and prints the built-in `jq` transforms |
| `backends check` | Checks that a backend you name works with this tool, and exits 0 only when nothing is critical |

The `runs` parent exposes `audit` and `diff` as settled by ticket 0440. Published top-level `audit` and `diff` remain hidden compatibility aliases with the same options, output, diagnostics and exit codes. Both spellings read saved inputs without setup or requests.

The top-level nouns `questions`, `items`, `answers`, `checks`, `datasets`, `setups`, `findings`, `people` and `search` are reserved by [ADR 0118](../sdlc/planning/adr/0118-backend-command-namespace.md). They are unimplemented and exit 2 as unrecognized commands. This adds no judging function or proxy.

[Proposed ADR0122](../sdlc/planning/adr/0122-openai-decisions-pure-adapter.md) and the [OpenAI target](backends.md#openai-decisions-target-for-02) refresh existing0441 for must-land 0.2 text support after0442–0444. Fresh ticket review is next; released System One behavior and the seventeen-ask/catalog completion scope remain.
