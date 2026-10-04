# 0400 provider format preparation evidence

Status: final copy prepared for fresh record review, 2026-10-04. This preparation refreshes source observations and public citations after fresh independent acceptance of the format. It changes no accepted parser or precedence choice. It makes no repository edit and runs no compilation, heavy gate, provider request, experiment or credential inspection. The earlier preparation browsed the official documents retained below. This refresh reads repository sources and existing records only; it makes no new web or live request.

## Preparation source baseline

The preparation used main `57de974e81fed7d1fb94d166d39253b4b30e70b2` and accepted 0399 source `d9bca78dac6def9bbded04089a5c4198ade5d9da`. At preparation, main had three built-ins and 0399 remained unlanded. ADR 0117 was free on both commits. The candidate added paths, Perplexity, OpenRouter and BothSides. These source references retain their original baselines.

The coordinator copied slice A after 0399 landed and pushed through `4cf69467320fe0d0508ca574da24ef161b0fe9d2`. Its owning record preserves both actual once-only checks. Latest-main full tests, specification, required lint, 308 actual language and 42 actual SQL documentation replays, strict verification with zero stale proofs and the final site build passed. This slice changes records only and repeats no paid call or full runtime gate. The new setup fields remain unsupported until B.

- Main `crates/thinkthen/src/config/backends.rs:14`: a built-in entry accepts a nonempty rate-only object. Setup prices, profile and added-entry BothSides remain planned for B.
- Main `core/backend/named.rs:158`: unnamed inheritance collects only rate-bearing built-ins. Main `:219` compares resolved posting URLs. Accepted 0399 candidate `:167` retains rate-only collection and `:229` resolves each entry's own path. Price-only/profile-only lookup still needs the accepted predicate change.
- Main `public/settings/environment.rs:43` stores captured top-level prices in `builder.prices`; `public/settings/prices.rs:15` replaces that pair explicitly. Setup precedence therefore needs the accepted provenance separation.
- Candidate `public/settings.rs:337` parses inline explicit profiles and `:406` parses file profiles at build. `core/backend_profile.rs` parses the existing closed schema and four positive limits. Preserve local-file error behavior and pre-lookup validation.
- Existing ADRs 0032, 0108, 0114 and 0115 establish profile, caller-price, key, tier, owner-only and wire-form rules. The accepted design remains unchanged.

Paths above are beneath `crates/thinkthen/src/`. Source observations are static reads of the named commits, not execution proof.

## Public source ledger, read 2026-10-04

