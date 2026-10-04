# ADR 0117: Provider setups extend the backend entry

- Status: accepted for ticket 0400 slice A. The format received fresh independent design acceptance. This revision refreshes source baselines and the existing 0399 receipts only. The accepted API choices are unchanged. Fresh independent record review accepted all seven frozen preparation artifacts before this copy.
- Date: 2026-10-04

Historical preparation baseline: main `57de974e81fed7d1fb94d166d39253b4b30e70b2`. Current slice A source baseline is `4cf69467320fe0d0508ca574da24ef161b0fe9d2`. Ticket 0399 landed through `4cf69467320fe0d0508ca574da24ef161b0fe9d2` and supplies paths and BothSides behavior; its source candidate `d9bca78dac6def9bbded04089a5c4198ade5d9da` is retained as historical evidence. Its owning [build record](../../records/0399-backend-path-build.md#actual-manual-provider-receipts) records the two once-only actual provider checks on 2026-10-04. ADR 0117 is free on both commits. The coordinator copied this slice after 0399 landed and fresh record review accepted the preparation. This draft authorizes no paid work.

## Slice boundaries

Land four slices in the existing lane order after 0398 and 0399.

| Slice | Outcome | Claims allowed at landing |
| --- | --- | --- |
| A | Accept the format ADR and starter provider table | Experiments can report rows in a stable format. New fields are planned and the parser still refuses them. |
| B | Parse and apply provider setup fields, then update the public specification and runnable examples | Configuration support exists with reviewed offline consumer proof. |
| C | Set the engine fallback throttle to 8 and preserve meaningful mapping tests | Every surface inherits 8 when unset; explicit settings still win. |
| D | Publish measured local-server and new-model rows | Only accepted experiments support the published compatibility claims. |

This renumbers the current three slices without changing their outcome or dependency order. The existing ticket already asks for an ADR-first commit. Making it its own landing slice satisfies the handoff without presenting the remaining parser as finished. Slice A contains the ADR/table and ticket/plan records. It does not add executable examples or settings rows that the reader still rejects. The experiment day needs the reporting format, not new configuration behavior; its runner can continue using existing named backends, explicit profile files and explicit caller prices until B lands.

## Decision

Use the existing read-only `thinkthen.config/1` object's `backends` map as the setup registry. Extend an added entry with `both_sides`, a caller price pair, and an inline existing backend profile. Keep transport selection in the entry and local limits in the profile. Preserve the owner-only guard on any configuration file holding `backends`.

A built-in entry may carry any nonempty subset of `requests_per_minute`, the atomic price pair and `profile`. Its base, path, key variables, model and wire form remain fixed by the built-in. An empty built-in entry remains refused, matching current behavior. Do not introduce an address-bearing profile or another setup file schema.

| Field | Added entry | Built-in entry | Meaning |
| --- | --- | --- | --- |
| `url` | Required safe base string | Refused | Existing HTTPS/loopback HTTP address rules |
| `key_env` | Required variable-name string | Refused | Existing `[A-Z_][A-Z0-9_]*` grammar; never a key value |
| `model` | Required nonblank string | Refused | Existing model grammar |
| `path` | Optional string; default `systemone` | Refused | Ticket 0399's relative path grammar; no host change |
| `requests_per_minute` | Optional integer 1 through 60000 | Optional | Existing process-local pacer; no automatic backend rate |
| `both_sides` | Optional boolean; default false | Refused | Ticket 0399's BothSides rendering when true |
| `usd_per_million_input` | Optional decimal string, paired | Optional decimal string, paired | Caller price estimate per million reported input tokens |
| `usd_per_million_output` | Optional decimal string, paired | Optional decimal string, paired | Caller price estimate per million reported output tokens |
| `profile` | Optional closed JSON object | Optional closed JSON object | Existing `thinkthen.backend-profile/1` limits and running profile name |

Both price fields occur together or neither occurs. Preserve ADR 0108: ASCII decimal strings from 0 through 1000000 inclusive, at most six fractional digits, no sign, exponent, whitespace or explicit null. Preserve exact integer arithmetic, usage-completeness requirements, rounding and absence of money when no pair exists. Prices are caller assertions, never a billing or spend-admission guarantee.

The profile retains its existing schema, name grammar, at least one positive integer limit, unknown-field refusal, and meanings for `max_evidence_bytes`, `max_request_bytes`, `max_questions` and `max_options`. Do not convert a published token limit to an invented byte limit. A profile may lower a request ceiling but cannot raise the engine's ceiling. It supplies no transport, key, model, cache, retry, timeout, rate or throttle.

Do not add `reads_images`, `timeout`, `throttle`, or an arbitrary description-dialect selector. Image support and operational advice belong in the evidence table. ThinkThen's text surface does not gain image input through a capability row.

## Selection and precedence

1. Preserve ADR 0114's backend/address tiers. The first tier naming either decides: typed CLI settings, explicit engine settings, captured environment, then configuration. For Rust, explicit engine settings are the top tier. A same-tier backend plus address uses that backend at that address; a higher address alone takes the unnamed route and ignores lower backend names.
2. An explicitly selected named backend supplies its setup prices and profile even if a same-tier address or explicit model overrides its defaults. They are settings of the selected setup. The caller selects a different pair/profile if the alternate model has another tariff or limit.
3. On the unnamed route, match the final canonical posting URL against the built-ins' own canonical posting URLs. Only an exact match takes that built-in entry's prices and profile, as rate matching does today. Include profile-only and price-only entries in this lookup. Do not match a base prefix, host alone, custom added entry, or lower-tier backend selection. A route posting to `/systemone` never matches Perplexity's `/decisions` merely because the host/base matches.
4. Thus a run naming neither backend nor address takes the configured `typesafe` entry's prices/profile at the existing default posting URL. An unnamed run at another custom loopback URL falls back to the top-level prices and has no implicit profile. Selecting no name never changes the unnamed key variable, model precedence, path or Authored rendering.
5. Profile precedence: latest explicit engine profile source (file or inline), including CLI/binding/SQL explicit settings, then selected setup profile, then none. A question file's `profile` is its tuning label, not a runtime profile selector. Preserve its warning and digest rules.
6. Price precedence: explicit builder/native/C price pair, then selected setup pair, then configuration top-level pair, then none. Both members select together. Preserve setter replacement semantics. CLI adds no price flags or environment variables.
7. Rate precedence stays existing explicit/ambient rate over selected entry rate, then no pacer. Keep rate independent from the global process throttle. Selecting a setup does not add an automatic rate or per-backend throttle.
8. A bare `Engine::builder()` reads no file or variable and knows built-ins only. It receives no setup enrichment from an unobserved configuration file. `from_env()` captures configuration and variables; later setters/build read no environment. The existing explicit API-key override and named-key host guard remain unchanged.

Current `from_env()` stores top-level prices in the same slot that an explicit price setter later uses. The parser implementation must keep captured fallback prices separate from explicit pair provenance; testing only setter-after-from_env without a setup pair misses this defect. Current unnamed rate lookup collects only built-ins with a rate. It must collect setup-bearing built-ins independently of the presence of `requests_per_minute`.

## BothSides semantics

Reuse ticket 0399's rendering, not a second parallel wire flag. `both_sides: true` selects BothSides for added entries. False/absent selects Authored. Built-in OpenRouter retains BothSides and built-in Ollama retains its existing Text workaround; configuration cannot override either.

For a decide/tag yes-or-no wire question with one non-null side description, supply `{}` for the other side. If both sides are absent or null, omit `criteria`; if both exist, preserve both exactly. An empty object is authored content and stays an empty object. Choice and score requests do not change. The empty object adds no invented wording. No loss-of-detail warning accompanies BothSides.

Question cache keys and recordings already cover the exact URL/body/question bytes as sent. BothSides differences naturally separate keys; identical bytes may share answers. Prices/profiles introduce no extra digest component. Profiles still validate before replay/cache/key/transport, and prices do not turn replay/cache into spend.

## Planned example: not accepted until slice B

```json
{
  "schema": "thinkthen.config/1",
  "backends": {
    "perplexity": {
      "requests_per_minute": 600,
      "usd_per_million_input": "0.04",
      "usd_per_million_output": "0",
      "profile": {
        "schema": "thinkthen.backend-profile/1",
        "name": "perplexity",
        "max_questions": 128,
        "max_options": 255,
        "max_request_bytes": 33554432
      }
    },
    "local-strict": {
      "url": "http://127.0.0.1:8080/v1",
      "path": "systemone",
      "key_env": "LOCAL_SERVER_KEY",
      "model": "example-model",
      "both_sides": true,
      "profile": {
        "schema": "thinkthen.backend-profile/1",
        "name": "local-strict",
        "max_request_bytes": 96000
      }
    }
  }
}
```

The local row illustrates syntax only and makes no compatibility claim. The Perplexity profile does not raise the existing engine request ceiling; the published 262144-token ceiling is recorded separately and not represented as a byte approximation. Its published score-level ceiling remains subject to existing score constraints, not a new profile field.

## Starter evidence table

Official documents below were read on 2026-10-04. Historical measurements are preserved in [0400 preparation evidence](../../records/0400-provider-format-evidence.md) and come from the owning [ticket 0400](../../tickets/0400-provider-setups-and-concurrency.md), the [0399 design](../0399-backend-path-design.md) and existing ADRs. Retained hosted observations describe 2026-10-02; local observations retain their recorded scope and have not been repeated. New-model day must record execution date, tested model, shape, passing check, run artifacts and omissions. Rows establish a reporting format and distinguish published facts from measured compatibility. The 0399 rows retain that ticket’s actual check results; this slice adds no runtime provider support or setup-field support. The new price, profile and added-entry `both_sides` fields remain planned until slice B.

| Provider/setup | Base; relative path; key variable; model | Published limits and price from retained evidence | Images / both sides | Suggested settings and existing proof |
| --- | --- | --- | --- | --- |
| TypeSafe built-in | `https://api.typesafe.ai/v1`; `systemone`; `TYPESAFE_API_KEY`; `jev-1.13.0` | [Retained jobs finding](../../issues/closed/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md) records 1200 requests/minute. Current [API reference](https://docs.typesafe.ai/api) confirms the wire; this preparation did not re-establish that historical rate or a current tariff. | Image reach not verified in this preparation; Authored | Optional configured rate 1200 for short records. Prior product passing check exists; no new run. No automatic rate. |
| Liquid built-in | `https://api.liquid.ai/decisions/v1`; `systemone`; `LIQUIDAI_API_KEY`, then `LIQUID_API_KEY`; `d1:free` | [Liquid docs](https://docs.liquid.ai/lfm/models/decision-models) name this endpoint and d1:free. Retained 2026-10-02 calls billed 0; a direct paid model was not found then. Current direct tariff and free-tier rate remain unverified. Do not transfer a routed tariff here. | Image reach unverified; Authored | If stalls occur, jobs 4 or fewer plus resume. Retained 2026-10-02 direct check passed; 52/1501 calls retried and separate processes defeated per-process pacing. No inferred free-tier rate. |
| Ollama built-in | `http://localhost:11434/v1`; `systemone`; `OLLAMA_API_KEY` (unset is allowed on loopback); `nimble` | [Ollama docs](https://docs.ollama.com/api/systemone) require v0.35.0+, limit text-only bodies to 64 KiB and leave context model-dependent. Retained local runs measured no API charge; hardware/energy excluded. | Ollama documents separate images for vision Clef models; the retained nimble/tev runs did not verify images. Text workaround, no configurable BothSides override | Seven-model local experiment includes nimble/tev1 variants through this built-in. Preserve the description warning/debt. No measured jobs recommendation beyond server capacity. |
| Perplexity built-in in accepted 0399 candidate | `https://api.perplexity.ai/v1`; `decisions`; `PERPLEXITY_API_KEY`; `pplx-decider-v1-27b` | [Perplexity docs](https://docs.perplexity.ai/docs/decisions/quickstart): 10 requests/sec per organization; 128 questions, 255 choice options, 10 score levels, under 262144 input tokens, 32 MiB body. Input $0.04/M, output free. | Images documented in state; ThinkThen sends none. Authored accepts one side. | Optional rate 600 in a single process. Other processes/clients share the organization allowance. Retained 2026-10-02 exact direct POST passed; old product check failed wrong path. [0399 actual check](../../records/0399-backend-path-build.md#actual-manual-provider-receipts) passed once on 2026-10-04 at `4349b91b3`, exit 0, critical 0, warning 0. Asked `pplx-decider-v1-27b`; all four successful replies returned that model. No other route/model established. |
| OpenRouter built-in in accepted 0399 candidate | `https://openrouter.ai/api/v1`; `systemone`; `OPENROUTER_API_KEY`; `typesafe/jev-1.13` | [OpenRouter Jev page](https://openrouter.ai/typesafe/jev-1.13): 32000-token context, input $0.042/M, output free. Retained routed Liquid input $0.04/M/output free describes 2026-10-02, not its current tariff. | Images unverified for this tested model; BothSides | Retained 2026-10-02 calls reached both routed models. Jev required timeout 120s after stalls. Old product check failed noul/mixed; `{}` probe passed. [0399 actual check](../../records/0399-backend-path-build.md#actual-manual-provider-receipts) passed once on 2026-10-04 at `4349b91b3`, exit 0, critical 0, warning 0. Asked `typesafe/jev-1.13`; all four successful replies returned `typesafe/jev-1.13-20260917`. This does not establish the dated ID as an accepted request model or other models/routes. |
| llama.cpp added setup | Chosen loopback base ending `/v1`; `systemone`; caller-named unset local key variable; tested Laya/Kev-4B/OpenJev model names | [llama.cpp server docs](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md) publish `/v1/systemone` and prompt-fitting ubatch behavior. Retained local run loaded 4 slots, not a universal limit, and measured no API charge. | Upstream supports images for compatible model/projector pairs; images were unverified in these retained runs. Authored | Build at/after systemone merge a4cb4c61f; release b11351 predates endpoint. Laya needed ubatch 2048 for 744-token request. More than 4 requests can queue and consume timeout. |
| Lichen added setup | Chosen loopback System One base; `systemone`; caller-named unset local key variable; tested gemma-4-26B QAT serving configuration | One request at a time observed in retained local evidence; no API token charge measured. Current upstream limits were not independently verified. | Images unverified; Authored | Retained local median 0.947s, p95 1.978s; limits depend on tested server. Queueing consumes timeout. Do not infer optimal client throttle from serialization alone. |
| Clef, experimental candidate | No runnable setup established. Candidate model IDs `@cf/cloudflare/clef`, `@cf/cloudflare/clef-flash`; direct Workers AI address shape contains account ID and model path. | [Clef](https://developers.cloudflare.com/workers-ai/models/clef/) and [Clef Flash](https://developers.cloudflare.com/workers-ai/models/clef-flash/) docs confirm input $0.24/M and $0.09/M respectively, 65536-token context and at most 64 questions. Output tariff not established here. | Official docs list PNG/JPEG/WebP, up to 4 images; actual ThinkThen BothSides/reply-envelope compatibility remains unverified | No new credential authorized. Test through OpenRouter only if decision endpoint lists/reaches it, or local weights if an admitted run verifies them. Direct route remains unverified; do not construct an accepted price pair from unknown output tariff. |

The existing rate pacer spaces starts, so 600 gives approximately 100ms spacing within one process. The organization-wide rate advice does not promise safety across processes or other clients. The proxy/shared-rate work remains separate.

## Proof contract

Slice A: fresh read-only review confirms the ADR/table follows admitted evidence, distinguishes planned support, preserves lane order and assigns parser/concurrency/measured-page work to later slices. Ticket and public-name checks pass. No broad build is needed for a records-only change.

Slice B: retain ticket 0400's field/refusal matrix, caller secrecy and explicit zero-send loopback evidence. Add accepted profile-only and price-only built-in entries, all legal built-in field subsets, and old empty-entry refusal. Prove named/address override tiers, exact unnamed posting-URL matching, no custom-entry match, default TypeSafe enrichment, same-host different-path nonmatch, named setup at an overridden address/model, and explicit prices versus setup versus top level. Setter order cannot change the result. A bare builder captures nothing. Replay/cache locally refuse an over-profile request before lookup, a cache miss or key use. Profile selection changes no recording/cache identity. Prices do not change request bytes. A valid setup cannot conceal malformed or unknown configuration fields elsewhere. BothSides table preserves no-description and fully authored request bytes.

Use one Rust builder, the CLI, one binding through from_env and one SQL surface through from_env for outside-in setup proof. Strict fixtures prove existing requests unchanged. Profile/price selected behavior must be visible in real results or a counted refusal; plan-only output does not establish zero sends.

Slice C: omitted throttle reaches exactly 8 on held loopback requests for CLI, Rust/C and the selected binding/SQL families. Move explicit-8 mapping witnesses to a distinct value such as 6 and verify removal still fails. Preserve 1..32/process-wide conflict/order/cancellation contracts and the pool size. Update only defaults and historical measurement labels supported by evidence.

Slice D: every compatibility claim cites a checked run. Published commands either replay a saved response or plan offline. Unsupported direct Workers AI remains explicitly unverified. No new built-in or image carrier is introduced by documentation alone.

## Decisions Ian can overturn

The queue owner may approve these details within the authorized outcome after fresh review: entry plus inline profile; per-setup atomic price pair; BothSides boolean on added entries; exact posting-URL inheritance for unnamed built-in routes; no images field; no automatic rate/per-backend throttle; and the four-slice split. Ian can overturn them. No new cost, credential, external enrollment or irreversible commitment is introduced by slice A.
