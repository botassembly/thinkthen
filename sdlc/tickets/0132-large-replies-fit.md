---
flow: build
priority: 132
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/error.rs crates/thinkthen/src/cli/failure.rs crates/thinkthen/src/cli/failure/convert.rs crates/thinkthen/src/cli/check.rs crates/thinkthen/src/public/error.rs crates/thinkthen/tests/backend/resend.rs crates/thinkthen/tests/backend/relate.rs crates/thinkthen/tests/backend/recognize.rs crates/thinkthen/tests/backend/check.rs crates/thinkthen/tests/public_controls.rs specification/backends.md specification/relate.md specification/check.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0132: Keep every reply the planner can ask for

Status: built (`sdlc/records/0132-build-large-replies-fit.md`); code review pending. Design accepted by the third fresh review on 2026-09-25, after two rounds of findings. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `relate` over 180 line names, or `recognize` over 64,000 bytes of text, and gets the answer the backend sent. The tool never pays for a reply and then throws it away as a network failure. A reply that is still too long, from a broken or hostile backend, fails with its own sentence. That sentence names the limit and says the answer was not kept.

The ask is `sdlc/issues/2026-09-25-a-reply-over-1-mib-is-thrown-away-and-called-unreachable.md`. The coordinator assigned it on 2026-09-25 with this outcome:

- (a) The planner keeps every request's expected reply under a known bound. It splits requests, or it derives the read limit from the plan if that is simpler and still bounded.
- (b) A reply that still passes the limit gets its own failure and sentence, never the "could not be reached" sentence.
- (c) The ticket records which option of (a) it takes, and why.
- The limit stays, as a guard against a hostile or broken backend.

## What is wrong today

`MAX_RESPONSE_BYTES` in `crates/thinkthen/src/engine/http.rs:45` caps every reply at 1 MiB, whatever the request asked. `send` at `http.rs:264-269` reads the body under that cap. `ureq` then returns `Error::BodyExceedsLimit`, and the catch-all arm of `transport` at `http.rs:305` maps it, with every error it does not name, to `TransportKind::Other`. The command prints `the backend could not be reached; check --url and the network` at exit 4. The library prints `the backend could not be reached`.

The planner splits a request by request bytes only for a relation at the built-in address (`Backend::relation_ceiling`, 96,000 bytes, ticket 0123). Every other plan goes in one request. Measured at main `2544a8f8` against the conformance backend's generic arm:

| Case | Address | Requests | Request bytes | Reply bytes |
| --- | --- | --- | --- | --- |
| `relate r --lines`, 180 names | loopback | 1 | 2,664,188 | 1,148,848 |
| `relate r --lines`, 180 names | built-in (dry run) | 30 | at most 96,000 each | not sent |
| `recognize --kind P=a --kind O=b`, 64,000 bytes | loopback | 1 | 13,929,255 | 1,589,053 |
| the same | built-in (dry run) | 1 | 13,929,255 | not sent |

The hosted backend answers a recognition question in about 105 bytes. The committed recording `demos/44-recognize-names/recording/65d8f289….json` holds 20 questions in 2,095 reply bytes. So 64,000 bytes of text would earn about 2.7 MB from the hosted backend, in one request, and the tool would throw it away.

## Design

