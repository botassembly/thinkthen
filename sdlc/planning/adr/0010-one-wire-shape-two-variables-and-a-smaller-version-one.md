# ADR 0010: One wire shape, two variables, and a smaller version one

- Status: Accepted by Ian on 2026-09-19. He accepted the configuration section the same day, after the rest
- Date: 2026-09-19

Ian read `open-concerns.md` and ruled. His goal stands over every line: the simplest grammar for intelligent control inside a Bash script, the most value out of the three question types, and primitives for processing data on one machine.

## Ian's rulings

1. **System One is the wire shape.** The tool speaks the hosted decider's request and response format and no other. Another model is reached by something that presents this shape at another address.
2. **Two variables name the backend.** `THINKTHEN_API_KEY` holds the key. `THINKTHEN_BASE_URL` names where the System One interface lives.
3. **`segment` leaves the plan.** It stays on the roadmap as an idea.
4. **`report` leaves the plan, and `jq` recipes come first.** The recipes are written and tried on real rows. Ian listed five outcomes: no `report` at all, the full `report`, a partial `report`, recipes alone, or a general way to carry recipes inside the tool. The last one enters only when several recipes exist that people would otherwise copy and paste. One recipe does not earn it.
5. **Every other recommendation in `open-concerns.md` is accepted.** The list:
   - The saved row is the contract. ADR 0008 items 1, 2, 3, 5, 6, and 7 are Accepted.
   - `decide` keeps the exit-code convention of `grep`. Its help shows a `case $?` block and the advice to word a question so that yes permits the action.
   - `find` stays Draft. Nothing is built until a live run compares it with `rank --top 1` on the same units.
   - `score` stays in version one and takes no threshold. The help shows `jq -e`, and `choose` with ordered labels is the way to branch on levels.
   - Structured questions are struck from version one.
   - A detailed result keeps every probability and the vendor's `confidence`. The cut stays on the winning probability until a live sweep says otherwise.
   - `--cache DIR` means `--record DIR --replay DIR`. It is specified with the records slice.
   - `choose --options POINTER` is Accepted, and `match` leaves the roadmap.
   - No new specification page is written until `choose` and `score` land. Pages are corrected and pruned as rulings require.
   - A live probe runs before more of the design is trusted.

## What the agent decided under those rulings

Ian can overturn each of these cheaply.

- The default base is `https://api.typesafe.ai/v1`, and the tool posts to `BASE/systemone`. This follows the convention of other model clients, where the base ends at the version.
- The order of sources for the address is: the hidden `--url` option, then `THINKTHEN_BASE_URL`, then the default. The key is read from `THINKTHEN_API_KEY`. Ticket 0006 still allowed a hidden `--key-env`, and ticket 0007 removes it.
- The `chat-logprobs` adapter leaves version one. Ruling 1 makes a second wire format inside the binary unnecessary. A local model is reached by a small server that presents the System One shape, and that server is a separate project.
- The recipes live in `recipes/` as `.jq` files, each with one example of its use. Demos 13 and 14 use them.
- `--jobs N` sets how many requests are in flight at once. The default is 4, and it is an advanced option. The configuration file held this setting before it left version one.

## The configuration file leaves version one

Ian accepted this section on 2026-09-19. Ticket 0007 carries it.

**The configuration file, profiles, and the `config` command leave version one.** ADR 0007 accepted them when a profile held an address, an adapter, a model, and the name of a key variable. Under rulings 1 and 2 a profile holds nothing that two variables and `--model` do not already say. The options `--profile`, `--adapter`, `--key-env`, and `--config` go with them, and `--url` stays as a hidden option for tests. Version one is then six commands: `decide`, `choose`, `score`, `filter`, `rank`, and `annotate`, with `find` waiting on its measurement.

The cost is small. A user with two endpoints sets a variable in front of the command, as every shell user already does. The file can return later without breaking anything.

Three details follow, and Ian can overturn each cheaply. The result object's `meta` loses `profile` and `adapter` and keeps `url`, `model`, `usage`, and `replayed`. The plan that `--dry-run` prints loses `profile`. The key in `THINKTHEN_API_KEY` goes to the address the user named, because naming the address is the user's own act.

## The first live answers

The key in `THINKTHEN_API_KEY` worked on 2026-09-19. Four calls went out by hand through the ticket 0005 binary. A clear refund request returned 0.99. A shipping question returned 0.01. One hedged message returned 0.24 on both of two identical requests, and a band of `0.2:0.8` left it unresolved. The model named itself `jev-1.13.0`. The four calls used 1,185 input tokens.

Two facts follow. The same request returns the same number, so a repeated trial means something only when the candidate's output changes. The result object and the exit codes of ticket 0005 hold against the real service.

## Consequences

ADR 0008 and ADR 0009 are Accepted in part, and their status lines say which parts. `segment.md` and `report.md` move to the roadmap. Demo 11 is held with `segment`. Demos 13 and 14 are rewritten over `jq` recipes. `backends.md` and `config.md` change for the two variables. The plan drops three slices and gains the live script, the live probe, and the recipes.

## Clarification, 2026-09-19: other backends will come

Ian restated that the tool will not serve one vendor alone. Competing decider services and local models are expected, and each will need an adapter. This matches ruling 1 and changes nothing already built. The seam exists: `backends.md` defines an adapter as two pure functions, one that turns a neutral plan into a request and one that turns a response into neutral answers. `systemone` is the one adapter compiled today. A recording names its adapter, and the recording key includes that name. Outside `crates/thinkthen-core/src/systemone/`, the vendor's field names appear only in tests, fixtures, and the default address.

What ADR 0010 held is the second adapter and the option that selects one. Both return with the first second backend that a user can call, and `roadmap.md` holds them until then. Three rules keep the seam clean meanwhile, and ticket 0017 turns the first into a check:

- A vendor's field name lives inside its adapter's module, its fixtures, and its tests, and nowhere else in the code. `sdlc/scripts/lint` checks it.
- Every new setting enters the neutral plan under the tool's own word. The texts for what true and false mean are `--true` and `--false` in the plan, and the `systemone` adapter alone knows they travel as `criteria`.
- The libraries that the design study proposes keep the same seam, and no library depends on one vendor's client outside its default adapter.


## Amendment, 2026-09-25, by ticket 0126: a repeated request can return another number

"The first live answers" above stays as the record of 2026-09-19. Its sentence "The same request returns the same number" is withdrawn. Two later measurements show the service moves.

- Experiment 212 sent 100 messages twice, minutes apart, all answered by `jev-1.13.0`. 63 of the 100 probabilities moved. Among those that moved, the mean move was 0.02 and the largest 0.08. The fifty borderline messages, 0.33 to 0.67, moved by up to 0.08. The fifty others, 0.02 to 0.96, moved by at most 0.03. Four answers flipped at 0.5, and each sat between 0.43 and 0.51. Its limits: one question, one set, one hundred messages, and one day. Half the sample was picked because it was borderline, so 63 of 100 overstates an ordinary file.
- Experiment 259 read the recordings of a benchmark. 681 request digests appear in more than one recording, and 178 of them hold different answers. Every one came from `jev-1.13.0`, and the largest gap was 0.09.

A repeated trial therefore shows the service's own play as well as any change in the candidate. A re-record into a used folder stops at exit 5 when the service answers a saved request differently, even with no model change. `specification/recording.md` and `specification/threshold.md` carry the measured play.
