# The build queue: everything left, in order

Historical. [cleanup-2026-09-30.md](cleanup-2026-09-30.md) sets the current order of work.

Written 2026-09-21 by the product side at Ian's request. It folds the quality wave 1.5 findings, the product rulings, and every surface into one queue. It replaces nothing: `build-team-response-to-handoff-2026-09-21.md` holds the ticket plan and `go-ahead-for-the-build-team-2026-09-21.md` holds the approval. This page is the order and the map. The build team creates each ticket when its turn begins, and every ticket keeps its independent review. Ticket 0055's first pass was green on every test and the reviewer still rejected it for real gaps, which is the review working.

Ian can overturn any placement here.

## Superseded order, 2026-09-24

[One line, one owner](one-line-plan-2026-09-24.md) is the current order and ownership. The sections below are history where they differ.

## Current handoff, 2026-09-23

Tickets 0074, 0087, 0079, 0080, and 0081 are landed on remote main. The launch-first queue continues with 0088 public `relate` command; 0082–0083 command completion; 0076–0078 deadlines, shared width, fork recovery, and host signal ownership; then 0084–0086 public Rust API and packaging. Independent ticket lanes may proceed in parallel when their write sets and dependencies do not overlap. Command completion follows both functions, 0077 follows 0076, 0078 follows both, and the public API follows stable controls and result shapes.

Ticket 0081 landed at `4229bbfa` after independent Sol Medium acceptance and a green four-gate ladder. The shared relation foundation now provides concrete wildcard expansion, exact relation state, per-concrete runtime backend-profile fallback, request identity, and recognition compatibility without a public `relate` command. Ticket 0088 is next and consumes that landed foundation for the public command.

The final relation contract is in `relate-design.md`: wildcards expand to first-seen concrete kinds; different kinds choose from the smaller kind while asking the larger kind; same-kind relations use lean yes/no pairs; runtime backend-profile option and unsplittable request-byte limits fall back per concrete relation; the default cut remains 0.5; recognize and relate reuse one typed state, planner, mapping, and assembler. Ticket 0088 owns the complete `@entities` grammar, saved calibration identity, exact dry-run and Option A schemas, partial output at exit 6, help, specification, replay documentation, and secrecy. Ticket 0077 follows ADR 0017's one process cap: an implicit engine sets no cap, the first explicit width sets it, implicit engines follow it, and only a later conflicting explicit width fails locally.

Ian ended the Luna-first 0081 trial after two substantive design-remediation passes and authorized the split. The original combined ticket reached its trial limit before implementation. Sol Medium now drives redesigned tickets 0081 and 0088; separate Sol sessions independently review each design and finished diff. Existing product rulings, evidence, test gates, and landing rules remain fixed. The trial status of 0082 and 0083 is unchanged; 0082 still pauses until Ian rules on `meta.profile_warning.calibrated` versus `tuned_for`.

The 0081 record preserves the stopped trial's level, substantive Sol findings, two Luna remediation passes, Ian's exit-6 ruling, and the absence of implementation or gate results. Repository practice does not create an 0088 record before work runs. Each replacement ticket has its own realistic owners, file and line budget, focused proof, independent review, and full sequential gate. A reviewer remains read-only. Ticket 0080 began before the trial ruling and is not a trial result.

The [mainline assessment and 0074 checklist](mainline-readiness-2026-09-23.md) preserve the evidence that completed 0074. Their original snapshot and controls-first ordering are historical where the update above differs. The separate `surfaces` branch remains unmerged. Release installers and installed-artifact checks remain open. Earlier snapshots below are historical.

## Execution amendment, 2026-09-22

Ian directed the architect to resume the reviewed plan, record this queue amendment, and drive bounded engine tickets with SWE-2 implementation and independent review. This order supersedes the A5–A8 sequence below; their scope remains in the queue. ADR 0017's execution-order amendment records the same dependency change.