The read limit follows the request. An attempt reads at most

    1 MiB + 8 × (the request body's bytes)

and refuses a reply of more bytes than that. `send` already holds the request body, so the limit is one line where the body is read. It needs no plan, no question count, and no model of the reply's shape. The limit is a `u64`, and both the product and the sum saturate, so neither `8 ×` nor `N + 1` overflows on an enormous body. The doc comment above `MAX_RESPONSE_BYTES` (`http.rs:40-43`), which says a judgment answers in well under a kilobyte, is rewritten to state the rule.

A body over the limit becomes a new engine failure, `Error::ReplyTooLarge(limit)`. It is a backend failure, it is never sent again, and it carries the limit in bytes. `ureq` reports it as `BodyExceedsLimit`. `ureq`'s reader fails once it has read the whole limit, even at the end of the body, so the code passes `limit + 1` to `ureq`. A reply of exactly the limit is kept, and one byte more is refused. `ureq`'s error then carries `limit + 1`, so the read site in `send` builds `ReplyTooLarge` from the limit it worked out, never from `ureq`'s value, and the error never passes through `transport()`.

The command prints, at exit 4:

    thinkthen: the backend's reply passed this request's limit of N bytes, so the answer was not kept; the request was not sent again

The library's `thinkthen::Error::Backend` carries the same sentence without the `thinkthen:` prefix. It names no command-line option, so both surfaces share it. `N` is the exact limit in bytes, written as digits alone.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The limit follows the request. The planner does not split for reply size.** Both options of outcome (a) were weighed.
   - Splitting needs a reply-size estimate for every question type. That estimate is a model of the backend's reply shape: a choice reply adds `choice` and `confidence`, and a score reply echoes every level's text in `legend`. The model would sit in the planner and drift from the backend. Splitting also changes the plan: request counts, digests, recordings, and every dry-run page for a large input. The splitter's own contract, "the fewest contiguous chunks that pass the profile", would gain a second meaning.
   - A limit that follows the request is one expression. Every byte the reply echoes, such as a question name, an option name, or a level text, is already in the request. So the reply stays proportional to the request, and the plan stays as it is.
   - The limit stays bounded. It is 1 MiB plus eight times the bytes this process already holds for the request. A backend that never stops writing still stops the read there.
   - The bound is loose where the request is large. `recognize` turns each text byte into about 217 request bytes, because every word's question repeats its instructions: 64,000 bytes of text make a 13.9 MB request and a limit of about 112 MB. A 1 MB text would make a limit of about 1.7 GB, and the read buffer can grow to about twice what it holds. Only a hostile or broken backend can reach that. An honest one sends about 0.11 bytes of reply per request byte there. Splitting `recognize` requests would tighten both, and it is a deferred gap.
2. **The factor is 8.** The worst reply shape, worked out by hand from the hosted recordings, is a score question with ten one-character levels and one-character instructions. The hosted reply adds `score`, `confidence`, a `legend` echoing each level, and a probability per level. It comes to about 2.9 times its request bytes with two-decimal probabilities. With full-precision probabilities such as `0.9299999999999999` or `3.774758283725532e-15`, which committed recordings hold, it comes to about 5 times. A level text the backend echoes with more escaping, such as `é` sent as `\u00e9`, adds a little more. Yes/no questions come to about 1 time and choice questions to at most about 2. The measured cases above come to 0.43 and 0.11. A factor of 8 keeps a margin of about 1.6 over the worst shape. A smaller factor saves memory only against a hostile backend, and a refused honest reply costs the user a billed request.
3. **The 1 MiB floor stays.** A one-question request of a few hundred bytes still reads a reply up to 1 MiB, as today. The model name and the usage block sit outside any question and are the backend's to write.
4. **The failure is its own kind, exit 4.** The backend answered, so the connection worked. `specification/backends.md` makes every backend failure exit 4, and the new failure is one. It is not retryable, because the backend already answered and may bill again. The sentence says the answer was not kept and the request was not sent again, which is what a user needs before running it again.
5. **The limit is not an option.** No user has asked to set it. A backend profile could carry one later, as it carries `max_request_bytes`.

## Edge cases

`N` is `1,048,576 + 8 × request bytes` for that attempt's request.

| Reply | Today | After |
| --- | --- | --- |
| At most 1 MiB | Kept | Kept |
| More than 1 MiB and at most `N` | Transport sentence, exit 4, 1 send | Kept. Changed |
| Exactly `N` bytes | Not reachable over 1 MiB | Kept. New |
| `N + 1` bytes | Transport sentence, exit 4, 1 send | The new sentence naming `N`, exit 4, 1 send. Changed |
| A body that never ends | Transport sentence after 1 MiB | The new sentence after `N + 1` bytes read. Changed |
| `relate` over 180 line names at loopback | Exit 4, 1 send | 32,220 edges, exit 0, 1 send. Changed |
| `recognize` over 64,000 bytes at loopback | Exit 4, 1 send | Answers at exit 0, 1 send. Changed |
| A 400 body read for its reason | At most 4 KiB | Kept |
| A body cut short, a reset, a timeout | Their sentences | Kept |
| A replayed or cached reply | Read from disk with no limit here | Kept |
| `thinkthen check` meets a reply over its limit | Stops the check as a `connection` failure | Fails that probe with the new sentence, and the check goes on, as a refused reply does. Changed |

## Proof

Each command test counts requests at a loopback backend and pins the exact sentence and exit code.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `relate::a_relation_of_180_names_keeps_its_reply`, in `tests/backend/relate.rs` | The conformance backend's generic arm. 180 line names, `relate r --lines`. Exit 0, 32,220 edge lines on standard output, empty standard error, and a backend count of 1 | Set the factor to 0 in the code and in the resend test's formula, so the resend rows stay green: exit 4 with the new sentence. Restore the fixed 1 MiB read limit: exit 4 with the old transport sentence |
| `recognize::text_of_64000_bytes_keeps_its_reply`, in `tests/backend/recognize.rs` | The same backend, 64,000 bytes of repeated words and two kinds. Exit 0, a `{"entities":…}` object on standard output, empty standard error, and a count of 1 | The same two plants: exit 4 |
| `resend::a_reply_past_the_bound_is_sent_once`, rewritten in `tests/backend/resend.rs` | A `decide` whose listener pads a valid reply with spaces to `N + 1` bytes. The test works out `N` from the request body the listener saw. Exit 4, the new sentence naming `N`, 1 request, and no key or evidence in any output | Map `BodyExceedsLimit` to `TransportKind::Other` again: the old sentence prints. Drop the limit: the padded reply is kept at exit 0 |
| `resend::a_reply_of_exactly_the_bound_is_kept`, beside it | The same padding to exactly `N` bytes. Exit 0 with the answer, 1 request | Pass `N` to `ureq` in place of `N + 1`: the reply is refused |
| `check::a_reply_over_its_limit_fails_its_probe_and_the_check_goes_on`, in `tests/backend/check.rs` | The first probe's listener pads its reply to `N + 1` bytes, and the later probes answer. The report pins `ok connection`, the first probe's `critical` line with the new sentence, the later probes' lines, the listener count of 4, and exit 4 | Route the new failure to the `connection` gate: the check stops and the later probes are `unchecked`. Leave it out of the match: the check aborts |
| `public_controls::a_reply_over_its_limit_names_it`, in `tests/public_controls.rs` | The public `Engine` decides against a listener that pads its reply to `N + 1` bytes. The error kind is `Backend`, `retryable()` is false, the message is the new sentence, and the listener counts 1 | Map the new failure to the transport text in `public/error.rs`: the message differs. Mark it retryable in `engine/error.rs`: the listener counts 3 |

The four questions for each new or changed test:

- **What behavior does it protect?** The two relate and recognize rows protect the issue's two cases, at the sizes a user meets. The resend rows protect the guard: a limit exists, it follows the request, it is exact at the edge, and it has its own sentence.
- **What credible regression fails it?** The plants above. The fixed limit is the code today. The resend rows pin the formula, whatever factor it holds. The relate and recognize rows pin a different contract: the factor fits the replies the planner's largest real plans earn. A later change that shrinks the factor below about 0.04, and updates the resend formula with it, stays green on the resend rows and turns these two red. That is the factor-0 plant. No row pins the factor against the worst honest shape, about 5 times. That is a deferred gap. The `check` row protects the check's grading rule, which is separate code. The public row protects the library's own sentence in `public/error.rs`, which no command test reaches. The transport mapping is what `transport()` does with any error it does not name. The off-by-one is `ureq`'s reader's own behavior.
- **Why does no existing test catch it?** No test sends a relation or a recognition big enough to pass 1 MiB. The only reply-size test, `a_reply_past_the_bound_is_sent_once`, pins the "could not be reached" sentence, and this ticket rewrites it.
- **Does it need a test-only hook?** No. Each test runs the compiled command against a loopback listener.

No unit test is added. The outside-in rows pin the sentence, the code, and the count at the real boundary, so a unit test of the same sentence would test one contract at two layers.

## Specification pages

`specification/backends.md`, "The request", gains the reply limit, its formula, and the new sentence beside the transport paragraph. `specification/check.md` gains one probe row: a reply over its limit fails that probe with the sentence every command prints, and the check goes on. `cli/check.rs` grades the new failure as it grades a refused reply. Today it would abort the check. `relate.md` line 62 already lists the failures that never become partial success by class. The new failure is a backend failure, and "status" and "transport" do not name it, so the list gains "reply size". No help text names a failure, and no `spec/` page pins the transport sentence. The build checks both again with `grep`.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src`: at most 30 added, net of lines removed.
- Tests: at most 130 added across `tests/backend/relate.rs`, `recognize.rs`, `resend.rs`, `check.rs`, and `tests/public_controls.rs`.
- Pages: at most 10 lines changed across `specification/backends.md`, `relate.md`, and `check.md`.
- `sdlc/ratchet.json` rises to the measured total in the commit that adds the code. The commit says what grew. Before adding, the builder looks for duplication to delete in `http.rs` and in the transport arms of `cli/failure.rs` and `public/error.rs`.
- No dependency. No new public type, method, or variant. The public API changes only in one new message text.

## Stop rules

1. Stop if the relate or recognize command test takes more than 20 seconds in the debug test build. The debug dry runs take 0.3 s and 1.7 s today, and a real run also decodes the reply. The fallback is the smallest input whose loopback reply still passes 1 MiB, and the record names the size and the time.
2. Stop before crossing a budget, adding a dependency, or changing the planner, the splitter, or a plan's request count.
3. Stop if any plant stays green.
4. Stop if another in-flight branch changes `engine/http.rs`, `engine/error.rs`, or `cli/failure.rs` before this one lands. The coordinator orders the two.

## Scope and exclusions

Excluded: splitting `recognize` requests at the built-in address. A 64,000-byte text still goes in one request of 13.9 MB there, and whether the hosted backend takes a request that size is unmeasured. Measuring it needs a live call, and `sdlc/scripts/live` never runs for this ticket. Any change to the planner, `relation_ceiling`, backend profiles, the retry rule, or the other transport sentences. Files ticket 0131 owns (`core/measure`, `cli/audit`, `cli/measure.rs`, the audit tests) and files the test-timing Quick Fix owns (the C door test, PostgreSQL, Polars).

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change raises the ceiling, so the code review names what it checked.

## Complexity

Contract 2; state and timing 1; reach 1; proof 1; cost of error 2; total 7. Final level: 1. The risk is a factor too small for an honest reply shape, which would still bill and refuse. Decision 2 and its worked worst case guard it.

## Deferred gaps

- The hosted backend's largest reply is worked out by hand from committed recordings, not measured live. A live probe of a large recognition would confirm the factor. No test pins the factor against that worst shape. A row that sends a hosted-shape score reply at about 5 times a request over 1 MiB would need a plan of many score questions, and no command builds one today.
- `recognize` at the built-in address sends one request for the whole text, however long. That also makes this ticket's limit loose there, as decision 1 says. Splitting it belongs to its own issue.
- A backend profile that sets its own reply limit. No user has asked.
- A script that retries on exit 4 still pays again after this failure. The sentence says the request was not sent again. A distinct exit code would change the exit-code contract, and no user has asked.

## What Ian can overturn

- Decision 1: the limit follows the request. The other choice splits requests by an estimated reply size.
- Decision 2: the factor of 8.
- Decision 3: the 1 MiB floor.
- Decision 4: exit 4 for the new failure.
- Decision 5: no option to set the limit.

## Closes

- `sdlc/issues/2026-09-25-a-reply-over-1-mib-is-thrown-away-and-called-unreachable.md`. The lander moves it to `closed/` in the landing commit.

## Evidence

- Starts from: The issue above, filed by experiment 218 wave 2, rows C2 and B3, at main `20e9b8d4`: `relate` fails over 180 names and `recognize` over 64,000 bytes, after one billed request each. `sdlc/issues/closed/2026-09-19-hands-on-test-pass-one.md`, which saw a body over 1 MiB fail at exit 4 naming the 1048576 limit. `sdlc/issues/closed/2026-09-21-transport-failure-messages-paste-the-http-clients-own-words.md`, whose fix replaced the client's words with fixed guidance and lost the size cause. Ticket 0123's `relation_ceiling`, which splits only relations at the built-in address. The measurements in "What is wrong today" at main `2544a8f8`, and the committed hosted recordings under `demos/` and `probes/`.
- Keeps: The 1 MiB floor for every request. The guard against a body that never ends. No transport failure and no reply-size failure is sent again. Every other transport sentence and its exit code. The 4 KiB read of a 400 body. The planner, the splitter, every plan, request count, digest, and recording. The public API's types.
- Changes: The read limit becomes 1 MiB plus 8 bytes per request byte, exact at its edge. A reply over it is `Error::ReplyTooLarge` with its own sentence naming the limit, at exit 4 in the command and `Backend` in the library. `thinkthen check` fails the probe and goes on. `specification/backends.md`, `relate.md`, and `check.md` say so. The resend test pins the new sentence.
- Proof: The six rows under "Proof": relate at 180 names and recognize at 64,000 bytes against the conformance backend at exit 0 with one send each, the reply at `N + 1` and at exactly `N` bytes, the check probe over its limit, and the library's sentence. Each row has a planted fault that turns it red.
- Defers: A live measure of the hosted backend's largest reply. Splitting `recognize` at the built-in address. A profile-set reply limit. A distinct exit code for this failure.
