# specification/

The contract for `thinkthen`. Code follows these documents. A behavior that is absent here is absent from the tool. Ian reads the design here, and a ticket cites the section it builds.

| Document | What it fixes |
| --- | --- |
| [channels.md](channels.md) | Arguments, standard input, standard output, standard error, and exit codes. The rules every command obeys |
| [result.md](result.md) | The JSON result, the acceptance policy, and the four outcomes |
| [backends.md](backends.md) | Backend profiles, adapters, the `systemone` wire format, keys, timeouts, and retries |
| [recording.md](recording.md) | `--record` and `--replay`: a folder of backend exchanges that lets a command run again with no network |
| [decide.md](decide.md) | The `decide` family, verb by verb |
| [records.md](records.md) | Draft. Framing, pointers, output modes, order, limits, and exit codes 6, 7, and 8 for the verbs that read a stream |
| [fixtures/](fixtures/) | Request and response files. Tests read them, and another implementer can test against them |

## Status words

Each section carries one of three words. **Settled** means code may be built against it. **Draft** means the shape is proposed and Ian has not read it. **Open** means a question in it waits on a ruling. Changing a settled section takes an ADR.

## Families

`decide` is first. The families below follow. Each gets its own document here before any ticket.

| Family | One line |
| --- | --- |
| `decide` | Judge meaning: yes/no, pick one, rate, filter, rank, match pairs, find boundaries, run a saved question file |
| `eval`, `record`, `backend`, `config` | Measure a pass mark against labeled cases, inspect and replay recordings, check a backend, show configuration |
| `patch` | Propose and apply exact edits at places chosen by meaning. Code applies the edit |
| `reduce` | Shrink material while a declared property still holds |
| `fold` | Interpret a stream of events and update a structured state |
| `resolve` | Reconcile judged possibilities under fixed constraints. It calls no model |
| `derive` | Keep named, versioned judged fields fresh in a store |
