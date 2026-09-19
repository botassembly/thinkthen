# Record 0006: The two variables, the live script, and the first green demo

- Ticket: `sdlc/tickets/0006-the-two-variables-the-live-script-and-the-first-green-demo.md`
- Branch: `ticket/0006-two-variables`
- Landed: 2026-09-19

## What was built

`thinkthen` reads its key from `THINKTHEN_API_KEY` and its address from `THINKTHEN_BASE_URL`, as ADR 0010 rules. `sdlc/scripts/live` is the one door for a paid call. Demo 01 holds two live exchanges and runs green in the `spec` rung with no key and no network.

The core changed in one module.

- `backend.rs`: the built-in profile row holds a `base` of `https://api.typesafe.ai/v1` and a key variable of `THINKTHEN_API_KEY`. `BackendValues::with_base` takes the base the environment offers, so the binary reads the variable at its edge and the core still reads none. `address` is one private function over the base and the adapter: it refuses a blank base, drops trailing slashes, refuses anything that does not begin with `http://` or `https://`, refuses a base carrying user information, and joins the adapter's own name as the path. Both the profile path and the ad-hoc path call it, so `BASE/systemone` is written once. `BackendError::NotAnAddress` is the new refusal, and its message shows no address.

The binary changed at two edges.

- `edge.rs`: `Environment` reads `THINKTHEN_BASE_URL` beside the hidden test wait, and `read` now counts a variable holding only white space as unset, the way the key was already read.
- `decide.rs`: the resolver is given the base the environment offered.

`sdlc/scripts/live` is new. It takes the path of a job script, refuses at the limit `sdlc/live-tokens` holds, refuses with a blank key, builds the binary onto `PATH`, runs the job, then adds the input tokens of every recording entry the job wrote. `sdlc/live-tokens` holds the limit of 476,000,000 tokens and the spend. `demos/01-refund-gate/record.sh` names `THINKTHEN_API_KEY` and runs through the script.

`sdlc/scripts/demos` read the replay folder name out of a page's prose as well as its blocks, where a backtick closes the name, so the name now ends at a backtick too. Demo 01's prose named `--replay recording/` and sent the runner looking for a folder called ``recording/`,``.

## Red then green

Each rule was watched failing at the code before the code was right.

- `address::a_base_reaches_the_same_path_with_a_trailing_slash_and_without_one` failed with `left: "POST /v1 HTTP/1.1"` against `right: "POST /v1/systemone HTTP/1.1"`, because the tool posted to the base itself.
- `address::the_option_outranks_the_variable_and_the_variable_outranks_the_default` and `the_key_comes_from_thinkthen_api_key_and_reaches_nothing_but_the_header` failed with `left: Some(4)` against `right: Some(0)`: the variable named no address and the key was read from `TYPESAFE_API_KEY`.
- `address::a_base_that_is_not_an_http_address_is_a_usage_error_that_shows_no_address` failed with `left: Some(4)` against `right: Some(2)`, because `ftp://127.0.0.1/v1` went out as a request.
- The three cases in `live_script.rs` failed with `sh: 0: cannot open .../sdlc/scripts/live: No such file`, then with `left: Some(2)` against `right: Some(1)` while the script's blank-key pattern broke under `dash`.
- `demo_runner::the_recorded_demo_runs_and_every_demo_still_red_is_skipped` failed against the landed runner with `demos: 0 green`.

## How each acceptance bullet is proven

| Bullet | Test |
| --- | --- |
| The order of the three address sources | `address::the_option_outranks_the_variable_and_the_variable_outranks_the_default` |
| The path `BASE/systemone` with and without a trailing slash | `address::a_base_reaches_the_same_path_with_a_trailing_slash_and_without_one`, `spec/decide.md` |
| A bad base is refused with zero requests | `address::a_base_that_is_not_an_http_address_is_a_usage_error_that_shows_no_address`, `backend::tests::a_base_is_read_to_one_address_or_refused_without_showing_itself` |
| An empty variable counts as absent | `address::a_variable_that_holds_nothing_counts_as_absent`, `address::a_key_that_is_unset_or_empty_is_exit_four_and_names_the_variable_it_read` |
| The key is read from `THINKTHEN_API_KEY`, and no error, plan, recording, or `Debug` line carries it | `address::the_key_comes_from_thinkthen_api_key_and_reaches_nothing_but_the_header`, `decide_edge::no_diagnostic_ever_carries_the_key_or_the_evidence`, `record_and_replay::a_recorded_exchange_replays_with_no_listener_and_no_key` |
| The pinned recording digest from ticket 0004 still holds | `recording::tests::the_digest_of_the_fixture_request_is_the_name_the_entry_keeps` |
| `sdlc/scripts/spec` prints `demos: 1 green` and touches no network | `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sdlc/scripts/spec`, exit 0, `demos: 1 green, 13 red` |
| The live script's refusal at the limit, with no network | `live_script::a_ledger_at_its_limit_refuses_the_job_and_leaves_the_spend_alone`, `a_key_that_holds_nothing_refuses_the_job_and_never_shows_a_value`, `a_job_the_repository_does_not_hold_is_refused_before_anything_else` |
| `sdlc/planning/plan.md` gains the tokens the recording spent | The table under Live testing budget, 626 tokens on 2026-09-19 |
| The ratchet equals the measured total | `sdlc/scripts/ratchet.mjs`, run by `lint` |

## The live calls

Two calls went out through `sdlc/scripts/live`, against the built-in profile at `https://api.typesafe.ai/v1/systemone`, with the model `jev-latest`. The service answered as `jev-1.13.0`.

- The refund message returned 0.99 and used 331 input tokens.
- The product question returned 0.02 and used 295 input tokens.

626 input tokens in total. The ledger reads 1,811 of 476,000,000.