1. Finish and land ticket 0065. Ian subsequently confirmed its original owner had stopped and transferred completion and landing to the architect. Preserve the existing implementation and review record; reverify the integrated tree before landing. The library team's ownership of `surfaces` is unchanged.
2. Recheck the forty command wording/help items against the current binary. Fix only surviving items in bounded reviewed tickets; leave unsettled wording or safety changes out of an otherwise independent correction.
3. Reconcile shared settings and result metadata, including the planned `replayed` to `cached` change, against settled contracts. Preserve existing validation, durability, and accounting guarantees.
4. Carry structured descriptions through the question file, canonical identity, wire requests and detailed results, and publish its JSON Schema. Ticket 0069 implements the explicit product rulings, including named score maps and structured tag arrays. Preserve existing string-only behavior. Typed builders stay with the library team.
5. Complete private engine controls: process-wide width, cancellation, deadlines, fast failure, fork recovery, and host signal ownership. Keep the command as the production caller and prove the behavior on local listeners before opening the public API.
6. Settle recognition policies and the optional instruction-packing decision. Preserve the baseline when packing lacks authorized evidence. The conflicting relation request forms remain blocked pending an explicit ruling; this amendment chooses neither form.
7. Build pure recognition/relation behavior and its engine/command callers after the relevant contracts are settled.
8. Expose the public Rust API over all ten functions after controls and result shapes are stable. Then complete C and separately review real-engine surface integration.
9. Finish installed-artifact QA and release checks. Publication, names, paid calls, and upstream reports retain their separate authorization boundaries.

The library team remains the only writer on `surfaces`. Use SWE-2 for bounded research, accepted implementation, and remediation; use separate Sol sessions for design and code review. Keep complexity floors and escalate irreducible high-risk implementation. The architect's full plan and library handoff are `architect-engine-survey-and-plan.md` and `library-team-architecture-punch-list.md`.

Verification amendment: Ian disabled GitHub Actions and explicitly selected coordinator-run local full gates for future engine tickets. `../issues/closed/2026-09-22-stop-github-actions-on-push.md` records the ruling. Keep independent reviews and `sdlc/scripts/{install,lint,test,spec}`; report cancelled or unavailable hosted checks honestly. Do not re-enable workflows or change their files as part of an engine ticket.

Product update received with main `692ba59`: structured descriptions now precede recognize/relate under ruling 9 of `../issues/closed/2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields.md`. This supersedes the earlier after-0.1 placement. Typed builders remain the library team's A7 follow-through. The architect must reconcile and bound this grammar/schema work before dispatch; the separate many-state packing probe is evidence, not authorization to change batching or make new paid calls.

Observed at resumption: main `c29e445` and surfaces `7fdb1fa` match their remote branches and have passing code gates. Ticket 0065 `6618694` is not an ancestor of main. The library report is committed on `surfaces`; its passing stand-in checks do not prove real-engine integration. The separate Pages workflow on main failed at `actions/configure-pages`, outside this engine lane.

Ian subsequently authorized creating bounded tickets within this plan as each stage begins, with independent review before each implementation. Ticket 0066 implements wording-list items 2 and 5: the root help introduction and tag/annotate example layout. Independent design and code reviews accepted it, and the coordinator's full local ladder passed. Hosted gate run `35742562545` passed on `42039dd`; that revision landed on main and was pushed. The remaining wording items stay open for verification or a separate ruling; this ticket does not close the forty-item issue.

Ticket 0067 addresses wording-list items 3 and 4: the eight operation-oriented command introductions and teaching order. It keeps short-help safety disclosures, long-help result shapes, and the examples from 0066. Independent design/code reviews and the coordinator's full local ladder passed. Hosted gate run `35745327707` passed on `be078b7`, which landed on main and was pushed. The next command work must reconcile the outcome-word ruling and verify the remaining vocabulary/diagnostic items rather than assume all forty still fail. Ticket 0065 is now landed at `ba60f04`, after integrated review repaired a matching-marker directory-sync gap and both local and hosted gates passed. The 0065 record holds the proof; the prerequisite for shared settings/cache work is cleared. The planned metadata rename may proceed independently of the remaining outcome-vocabulary ruling, without changing answer semantics.

