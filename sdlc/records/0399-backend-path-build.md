# 0399 backend paths and complete criteria

Status: in progress. Fresh High source and manual-job review accepted `5a2aa3323`; fresh Medium fixture review accepted `4349b91b3`. Both actual provider checks passed once on `4349b91b3`. Final site and strict proof remain required before landing. The builder executed no paid call, read no credential file and inspected no ambient key value.

Fresh High design acceptance is recorded in [0399-design-review.md](0399-design-review.md). The accepted design is unchanged. The implementation preserves `Backend::resolve` and adds the internal named-path resolver. Configuration validates the exact relative grammar before joining. Named and built-in paths determine the existing resolved URL identity. Perplexity uses `decisions`; OpenRouter uses `systemone` and `BothSides`. BothSides preserves authored values, fills one absent or null side with `{}`, and omits criteria when neither side remains. The request encoder remains the sole transformation owner.

The host guard, key tiers, existing wire forms, request-rate identity, decoder, cache and recording formats remain unchanged. The new pure rate regression proves an unnamed Perplexity base posting to the default path does not inherit the Decisions-path rate. Public contracts, the site table reader and backend pages describe the new rows and the custom-name migration. Site `CHECKS` now cites the actual coordinator-run receipts below.

## Executed offline proof

Nine new outside-in regressions pass in `crates/thinkthen/tests/backend/named_backends/paths_0399.rs`. They count paths and authorization selection, cover nested and 128-byte paths, refuse invalid encodings and types without values, count zero connections for all new cross-provider host pairs including normalized hosts, pin BothSides decide and tag edge tables, retain choose and score bytes, prove described-cache separation and bare-cache sharing, and exercise an OpenRouter-like schema refusal. The schema check exits 0 with eight ok rows and no warnings; the unnamed path fails noul and mixed as expected. Exact BothSides check bodies are pinned in `specification/fixtures/check/requests-both-sides.jsonl`.

Both selected backends send zero requests under a child estimated-input cap of 1. Each sends sixteen retry-status requests under its 10000 cap. These are loopback proofs. They do not establish real provider success or measured spend. Two new pure path/rate regressions pass in `core/backend/named/path_tests_0399.rs`. The complete named-backend suite passes: 23 passed and one intentionally ignored child-only test. Its parent tests execute that child. Default and text request fixture pins remain unchanged.

`CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, the candidate binary's `sdlc/scripts/settings` and `sdlc/scripts/tickets` pass. Settings initially saw the installed older binary and reported its missing window flag; placing this checkout's binary first on PATH gives 63 rows, 65 flags and zero failures. No unrelated window change was made.

On the final rebase onto main `57de974e8`, the ratchet rises from 111656 to 112305, a net 649 nonblank Rust lines. The new outside-in file has 463 nonblank lines; the pure regression file has 43. The separate saved-response regression has 64 nonblank lines. Remaining growth adds path plumbing, the built-in rows, grammar validation and BothSides encoding. I searched the existing named-backend helpers, request encoder and test mimics for duplication and reused the existing Home, counting listener, proxy and serializer. Existing fixture pins and regression tables remain. Every Rust file remains under 500 nonblank lines.

The exact keyless plans match the reviewed design: Perplexity body bytes 253, 326, 300, 725 and four-attempt estimate 5836; OpenRouter 262, 324, 298, 745 and estimate 5924. Plans are preserved in `target/0399-keyless/`. The manual shell jobs and coordinator instructions are in [sdlc/manual/0399](../manual/0399/README.md). The launcher is `sdlc/scripts/manual-0399-launch.py`. Shell syntax and launcher AST checks pass. The coordinator later executed each reviewed job once; the builder executed neither.

Required lint runs in a user scope with MemoryMax 12G and MemorySwapMax 1G, two Cargo jobs, empty RUSTC_WRAPPER, explicit installed-tool PATH and TMPDIR `/tmp/thinkthen-0399-lint` outside the checkout. A local Cargo fixture wrapper validates its exact fetch arguments, the owned TMPDIR, the planted manifest's sole file-backed git dependency and absence of dependency-of-dependency entries before permitting that local fixture fetch. Every other Cargo call forces offline mode. The wrapper is local build output under `target/0399-lint/`; it admits no remote fetch.

## Initial offline checkpoint

Required lint exits 0 on the final source. Policy, children, versions, workflow plants, exact ratchets, offline dependency checks, formatting, Clippy, warning-free docs and inventory pass; inventory checks 569 declarations and refuses four plants. The receipt is `target/0399-lint/lint.log`. Initial Clippy nesting findings were fixed by extracting schema-answer and tag-fill helpers. The children guard blanket-refuses `os.execve` outside gate tools, so the coordinator launcher lives under the existing `sdlc/scripts/` tool boundary. No guard was weakened. The coordinator names the full test, specification and strict 350-example checkpoint. Source changes stale binding receipts; refresh those samples before claiming strict proof. At that checkpoint, site dependencies were absent in this checkout, and the deliberate `checkLines('perplexity')` check failed because no actual receipt existed. The site build and check were held at that checkpoint. Final site proof still awaits the coordinator checkpoint.

The coordinator obtained fresh High acceptance for the source, exact jobs and ambient-only launcher, then reverified official pricing and ceilings before executing the authorized pair. [Manual instructions](../manual/0399/README.md) retain the combined $1 authority, the $0.18927616 conservative pair allowance, sequential single invocations and the no-automatic-rerun rule. Both actual exit-0 and critical-0 receipts are retained below. Landing still waits for final required checks.

## Fresh review follow-up: provider facts

Fresh High review of `b9e3dcc6b` found that the backend pages omitted ticket 0399's dated pricing, limits and pacing guidance. On 2026-10-04 the builder re-read [Perplexity's Decisions quickstart](https://docs.perplexity.ai/docs/decisions/quickstart) and [OpenRouter's Jev 1.13 page](https://openrouter.ai/typesafe/jev-1.13). The pages now state their published input prices and free output, Perplexity's absence of a per-request fee, both input limits, and Perplexity's question/body/organization limits. Perplexity's page suggests `"perplexity": {"requests_per_minute": 600}` and distinguishes each process's pacing from the shared organization allowance and provider token-burst limit. No runtime rate default changes. These facts are dated documentation, not actual provider-check receipts.

## Fresh review follow-up: saved response replay

The same review found that the synthetic loopback envelope did not satisfy the promised saved-response replay proof. The new `replay_0399.rs` regression retains the saved 2026-10-02 bare yes-or-no probe's returned model, provider, probability, token counts and cost in [the sanitized fixture](../../specification/fixtures/backend-0399/openrouter.response.json). The probe record retained those fields as prose, not a complete raw envelope. The fixture reconstructs them and labels its substituted ID `fixture-only-substituted-request-id-0399`. It claims no captured identifier or new provider success. [Fixture provenance](../../specification/fixtures/backend-0399/README.md) states these limits.

The regression passes through the existing legacy recording converter and then the actual `--replay` command with both applicable key variables removed. It compares the answer with the existing System One decide fixture. Both return exact `true` lines and exit 0 with empty standard error. Counted loopback requests and proxy connections remain zero for both replays. The focused regression and offline policy pass. The ratchet grows by 66 lines for this proof and its module registration; the new file has 64 nonblank lines. I searched existing recording helpers and reused `plant_recording`, Home, the listener and proxy instead of adding another recording implementation. Required lint exits 0 on fixed candidate `f9b32bfdf`. Its receipt is `target/0399-lint/review-followup-stable.log`. The run used the same 12G/1G scope, two Cargo jobs, explicit PATH, empty RUSTC_WRAPPER and verified local-only fetch wrapper. Policy, exact ratchet, workflow plants, Clippy, docs and inventory pass. An earlier lint run correctly refused after a commit moved HEAD during its exact-SHA archive fixture; the fixed run kept HEAD unchanged until completion. Actual provider proof is recorded below; final site proof remains pending.

## Checkpoint amendment: isolate the C-door type fixture

The coordinator's checkpoint passed 1339 workspace tests, 134 library-only tests and 21 external consumer tests, then stopped at `01-decide-yes-captured` because the C constructor returned null. The builder's bounded diagnosis identified the exact value-free built-in configuration refusal through the public C error accessor. An owned empty configuration home initially did not isolate the fixture: `allow-list` removes `XDG_CONFIG_HOME`, and the fixture sources it again. An owned scratch HOME with explicit build-cache homes made all 56 real C-door schema cases pass. No user configuration contents or credential files were inspected.

The owning `specification/fixtures/types/self-test` now calls the existing `scratch_dir` helper and exports its empty `XDG_CONFIG_HOME` after the allow-list, lock re-execution and existing usage isolation. The helper's existing exit cleanup removes only directories this invocation creates. Usage isolation, lock behavior and source identity guards remain unchanged. The runtime implementation and safety-reviewed manual launcher and jobs remain byte-identical to accepted candidate `5a2aa3323`.

The ignored fixture runner `target/0399-types-diagnose/isolation-self-test` creates its own HOME and plants two cases: custom `perplexity` and `openrouter` definitions that conflict with the new built-ins, then malformed configuration. It invokes the actual type fixture without reading real user configuration and checks that its own planted file remains unchanged. Before the remedy, the conflicting-name leg exits 1 at the original constructor assertion; the receipt is `target/0399-types-diagnose/regression-before.log`. After the remedy, both legs exit 0 and each passes all 56 schema cases and real C-door checks; the receipt is `target/0399-types-diagnose/regression-after.log`. The root's gate already exercises these real C engines, so no permanent additional test or routing change was added.

The exact bounded invocation is retained in ignored `target/0399-types-diagnose/run-isolation.sh`. It applies the 12G/1G user scope, empty RUSTC_WRAPPER, explicit tool PATH and an owned lock. The ignored Cargo wrapper forces offline mode and two build jobs. Shell syntax, `scratch_lint`, `usage_lint`, offline policy and diff checks pass. The Rust ratchet stays at 111279. Required scoped lint exits 0 on amendment `55ed336b2`; the receipt is `target/0399-lint/types-isolation-stable.log`. Policy, exact ratchets, workflow guards, offline dependency checks, Clippy, docs and inventory pass. An initial lint refusal identified an absolute local path in this record; exact local run paths now live only in the ignored runner. No private-name guard was weakened. The coordinator owns another full checkpoint after fresh review; no full test, spec or surfaces rung ran in this amendment.

## Actual manual provider receipts

On 2026-10-04, the coordinator executed each reviewed manual job once on `4349b91b333e45793ec7ddebe05d4a9638588995`. Whole source and manual-job High acceptance was `5a2aa3323`; the fixture amendment received fresh Medium acceptance at `4349b91b3`. Runtime and manual jobs were unchanged. The builder made no provider call and read no credential file or ambient key value.

Before the run, the coordinator checked [Perplexity's official Decisions documentation](https://docs.perplexity.ai/docs/decisions/quickstart) and [OpenRouter's official Jev endpoint data](https://openrouter.ai/api/v1/models/typesafe/jev-1.13/endpoints) at 2026-10-04 19:26:16 UTC. Input prices were $0.04 and $0.042 per million respectively, with free output. The reviewed sixteen-attempt bound remains $0.18927616 for the pair, held as a conservative $0.20 allowance against the authorized combined $1 cap.

Both jobs exited 0 and printed critical 0, warning 0. Perplexity's four successful replies named `pplx-decider-v1-27b`, with 901 input and 8 output tokens; their reported usage costs $0.00003604 at that published rate. OpenRouter was asked for `typesafe/jev-1.13`; all four successful replies named `typesafe/jev-1.13-20260917`, with 1483 input and 176 output tokens, costing $0.000062286 at the published rate. The successful-reply usage totals $0.000098326. The check does not print attempt counts. These figures establish neither an absence of retries nor an exact total bill; the conservative $0.20 allowance remains.

The live ledger moved from 467803291 to 467813291 after Perplexity, then to 467823291 after OpenRouter, charging the two durable 10000-token reservations. Nobody hand-edited the ledger. Ignored evidence lives under `target/0399-manual-receipts/`: each provider's `.log`, `.exit`, `before-*.status` and `after-*.status`, plus `pricing-preflight.json`, `keyless-preflight.json` and `successful-check-summary.json`. The following blocks copy the actual check transcripts unchanged; exit codes were independently retained as 0.

### Perplexity

```text
url https://api.perplexity.ai/v1/decisions
provider systemone
model asked pplx-decider-v1-27b
model sent pplx-decider-v1-27b
reply noul {"model":"pplx-decider-v1-27b","answers":[{"kind":"yes_no","probability":0.9615924506318492}],"usage":{"input_tokens":105,"output_tokens":1}}
reply choice {"model":"pplx-decider-v1-27b","answers":[{"kind":"choice","pick":"Tuesday","probabilities":{"Monday":0.01104863262379309,"Tuesday":0.982408089114048,"Wednesday":0.006543278262159065},"confidence":0.9736121336710718}],"usage":{"input_tokens":124,"output_tokens":1}}
reply score {"model":"pplx-decider-v1-27b","answers":[{"kind":"score","level":"excellent","probabilities":{"fair":0.06563975150997044,"good":0.007966902980280433,"excellent":0.926393345509749},"confidence":0.8607535939997787}],"usage":{"input_tokens":125,"output_tokens":1}}
reply mixed {"model":"pplx-decider-v1-27b","answers":[{"kind":"yes_no","probability":0.7292369757371859},{"kind":"choice","pick":"Tuesday","probabilities":{"Monday":0.0032943382441658806,"Tuesday":0.9967056617558342},"confidence":0.9934113235116684},{"kind":"score","level":"good","probabilities":{"poor":0.01970284758832024,"good":0.9802971524116796},"confidence":0.9605943048233596},{"kind":"tag","probabilities":{"on_time":0.17203050768979866,"damaged":0.02527703564746346}}],"usage":{"input_tokens":547,"output_tokens":5}}
ok connection
ok key
ok endpoint
ok noul
ok choice
ok score
ok mixed
ok usage
critical 0, warning 0
```

### OpenRouter

```text
url https://openrouter.ai/api/v1/systemone
provider systemone
model asked typesafe/jev-1.13
model sent typesafe/jev-1.13
reply noul {"model":"typesafe/jev-1.13-20260917","answers":[{"kind":"yes_no","probability":0.96}],"usage":{"input_tokens":312,"output_tokens":21}}
reply choice {"model":"typesafe/jev-1.13-20260917","answers":[{"kind":"choice","pick":"Tuesday","probabilities":{"Monday":0.0,"Tuesday":1.0,"Wednesday":0.0},"confidence":1.0}],"usage":{"input_tokens":353,"output_tokens":39}}
reply score {"model":"typesafe/jev-1.13-20260917","answers":[{"kind":"score","level":"excellent","probabilities":{"fair":0.03,"good":0.0,"excellent":0.97},"confidence":0.92}],"usage":{"input_tokens":338,"output_tokens":18}}
reply mixed {"model":"typesafe/jev-1.13-20260917","answers":[{"kind":"yes_no","probability":0.59},{"kind":"choice","pick":"Tuesday","probabilities":{"Monday":0.0,"Tuesday":1.0},"confidence":1.0},{"kind":"score","level":"good","probabilities":{"poor":0.0,"good":1.0},"confidence":0.99},{"kind":"tag","probabilities":{"on_time":0.49,"damaged":0.03}}],"usage":{"input_tokens":480,"output_tokens":98}}
ok connection
ok key
ok endpoint
ok noul
ok choice
ok score
ok mixed
ok usage
critical 0, warning 0
```

## Coordinator checkpoint and remaining proof

At `4349b91b3`, the coordinator's full test and specification gates passed with an owned empty HOME and explicit existing Cargo and Rustup cache homes. The test receipt includes 1339 workspace tests (25 skipped), 134 library-only tests (4 skipped), 21 external consumer tests (3 skipped) and all 56 real C-door type cases. The specification receipt ends with 24 green demos and zero red. These prove the owned isolated HOME, not the default HOME. Exact commands and logs remain in ignored `target/0399-checkpoint-final/test-isolated.log` and `spec-isolated.log`.

A second ambient run after the type-fixture fix reached the triage demo's `--plan` and produced the same value-free built-in configuration refusal. Its receipt is `target/0399-checkpoint-final/failed-ambient-after-types.log`. [Debt 038](../issues/2026-10-04-local-test-fixtures-read-ambient-configuration.md) owns remaining fixture isolation for milestone 0.2 and the next reviewed slice related to 0404. No additional fixture or runtime change belongs to this receipt update.

Provider pages and CHECKS cite these actual checks, their date and the asked model. OpenRouter's page distinguishes its served dated model. Final site proof and the actual strict 350-example checkpoint remain pending. The coordinator will name those checks after the source-preserving rebase onto 0401B. No rebase, full checkpoint, canonical documentation gate or additional paid job ran during this update.

## Receipt update verification

The focused site-data check resolves both new backend rows, pins CHECKS to their actual asked models, date, reviewed commit, exit and finding counts, and compares each copied transcript byte for byte with its retained real log and independently saved exit status. It passes. Offline policy, ticket checks and diff checks pass. No Rust source changed; the ratchet remains 111279. Site dependencies are absent in this checkout, so no site build or canonical documentation gate is claimed here. Required scoped lint exits 0 on receipt candidate `58b00e508`; the ignored receipt is `target/0399-lint/manual-receipts-stable.log`. It used the verified local-file-only fixture wrapper, offline Cargo jobs 2, empty RUSTC_WRAPPER, explicit installed-tool PATH, outside-checkout TMPDIR and 12G/1G user scope. Policy, private-name guards, exact ratchets, workflow plants, formatting, Clippy, docs and inventory pass. HEAD stayed unchanged until lint completed. This final commit records that completed receipt only.

## Final source-preserving rebase and current checkpoint

The coordinator rebased onto main `57de974e8`, preserving every product addition and deletion exactly. Fresh High review accepted source, fixture isolation, provider pages and actual manual receipts at `0fd26dae7`. The final ratchet is 112305 over main’s 111656, with unchanged net growth of 649. Historical totals above describe their named earlier candidates. No paid job was repeated after rebase.

The latest-main full test and specification gates passed with an owned isolated HOME and explicit existing build-cache homes. Tests passed 1355 workspace cases (25 skipped), 134 library-only cases (4 skipped), 21 external consumer cases (3 skipped), and the real C-door fixtures. Specification finished with 24 green demos and zero red. Receipts are `target/0399-checkpoint-final/test-after-display-rebase.log` and `spec-after-display-rebase.log`. The proof remains limited to that isolated test environment; Debt 038 retains the ambient-configuration fixture gap.

Canonical documentation checks first refused missing page goals, then the word “preview” in the two new backend pages. The coordinator added the required goal comments and described `--plan` as a run. These prose corrections are pushed at `d9bca78da` and change no request, decoder or CLI behavior. The two failed receipts remain separate from the subsequent proof in the ignored checkpoint folder.

Canonical `npm run test-docs` passed after the two page corrections: 308 actual language samples and 42 actual SQL samples replayed their saved answers. Strict verification reports 350 samples matching their proofs and zero pages to prove again. The regenerated `site/examples/bindings-proof.json` comes from those runners. The coordinator’s separate strict check and final site build also pass. Receipts are `target/0399-checkpoint-final/test-docs-after-page-fixes.log`, `strict-after-display-rebase.log` and `site-build-after-display-rebase.log`. This is Linux documentation proof; it supplies no new native Windows result. The source hash remains unchanged by final record updates.

Required latest-main scoped lint exits 0 in `target/0399-checkpoint-final/lint-after-display-rebase.log`. It preserves the verified local-file-only negative dependency fixture, offline ordinary Cargo, two build jobs, owned scratch HOME and 12G/1G scope. Policy, exact ratchets, workflow guards, private-name checks, formatting, Clippy, warning-free docs and inventory pass. All historical pending documentation statements above are resolved by this final checkpoint. Neither live job ran again.