Every branch demo 01 asserts is the branch the model took. The first block routes the refund message to `refunds`, the second routes the product question to `normal`, the band `0.1:0.9` leaves the refund message resolved at `refunds`, and the saved result carries `{"value":true,"threshold":"0.1:0.9"}`. No expected output was bent and no probability was changed, because the page asserts on no probability. The page now says what the model answered and carries `Status: green`.

The two entries hold bodies alone. Searching the committed recording for the literal `apikey_` and for `authorization` returns zero matches.

## What was decided where the ticket was silent

Ian can overturn each of these cheaply.

- **`--url` names a base, like the other two sources.** The ticket lists the option as the first of three address sources and says the tool posts to `BASE/systemone`, so the option takes the same kind of value the variable takes. A caller that passed a whole URL before now passes the base. The test fixtures and `spec/decide.md` changed with it, and the digest of the pinned fixture recording is unchanged because the composed address is the same.
- **The ad-hoc rules survive untouched.** A URL still needs an adapter and a model beside it, an adapter still needs a URL, and a profile name beside a URL is still a usage error. The ticket excludes the removal of `--profile`, `--adapter`, and `--key-env`, and an ad-hoc backend is what lets a test reach a local server with no key at all. Only the composition of the address changed.
- **The refusal message shows no address, and a base carrying user information is refused.** The address is printed in a plan and kept in a recording, so a password inside one would be written to disk and committed. The message names the rule and never the value.
- **Space around a base is not part of it, and a scheme is matched without regard to case.**
- **A blank variable is an absent variable, for both variables.** `THINKTHEN_BASE_URL` holding nothing or only white space falls through to the default base. `THINKTHEN_API_KEY` holding nothing or only white space is the absent key it already was, which is exit code 4 with the message naming the variable. No new exit code was invented.
- **A blank base is refused before the scheme is read**, so `--url " "` keeps the message "a URL is text, not white space" that the blank rule gives every other text value. The core test for a blank `--url` now passes an adapter and a model beside it, because with neither of them the incomplete ad-hoc rule answers first.
- **The live script takes the path of a job**, rather than a name from a registry. One path names one job and nothing has to be kept in step.
- **The tokens are counted from the recording**, by reading `input_tokens` out of every entry the job wrote. The ticket allows the recording or a `--details` result, and demo 01 records with `--quiet`, so the recording is the only source a job has to offer.
- **The ledger is `sdlc/live-tokens`**, two rows of a name and a whole number, with the comments above them kept when the script rewrites the spend. A shell script reads it with `awk` and needs no other tool.
- **The ledger path is overridable** through `THINKTHEN_LIVE_LEDGER`, which is how the refusal test gives the script a ledger at its limit without touching the committed one.
- **The refusals exit 1 and a mistake in the call exits 2.** A limit reached and a blank key are the script refusing to spend. A missing job name, a job that is not a file, a ledger that is not a file, and a ledger holding no whole numbers are the caller getting the command wrong.
- **A job that fails still spends.** The ledger is written whatever the job's exit code was, and the run then carries that code out, because a job that stopped partway may already have paid for a call.

## The second agent's review

A second agent read the whole branch, built the binary, probed the composition rule by hand, and ran the suite offline. It checked the public surface against the crate's style, the paths a key could leak along, the composition rule, and what each new test really asserts. Four findings changed the code.

1. **Space around a base was not dropped before the address was composed.** `THINKTHEN_BASE_URL` ending in a newline, which is what `$(cat file)` and a `.env` line give, composed to an address with a newline inside it and failed later as a transport error. The base is trimmed now, by the same rule that decides absence.
2. **The scheme was matched with regard to case.** `HTTP://host/v1` is a legal address and was refused. `after_scheme` matches without regard to case.
3. **A base carrying user information wrote a password into a plan and a recording.** The error message hid the address for that very reason while the success path published it, and a recording is committed. A base whose authority holds `@` is refused now, so no user information reaches a plan, a recording, or a digest.
4. **Four assertions claimed more than they held.** The bad-base cases now assert that the message names no address, the key case counts the entry it recorded, and the unset-key case no longer repeats itself.

Two findings were left as they are. `BackendValues::new` still takes the five flag values and `with_base` still adds the environment's, which keeps the two sources apart at the type level. The rule that an empty variable counts as absent still lives in the binary, because the core reads no environment.

The review also named the failure of a live job as a lost spend. That was fixed before the review landed: the ledger is written whatever the job's exit code was.

## The ladder

| Rung | Script | Exit |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | 0 |
| 1 | `sdlc/scripts/lint` | 0 |
| 2 | `sdlc/scripts/test` | 0 |
| 3 | `sdlc/scripts/spec` | 0 |

One hundred and four tests, one documentation test, twenty-one spec examples, and the demo runner over one green demo and thirteen red ones pass. Rung 3 was run with `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL`.

## The ratchet

The ceiling was 4094 and is 4520. `tests/address.rs` holds the six cases the two variables need and `tests/live_script.rs` holds the three refusals, which is the whole growth; the source itself moved by a few lines, because the address rule replaced a stored URL with one function both paths call. Duplication was looked for in `backend.rs`, `edge.rs`, and the three listener test files before a line was added. The one shared shape across the test files is the process runner, and each file writes the environment its own cases need, so folding them would cost more in arguments than it saves in lines.

## Dependencies

None added, none removed.

## Correction

Written 2026-09-19 under ticket 0007, from `sdlc/issues/2026-09-19-review-leftovers-from-ticket-0006.md` item 4. This record first said the ceiling was 4531 and that fourteen demos were red. `sdlc/ratchet.json` held 4520 and `sdlc/scripts/spec` printed `demos: 1 green, 13 red`. The three lines above now carry the measured numbers.