Ticket 0068 implements the planned `meta.cached` rename under ADR 0036. Provenance semantics, `--replay`, request counts, recording bytes, and historical measurements are unchanged; offline readers accept the legacy key only when `cached` is absent. Design/code review and the full sequential local ladder passed. Its hosted run was cancelled under Ian's Actions pause; the final integrated local ladder passed and `22a4193` landed on main and was pushed under the local-gate ruling. Marketing must refresh detailed-output captures; the library handoff records the canonical field for real-engine integration. Remaining wording decisions stay separately open.

Ticket 0069 implements structured question descriptions, the published question-file schema, and the later-ruled structured `state` half under ADR 0039. String-only requests and identities remain exact. Historical probe evidence remains unchanged; current demo entries are additive and mechanically derived. Independent design/code review and the final sequential local ladder passed 622 Rust tests with one intentional ignore, all replay checks, and nineteen how-tos at exact ratchet 35,909. Revision `e261e4e` landed on main and was pushed under local-gate authority. Ticket 0072 makes typed refused connections fail after one attempt while preserving premature-close retries. Independent design/code review and the final sequential local ladder passed 625 Rust tests with one intentional ignore, all replay checks, and nineteen how-tos at exact ratchet 35,974. Revision `47650c1` landed on main and was pushed under local-gate authority. Ticket 0073 adds the private cooperative-cancellation token, scheduler/request polling, and cancellation-aware retry and request-lock waits. Independent design/code review and both sequential local ladders passed 633 Rust tests with one intentional ignore, all replay checks, and nineteen how-tos at exact ratchet 36,701. It landed on main through `7ec6f82` and was pushed. CLI SIGINT binding stays separate. Width, deadlines, and fork/signal ownership remain unbuilt.

## Where things stand

- Landed: 0053 (request identity on every result) and 0054 (good answers survive one failed question, exit 6).
- Landed: 0055, the move to one package through the private engine. Its final review accepted the repaired engine ownership, conformance proof, worker lifetime, doctests, and core boundary enforcement.
- Landed: 0056. Detailed filtering now keeps only passing records, ranked details keep the same `--top` membership, and a band's exact low edge is `not sure`.
- Landed: 0057. Help now states the record-mode exit boundary and visible defaults, `--url` appears in short help under ADR 0031, zero timeouts fail before input or network access, and the specification agrees with the binary.
- Landed: 0058. Backend, transport, question-file, recording, table, and empty-input diagnostics now name the condition and a safe action. Dry runs apply the live one-document width rule.
- Landed: 0059. Explicit backend profiles enforce local byte and question limits before replay, cache, key lookup, or network access. Saved calibration names warn once at the ordered result boundary and appear in detailed metadata.
- Landed: 0060. Default streamed value rows keep each parsed record under `input` beside its answer under `value`. Streamed annotation still enriches object records and wraps non-object records. One-document and explicit views stay unchanged.
- Landed: 0061. Recording writers preflight storage, repair damaged entries atomically, handle file-size signals safely, sync complete entries, preserve valid old bytes, and remove new digest locks after a valid entry lands.
- Done: tickets 0062 and 0063 enable the bounded platform cache, its read-only configuration, `--no-cache`, explicit prune, offline `status`, and persistent numeric counters.
- Library team: on the `surfaces` branch in its own worktree, running the fix wave from `../issues/closed/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`.

## Lane A: the build team, on main

**A0. Done.** Ticket 0055 moved the scheduler, cache locks, recording, and retries into the private engine. Work that depends on those files may now start in the order below.

**A1. The command-layer fixes. Safe beside 0055 only where they avoid the files 0055 changed. Otherwise first after it.**

Tickets 0056 through 0058 completed every row below. Lane A continues with backend profiles.