| Source | Establishes | Does not establish |
| --- | --- | --- |
| [TypeSafe API reference](https://docs.typesafe.ai/api) | Published System One API | Revalidated historical 1200/minute rate, current direct tariff or image compatibility |
| [Liquid decision docs](https://docs.liquid.ai/lfm/models/decision-models) | Direct endpoint, d1:free and LIQUID_API_KEY | Current tariff, free-tier rate or present absence of paid direct models |
| [Ollama System One docs](https://docs.ollama.com/api/systemone) | v0.35.0+, endpoint, text-only 64 KiB limit, model-dependent context and vision-model image extension | New ThinkThen image support or deletion of its retained Text workaround |
| [Perplexity Decisions docs](https://docs.perplexity.ai/docs/decisions/quickstart) | Decisions path/model, limits, organization rate and input/output tariff | Product support beyond the exact 0399 receipts below |
| [OpenRouter Jev page](https://openrouter.ai/typesafe/jev-1.13) | 32000-token context and current $0.042/M input, free output | Current routed Liquid tariff or accepted dated request model ID |
| [llama.cpp server README](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md) | System One endpoint, ubatch prompt-fitting condition and model/projector image requirements | Repeated local performance measurements, universal slot limit or all models supporting images |
| [Cloudflare Clef](https://developers.cloudflare.com/workers-ai/models/clef/) and [Clef Flash](https://developers.cloudflare.com/workers-ai/models/clef-flash/) | Model IDs, 65536 context, 64 questions, input tariffs and documented image inputs | Known output tariff, new ThinkThen direct-route compatibility or OpenRouter reachability |

No current Lichen primary-source limit or image claim was verified. Its row preserves retained measured observations with that limitation. Public docs list capabilities; the product's accepted text surface still sends no images.

## Retained observations, no new run

The owning ThinkThen ticket 0400 at the main baseline preserves the following evidence. The 0399 design preserves hosted route/schema findings. The public ADR cites these owning records and official pages. It identifies no separate experiment repository or private path.

- Hosted observations dated 2026-10-02: direct Liquid billed zero and passed check; 52 of 1501 calls retried and separate processes defeated per-process pacing. OpenRouter Jev finished with a 120-second timeout after stalls. Routed Liquid input tariff was $0.04/M then. Old Perplexity check used the wrong path; its exact direct probe passed. Old OpenRouter check failed one-sided criteria, while an empty-side probe passed.
- Retained local findings recorded by ticket 0400: seven working models; llama.cpp merge a4cb4c61f versus earlier release b11351; 744-token Laya request required ubatch 2048; four loaded slots; Lichen serialized replies with median 0.947s and p95 1.978s. These are retained observations, not current universal defaults. This preparation did not independently recover each local execution date or rerun the measurements.
- Retained concurrency evidence in ticket 0400: 100000 rows at throttle 4 took 60.29s; at 8 took 52.00s. Both used 489 requests, 11326950 input tokens, $0.4757 and 0.890 accuracy. The 25000-row case took 16.18s and 10.58s. The run has no server clock and cannot separate network from server time. This preparation does not rerun or infer such a split.

The earlier independent reviewer accepted the format conditional on public sourcing before copying. The final copy preserves every accepted API rule. It updates only baselines, existing live evidence, readiness language and four-slice records. This revision meets that preparation condition through owning records, official links and explicit unknowns. The coordinator must review the exact draft before copying it. Proposed destination for this evidence is `sdlc/records/0400-provider-format-evidence.md`; it records design sources, not completed product or live proof.


## Accepted 0399 receipts retained without rerun

The owning [0399 build record](0399-backend-path-build.md#actual-manual-provider-receipts) records sequential checks executed once on 2026-10-04 at `4349b91b333e45793ec7ddebe05d4a9638588995`. Its reviewed runtime and manual jobs are unchanged in the accepted candidate. Both jobs exited 0 with critical 0 and warning 0.

| Backend | Requested model | Served model in all four successful replies | Successful-reply usage | Dated published prices |
| --- | --- | --- | --- | --- |
| Perplexity | `pplx-decider-v1-27b` | `pplx-decider-v1-27b` | 901 input, 8 output tokens | $0.04/M input; free output |
| OpenRouter | `typesafe/jev-1.13` | `typesafe/jev-1.13-20260917` | 1483 input, 176 output tokens | $0.042/M input; free output |

Official pricing was checked by the coordinator at 2026-10-04 19:26:16 UTC. Perplexity’s published limits include ten requests/second per organization, 128 questions, 255 choice options, ten score levels, fewer than 262144 input tokens and at most 32 MiB per body. OpenRouter’s Jev page lists 32000-token context. The candidate pages distinguish process-local configured pacing from the organization allowance. The exact dated price/limit sources and transcripts remain in the 0399 record; this record does not replace canonical proof.

Successful-reply usage totals $0.000098326 at those prices. The check does not print attempt counts, so this is neither proof of no retries nor an exact total bill. The reviewed conservative pair allowance remains $0.20 within Ian’s authorized combined $1 cap. No new call or automatic rerun is authorized. These checks establish neither other routes/models, the dated OpenRouter ID as an accepted request ID, images, Clef compatibility, nor the planned 0400 parser.

## Preparation boundaries

The accepted shape remains the read-only backend entry plus the existing closed inline profile. Built-ins accept any nonempty legal subset of rate, atomic prices and profile only after B; they never accept URL, path, key variable, model or wire override. Explicit profile outranks setup then none. Explicit prices outrank setup then top-level configuration. Named settings remain selected when explicit address/model overrides apply. Unnamed inheritance uses exact final canonical built-in posting URLs, including path; it uses no custom-entry or host-only match. No rate or tariff enters built-in code.

Slice A lands records and the stable starter table. New fields remain refused until B. Slice C raises the common default to 8 and moves explicit-8 mapping witnesses to 6 or another distinct value. Slice D publishes measured pages. No images field, automatic pacer, direct Cloudflare key or new built-in belongs to this slice.
