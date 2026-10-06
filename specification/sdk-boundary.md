# SDK boundary

Status: **Settled** by [ADR 0119](../sdlc/planning/adr/0119-one-configured-route-per-engine.md), under Ian's 0.2 outcome. This boundary applies to every surface and later releases. Ian can overturn it with a reviewed amendment.

## One configured route

Each built engine resolves one final posting endpoint, one effective key (or no key on an admitted local route), and one provider API type. The provider API type is the request/response adapter and its dialect; released behavior uses System One. The proposed OpenAI Decisions text target in ADR0122 selects its distinct pure adapter explicitly through the existing named-backend door, before engine use. A named backend's posting suffix, such as `decisions`, remains part of its endpoint, not a routing instruction.

The engine fixes these values before a call. Every stage, packed request, retry and refusal-split child uses them. Concurrent calls on that engine cannot select another route. Changing the route requires an explicitly constructed engine. SQL session settings may construct a new engine for a later invocation; they never reroute a started invocation. A failed call cannot discover or switch to another provider, key or API type.

Named backends and provider setup shortcuts keep the precedence in [backends.md](backends.md#named-backends). A direct model name remains an opaque caller parameter. The caller's explicit model and question-file model retain their precedence; the SDK neither resolves a group nor chooses a target. A literal name that happens to name a provider alias is still sent literally. Nested `annotate` and rank-set members cannot select their own model. A backend hostname or unknown reply field never grants proxy authority.

## Keep and refuse

| Current or planned control | SDK ruling | Reason and owning contract |
| --- | --- | --- |
| Named backend, address, key and provider setup | Keep explicit selection before engine use | Caller chooses the route; [backends.md](backends.md) fixes resolution and secrecy |
| Direct model setter, CLI model and top-level question-file model | Keep opaque literal selection and existing precedence | Compatibility parameter, not group discovery; [settings.md](settings.md#precedence) |
| Nested annotate or rank-set model | Refuse | One call cannot carry member model routing; [annotate.md](annotate.md), [rank.md](rank.md) |
| Model groups, target maps, provider discovery and fallback providers | Refuse SDK execution | Proxy owns route selection and provider failure policy |
| A/B allocation, curation and automatic threshold tuning | Refuse SDK execution | Proxy owns business policy; a runtime answer must not install it |
| Caller cut/band, authored meanings and explicit reading overrides | Keep current function-specific rules | Deterministic caller reading; [threshold.md](threshold.md), [question-file.md](question-file.md) |
| Calibration profile identity and warnings | Keep | Report a caller's reading assumptions; never silently change the cut or batch |
| Status retries, batching, packing and one-time refusal splitting | Keep on the fixed route | Transport recovery and request organization; [backends.md](backends.md), [records.md](records.md) |
| Jobs, pacing, timeout, deadline, cancellation, request/profile/image limits and estimated-input admission | Keep | Bound caller and transport resources; do not optimize business outcomes; [settings.md](settings.md), image admission ticket 0448 |
| Caller prices, cost arithmetic, usage and attempt facts | Keep | Report resources and actual work; prices select no provider; [result.md](result.md) |
| Cache, refresh, no-cache, recording and strict replay | Keep | Storage and caller freshness controls, not route discovery; [recording.md](recording.md), tickets 0442–0444 |
| Offline rereading with an explicit cut | Keep with zero sends | Reinterpret saved observations without changing runtime defaults; [recording.md](recording.md) |
| Explicit runs audit/diff and transforms | Keep | User-invoked analysis and read-only transforms; [audit.md](audit.md), [diff.md](diff.md), [transform.md](transform.md), ticket 0440 |
| Audit tuning and caller-requested question-file writes | Keep explicit offline write only | `--write QUESTIONS` edits the named question file in place; `--write-to` creates a separate file. Neither automatically mutates configuration or running-engine policy; [audit.md](audit.md#writing-the-bar) |
| Call/request IDs, truthful retrieval/model facts and stable answer IDs | Keep under tickets 0442–0445 and 0450 | Describe sends and observations; confer no routing authority |
| Reserved proxy question identity, resolved code threshold and override fields | Reserve schema/types in 0.2; refuse activation locally before lookup or send | Ticket 0450 owns validation. Ordinary vendor bytes exclude them; direct replies cannot attest them |
| Explicit proxy mode and proxy override execution | Defer to an admitted 0.3 protocol | No current proxy protocol establishes capabilities, correspondence or policy persistence |

## Proxy separation

The proxy owns business routing, model groups, provider fallback, A/B policy, curation and automatic threshold tuning. The SDK performs typed judgment transport and explicit deterministic reading. It implements no policy optimizer, routing table or second engine for proxy behavior.

Future proxy mode must be explicit. It must refuse question-level model selection because the proxy owns that choice. It must bypass local answer reuse until an admitted policy-version contract proves reuse safe. Merely sending today's model wire to a proxy hostname remains a direct call with ordinary vendor semantics. Reserved fields and unknown response fields cannot activate overrides. The 0.2 reservation establishes no HTTP path, header, capability handshake or executable proxy mode.

Cache identity records literal requested and reported models. A mismatch cannot prove the provider's current target. Mutable aliases that echo themselves need caller refresh/no-cache or provider no-store. Offline replay may return a validated historical answer and must refuse ambiguity without discovering a current target. Tickets 0442–0444 own these compatibility changes.