| Fix | Source issue | Product ruling |
| --- | --- | --- |
| `filter --details` prints only the kept records | `2026-09-21-filter-details-must-still-filter` | Ruled by Ian. Details never change which records print. Wave 1.5 checked 30 cells and found no other command with this fault |
| The low edge of a band includes the value that reaches it | `the-low-edge-of-a-band-excludes-the-value-that-reaches-it` | The specification's "boundaries are inclusive" sentence wins. A probability at the low edge is `not sure`; one below it is `no` |
| Exit 6 is told one way | `exit-6-and-partial-failure-are-told-two-ways` | The binary and the three pages win. Exit 6 means some questions failed and the good answers printed. Fix the one page that says the code is unused, and add 6 to every exit-code table |
| Record mode and exit 0 | `record-mode-always-exits-0-and-the-help-never-says-so` | Keep it. With many records no single yes or no exists to report, so 0 means every record got an answer. The help says so in the first screen for `decide`, and the `set -e` warning names it. A gate belongs on one text or on `filter` plus a count. A how-to shows both |
| The help shows its defaults | `the-help-hides-the-defaults-a-user-assumes-wrong` | Threshold 0.5, jobs 4, and the other three appear in `--help` |
| Statuses 400 and 500 carry a phrase | `statuses-400-and-500-carry-no-phrase` | Agreed as filed |
| Messages a stranger can parse | `several-messages-cannot-be-parsed-by-a-stranger`, plus the older wording issues (`question-file-refusals-name-the-wrong-thing`, `the-empty-evidence-refusal-names-the-rule-backwards`, `the-probability-total-refusal-prints-float-noise`, `the-set-e-warning-names-only-no`, `transport-failure-messages-paste-the-http-clients-own-words`, `a-refused-request-hides-the-backends-reason`) | One wording ticket. Each message pinned by its exact sentence |
| Four specification sentences the binary refuses | `four-spec-sentences-promise-what-the-binary-refuses` | Decide each one in the ticket: the specification is the contract, so the binary moves unless the sentence was wrong |

**A2. Done.** Ticket 0059 added explicit backend profiles, local size checks, and the threshold warning. It also closes `windows-over-long-text` as a refusal with a clear message. The mismatch warning goes in `meta` too (product addition E).

**A3. Done.** Ticket 0060 returns each streamed record with the value verbs' answer and closes `two-function-flows-lose-the-record-between-stages`. `recognize` and `relate` follow the same rule when they land (addition D). A single text still prints a bare answer, pinned by tests (addition C).

**A4. The cache, and the money bugs with it.** Proposed tickets 6, 7, and 14 together, because a cache that is on by default puts these bugs in front of every user. They are one body of work in the files 0055 just moved:

Tickets 0061 through 0063 complete recording durability, the bounded default cache, `status`, and numeric counters without changing entry contents. Ticket 0064 bounds retry waits, reports successful per-result sends, and pins prompt clean-close handling. Ticket 0065 binds every new write-capable folder to one canonical backend before key lookup or a request while preserving exact read-only replay from older folders.

- Engine settings for the cache and the counters.
- The default bounded cache, the smallest config file, `--no-cache`, prune, and `thinkthen status` (`the-disk-cache-is-never-on-unless-the-user-names-a-folder`, `a-status-command-for-configuration-and-usage`).
- **A corrupt recording entry bills every retry and never repairs itself.** The worst wave 1.5 finding. A bad entry is replaced on the next good answer, never trusted and never fatal.
- Address-safe resume landed in ticket 0065. A mismatched backend now fails locally before any paid request.
- A cache write that hits a size limit kills the process.
- A write failure after a good exchange throws away the paid answer.
- Lock files stay after their entries land.
- Landed through ticket 0064: `Retry-After` and exponential waits are bounded by `--timeout`; a clean close before headers fails promptly; a retried send is visible in detailed output; and the usage counter counts every attempt that left the machine.
- A run stopped by Ctrl-C prints its stopped-at line.
- ADR 0036 and ticket 0068 settle the planned `meta.cached` spelling from the approved handoff. The name changes, the stored-answer meaning does not; `--replay` keeps its name.

**A5. The probability tolerance, and shared instructions packed once per request.** Proposed ticket 8. It changes request bytes and the cost record, so it lands before `recognize` makes any cost claim.

