# specification/

The contract for `thinkthen`. Code follows these documents. A behavior that is absent here is absent from the tool. Ian reads the design here, and a ticket cites the section it builds. ADR 0007 fixes the surface, and changing a Settled section takes a new ADR.

Version one is six commands: `decide`, `choose`, `score`, `filter`, `rank`, and `annotate`. `find` waits on a live measurement, and the configuration file waits on Ian. ADR 0010 holds both rulings, and `roadmap.md` holds what left.

`spec/` holds executable pages that describe the code that has landed. `specification/` is the contract the code is moving to. Slice 3 of `sdlc/planning/plan.md` closes the gap between the two. The fixtures under `fixtures/` follow the landed code until a ticket changes them together with `spec/`.

| Document | What it fixes | Status |
| --- | --- | --- |
| [channels.md](channels.md) | Arguments, the five channels, exit codes, `--quiet`, `--raw`, `--dry-run`, option placement | Settled |
| [threshold.md](threshold.md) | The one threshold rule, its two forms, and which verbs take which | Settled |
| [result.md](result.md) | The bare value, the `--details` object, and the three answer kinds | Settled, with one open point |
| [records.md](records.md) | Reading a stream of records: framing, pointers, order, failure, resume, `--cache` | Settled, with a Draft section |
| [backends.md](backends.md) | One wire shape, the key, the address, the request, retries, the `systemone` adapter | Settled, with Draft sections |
| [recording.md](recording.md) | `--record` and `--replay`: a folder of exchanges that runs again with no network | Settled |
| [decide.md](decide.md) | `decide` | Settled |
| [choose.md](choose.md) | `choose` | Settled |
| [score.md](score.md) | `score` | Settled |
| [filter.md](filter.md) | `filter` | Settled |
| [rank.md](rank.md) | `rank` | Settled |
| [annotate.md](annotate.md) | `annotate` and the saved question file | Settled |
| [find.md](find.md) | `find`, waiting on a live measurement against `rank --top 1` | Draft |
| [config.md](config.md) | The configuration file and the `config` command | Draft, waiting on Ian |
| [fixtures/](fixtures/) | Request and response files. Tests read them, and another implementer can test against them | Settled |

[roadmap.md](roadmap.md) lists every held verb and option with the reason it is held. The roadmap is not a contract. It carries no status word.

## Status words

Each section carries one of three words. **Settled** means code may be built against it. **Draft** means the shape is proposed and Ian has not read it. **Open** means a question in it waits on a ruling.

## The commands

| Command | One line |
| --- | --- |
| `decide QUESTION` | Answers yes, no, or unresolved, and sets the exit code |
| `choose QUESTION OPTION...` | Picks one label from a fixed list |
| `score QUESTION LEVEL...` | Places the evidence on named levels and prints a number |
| `filter QUESTION` | Keeps the records that reach the mark and prints them unchanged |
| `rank QUESTION` | Prints the records in order of the probability of yes |
| `annotate FILE` | Asks a saved file of questions and adds one field per question |
| `config path\|show\|check` | Draft. Prints the configuration path, the effective settings, or a verdict |
| `find QUESTION` | Draft. Picks the unit that best answers a question, out of a set the model sees at once |
