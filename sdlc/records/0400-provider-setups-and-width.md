# 0400: Provider setups and request width

A/B/C landed: sourced starter provider guidance, read-only backend rate/prices/inline-profile selection and a shared omitted-width default of eight (`8ef1415b5`). CLI/Rust/C/Python/R/DuckDB retain explicit width overrides and preview/runtime agreement. Parser/preference/trust/secrecy tests and affected public API cases passed; default-eight landing full tests passed.

Built-ins accept nonempty legal subsets of rate, atomic prices and closed inline profile; they accept no address/path/key/model/wire override. Explicit profile outranks setup then none; explicit prices outrank setup then top-level configuration. Selected named setups survive address/model overrides. Unnamed setup inheritance uses the exact final built-in posting URL, including path; no custom-entry or host-only match, automatic pacer, image field or tariff enters built-in code.

Left: measured provider-page publication under accepted evidence (slice D); no new call is authorized. The 0399 checks are summarized in [its note](0399-backend-paths.md): they establish only their dated tested routes/models, not general image/Cloudflare compatibility or retry-free billing. The source ledger and observations below were read/retained on 2026-10-04 and are not current remeasurements.

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