**A6. `recognize` and `relate`.** Proposed ticket 9. The method is final. Read `experiments/225-recognize-harvest-package/README.md`, then its rules and words files, then `recognize-design.md` and `relate-design.md`. Settled: `source` and `target`, `strength` on a name, `probability` on a relation, no word list of any kind, forty recorded cases that replay with no key. The vendor's `confidence` field is never used.

**A7. The public Rust library over all ten functions.** Proposed ticket 10.

**A8. Width, cancellation, fork repair, and fast failure on a dead address.** Proposed ticket 11. The library team's lesson applies here: an interrupt or a spent deadline surfaces within one poll tick even when the backend never idles. `jobs-opens-one-connection-per-in-flight-request` and `a-process-that-forks-after-its-first-call-hangs` close here.

**A9. The C door.** Proposed ticket 12. One JSON result, one free function.

**A10. How the command gets installed.** Ruled by Ian: one Homebrew line from a public tap repository and one download script, copied from BioMCP's working pair. The release ticket works from `../issues/2026-09-25-release-and-install-for-0-1.md`, which is the best-practices checklist for release, install, CI, and the docs site.

**A11. The promise findings.** Proposed ticket 15: public examples, pipeline outcomes, vocabulary, and `printed-speed-and-cost-numbers-name-no-measuring-record`.

**A12. The rest of the quality findings, quality waves 2 to 4, then the release pass.** Proposed ticket 16. The wave 1.5 standing rules stay: the matrix for membership, the pages for contradictions, the stranger with only the binary for everything else.

## Lane B: the library team, on the `surfaces` branch

1. Finish the fix wave: the poll loop, `source` and `target` in the question file, `reset_usage` removed, the counter, the generator's `strength` key, the stale prose, the gate rung, the public-name check.
2. Write the merge note for the build team: one conformance file, the union of cases, the engine-only defect case, the rulings issue both sides edited, the ratchet.
3. The merge ticket belongs to the build team and comes after A7, when the public Rust library exists for the surfaces to sit on.

### What the library team does while lane A runs

The surfaces are built over `contract/` and the stand-in engine, so none of this waits for main. All of it stays on the branch, publishes nothing, claims no name, and makes no paid call.

