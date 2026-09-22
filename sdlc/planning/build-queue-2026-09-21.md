# The build queue: everything left, in order

Written 2026-09-21 by the product side at Ian's request. It folds the quality wave 1.5 findings, the product rulings, and every surface into one queue. It replaces nothing: `build-team-response-to-handoff-2026-09-21.md` holds the ticket plan and `go-ahead-for-the-build-team-2026-09-21.md` holds the approval. This page is the order and the map. The build team creates each ticket when its turn begins, and every ticket keeps its independent review. Ticket 0055's first pass was green on every test and the reviewer still rejected it for real gaps, which is the review working.

Ian can overturn any placement here.

## Execution amendment, 2026-09-22

Ian directed the architect to resume the reviewed plan, record this queue amendment, and drive bounded engine tickets with SWE-2 implementation and independent review. This order supersedes the A5–A8 sequence below; their scope remains in the queue. ADR 0017's execution-order amendment records the same dependency change.

1. Let ticket 0065's existing owner finish and land it. Do not take over its worktree. Independent command research and non-overlapping command corrections may proceed while it finishes.
2. Recheck the forty command wording/help items against the current binary. Fix only surviving items in bounded reviewed tickets; leave unsettled wording or safety changes out of an otherwise independent correction.
3. Reconcile shared settings and result metadata, including the planned `replayed` to `cached` change, against settled contracts. Preserve existing validation, durability, and accounting guarantees.
4. Complete private engine controls: process-wide width, cancellation, deadlines, fast failure, fork recovery, and host signal ownership. Keep the command as the production caller and prove the behavior on local listeners before opening the public API.
5. Settle recognition policies and the optional instruction-packing decision. Preserve the baseline when packing lacks authorized evidence. The conflicting relation request forms remain blocked pending an explicit ruling; this amendment chooses neither form.
6. Build pure recognition/relation behavior and its engine/command callers after the relevant contracts are settled.
7. Expose the public Rust API over all ten functions after controls and result shapes are stable. Then complete C and separately review real-engine surface integration.
8. Finish installed-artifact QA and release checks. Publication, names, paid calls, and upstream reports retain their separate authorization boundaries.

The library team remains the only writer on `surfaces`. Use SWE-2 for bounded research, accepted implementation, and remediation; use separate Sol sessions for design and code review. Keep complexity floors and escalate irreducible high-risk implementation. The architect's full plan and library handoff are `architect-engine-survey-and-plan.md` and `library-team-architecture-punch-list.md`.

Observed at resumption: main `c29e445` and surfaces `7fdb1fa` match their remote branches and have passing code gates. Ticket 0065 `6618694` is not an ancestor of main. The library report is committed on `surfaces`; its passing stand-in checks do not prove real-engine integration. The separate Pages workflow on main failed at `actions/configure-pages`, outside this engine lane.

Ian subsequently authorized creating bounded tickets within this plan as each stage begins, with independent review before each implementation. Ticket 0066 implements wording-list items 2 and 5: the root help introduction and tag/annotate example layout. Independent design and code reviews accepted it, and the coordinator's full local ladder passed. Hosted gate run `35742562545` passed on `42039dd`; that revision landed on main and was pushed. The remaining wording items stay open for verification or a separate ruling; this ticket does not close the forty-item issue.

Ticket 0067 addresses wording-list items 3 and 4: the eight operation-oriented command introductions and teaching order. It keeps short-help safety disclosures, long-help result shapes, and the examples from 0066. Independent design/code reviews and the coordinator's full local ladder passed; hosted verification and landing are pending. The next command work must reconcile the outcome-word ruling and verify the remaining vocabulary/diagnostic items rather than assume all forty still fail. Shared engine settings/cache work still waits for 0065's owner to land it.

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
- Library team: on the `surfaces` branch in its own worktree, running the fix wave from `../issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`.

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

Tickets 0061 through 0063 complete recording durability, the bounded default cache, `status`, and numeric counters without changing entry contents. Ticket 0064 bounds retry waits, reports successful per-result sends, and pins prompt clean-close handling.

- Engine settings for the cache and the counters.
- The default bounded cache, the smallest config file, `--no-cache`, prune, and `thinkthen status` (`the-disk-cache-is-never-on-unless-the-user-names-a-folder`, `a-status-command-for-configuration-and-usage`).
- **A corrupt recording entry bills every retry and never repairs itself.** The worst wave 1.5 finding. A bad entry is replaced on the next good answer, never trusted and never fatal.
- A cache resumed under a different address silently bills everything again.
- A cache write that hits a size limit kills the process.
- A write failure after a good exchange throws away the paid answer.
- Lock files stay after their entries land.
- Landed through ticket 0064: `Retry-After` and exponential waits are bounded by `--timeout`; a clean close before headers fails promptly; a retried send is visible in detailed output; and the usage counter counts every attempt that left the machine.
- A run stopped by Ctrl-C prints its stopped-at line.
- Ask the product side before freezing the field name `meta.replayed`. Ian dislikes the word. The product side proposes `meta.cached`.

**A5. The probability tolerance, and shared instructions packed once per request.** Proposed ticket 8. It changes request bytes and the cost record, so it lands before `recognize` makes any cost claim.

**A6. `recognize` and `relate`.** Proposed ticket 9. The method is final. Read `experiments/225-recognize-harvest-package/README.md`, then its rules and words files, then `recognize-design.md` and `relate-design.md`. Settled: `source` and `target`, `strength` on a name, `probability` on a relation, no word list of any kind, forty recorded cases that replay with no key. The vendor's `confidence` field is never used.

**A7. The public Rust library over all ten functions.** Proposed ticket 10.

**A8. Width, cancellation, fork repair, and fast failure on a dead address.** Proposed ticket 11. The library team's lesson applies here: an interrupt or a spent deadline surfaces within one poll tick even when the backend never idles. `jobs-opens-one-connection-per-in-flight-request` and `a-process-that-forks-after-its-first-call-hangs` close here.

**A9. The C door.** Proposed ticket 12. One JSON result, one free function.

**A10. How the command gets installed.** Ruled by Ian: one Homebrew line from a public tap repository and one download script, copied from BioMCP's working pair. The release ticket works from `../issues/2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs.md`, which is the best-practices checklist for release, install, CI, and the docs site.

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
6. **One examples file per surface, keyed by function, run by that surface's tests.** The site's function pages and surface pages draw every tab from these files (`repos/mktg/products/thinkthen/site.md`), so an example nobody runs cannot reach the site. This is the library team's largest gift to the launch.
7. **Packaging rehearsals, local only.** Build the wheel, the npm package, the gem, the R package, the crate, the C archive, and the three extensions, and install each from the local file on a clean machine or container. Record the install line that worked. `yellow.local` is the build host. Nothing is uploaded.
8. **The two known blockers.** Find the road around `the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api` and `rusqlites-loadable-headers-stop-at-sqlite-3-34`, or record that none exists.

Not for the library team: anything on main, publishing, the spreadsheet bridge, and a serve mode. Those wait for Ian.

Everything ships together. Ian ruled on 2026-09-20 that the first release is 0.1 on the command, every library, and every extension at once (`../issues/2026-09-20-the-first-release-is-0-1-on-every-surface.md`). The table below is only the order the library team finishes and proves them in, and Ian can reorder it freely:

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

Experiment `229-thinkthen-spreadsheets` is closed. Its verdict replaces the product side's guesses in `../issues/2026-09-21-spreadsheet-surfaces-need-a-spike.md`:

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

Linking, coreference, and decomposition (`../issues/2026-09-21-three-next-language-problems-linking-coreference-decomposition.md`), widening `rank`, and a second vendor. All backlog.