1. **The fix wave and the merge note**, items 1 and 2 above. Now.
2. **Track main's new shapes on the branch.** Read 0053 and 0054 from main and carry them into the stand-in and every surface: the `meta.requests` list, the failed-question marker in each host's own types (a typed error value in Rust, an exception class or a marker object elsewhere, a ruled form in SQL), and exit 6. Then do the same ahead of time for the ruled shapes main has not built yet: the `{"input","value"}` record row, `strength`, and `source` and `target`. Every shape adopted early is one less conflict at the merge.
3. **`recognize` and `relate` on all nine surfaces** against the forty recorded cases, including the database shape for relations that the team's own finding says can run.
4. **The Polars Series door, then pandas through the same door**, as experiment 228 ruled. Buffer-address equality is the proof of no copy.
5. **Cancel and deadline on every surface**, each with the fast-backend test that caught the poll bug: Ctrl-C in Python and R, `AbortSignal`, `pg_cancel_backend`, `statement_timeout`, DuckDB's interrupt.
6. **One examples file per surface, keyed by function, run by that surface's tests.** The site's function pages and surface pages draw every tab from these files (the marketing repository's `products/thinkthen/site.md`), so an example nobody runs cannot reach the site. This is the library team's largest gift to the launch.
7. **Packaging rehearsals, local only.** Build the wheel, the npm package, the gem, the R package, the crate, the C archive, and the three extensions, and install each from the local file on a clean machine or container. Record the install line that worked. `yellow.local` is the build host. Nothing is uploaded.
8. **The two known blockers.** Find the road around `the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api` and `rusqlites-loadable-headers-stop-at-sqlite-3-34`, or record that none exists.

Not for the library team: anything on main, publishing, the spreadsheet bridge, and a serve mode. Those wait for Ian.

Everything ships together. Ian ruled on 2026-09-20 that the first release is 0.1 on the command, every library, and every extension at once (`../issues/closed/2026-09-20-the-first-release-is-0-1-on-every-surface.md`). The table below is only the order the library team finishes and proves them in, and Ian can reorder it freely:

| # | Surface | Notes |
| --- | --- | --- |
| 1 | Python, plain lists | The first library. The site's Python page goes live with it |
| 2 | Polars, the Series door | Experiment 228's verdict: ship the Series door first. It is 5 to 8 times cheaper than the list door, copies nothing, and releases the interpreter lock. Known traps go in the manual: fork after a warm Polars pool hangs, and no cancel hook exists inside an expression |
| 3 | pandas | The same Series door. The five checks are closed. The fast path is pandas 3 only. A short page under Python |
| 4 | TypeScript | `AbortSignal` cancels a batch |
| 5 | DuckDB | Blocked today by `the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api`. `thinkthen_relations` cannot run as drawn, and the library team's finding says what can |
| 6 | Ruby | |
| 7 | R | |
| 8 | SQLite | Limited by `rusqlites-loadable-headers-stop-at-sqlite-3-34`. `thinkthen_warm` is the way around row-by-row asking |
| 9 | PostgreSQL | `jsonb` results, cancel through the server's own tools |
| 10 | Polars, the plugin expression | Ruled by Ian 2026-09-21: not in 0.1. The column form ships alone. The plugin comes later, pinned to a Polars release, with its support window documented |

Nothing depends on this order except Python before Polars and pandas. A surface with a blocker that cannot be cleared (DuckDB and SQLite each have one filed) is the only reason a surface would miss 0.1, and that would come back to Ian.

## Lane C: spreadsheets, after the libraries

Experiment `229-thinkthen-spreadsheets` is closed. Its verdict replaces the product side's guesses in `../issues/closed/2026-09-21-spreadsheet-surfaces-need-a-spike.md`:

- **Both need a bridge over HTTP.** Neither host can load the engine the way a library does.
- **Google Sheets:** the script file is written. It needs an endpoint Google's cloud can reach, which means a small deployed service wrapping the engine, some form of `thinkthen --serve`. It also needs a ruling on a token per sheet, and one live run in a real sheet.
- **Excel:** a native add-in (an XLL) built on Windows against the Excel kit, a code-signing certificate because Excel blocks unsigned add-ins, the engine shipped beside it, and one live run. Windows is the only blocker. The web add-in road is described and not written.
- Neither live run has happened.

**Ruled by Ian on 2026-09-21: paused.** No serve mode, no HTTP bridge, no Google Sheets, and no Windows work for now. Sheets is a poor fit. Excel might work later and waits until Ian wants to deal with Windows. Nothing in lanes A or B plans for either.

## Lane D: only Ian

- Claim the names: crates.io, npm and its scope, PyPI, RubyGems, the handle on X, and the trademark search. Eight open todos in the vault, dated 2026-09-20. These come before anything public names the product.
- Ruled 2026-09-21: Polars is an optional extra. `pip install thinkthen` never pulls Polars in, and `pip install thinkthen[polars]` does.
- Ruled 2026-09-21: the spend record ships in 0.1 if it is easy and waits if it is not. The product side's reading: the showing half is easy once the default cache lands in A4, and the limiting half is not needed. So A4 adds it. `thinkthen status` prints requests sent, tokens billed, and answers served from the cache, for this month and in total, from plain count rows kept beside the default cache. A row holds counts and never a text. No ceiling and no refusal in 0.1. If the build team finds the showing half is not easy, it says why and the item goes back to the backlog.
- Ruled 2026-09-21: the Polars column form ships alone in 0.1, and the command installs by Homebrew and a download script. Nothing else is waiting on Ian except the names.

## Not in this queue

Linking, coreference, and decomposition (`../issues/closed/2026-09-21-three-next-language-problems-linking-coreference-decomposition.md`), widening `rank`, and a second vendor. All backlog.
