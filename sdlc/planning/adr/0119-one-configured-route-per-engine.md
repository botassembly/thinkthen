# ADR 0119: Keep one configured route per engine

Status: accepted under Ian's 2026-10-06 expanded 0.2 outcome and reviewed ticket 0449.

## Decision

Make [the SDK boundary](../../../specification/sdk-boundary.md) the permanent contract for every surface. Each engine fixes one posting endpoint, effective key and provider API type before a call. Stages, retries and split children retain that route. Users may explicitly construct another engine.

Keep named backend/setup selection and literal direct model names with their existing precedence. Keep caller cuts, explicit offline rereading, resource limits, prices, calibration warnings, transport recovery and user-invoked offline analysis. Refuse SDK business routing, model-group discovery, fallback providers, A/B allocation, curation and automatic threshold tuning. The boundary table names each retained or refused control and its reason.

The proxy owns business policy. Reserve its typed question/threshold/override shape in 0.2 through 0450 and 0442. Refuse activation before lookup or send. Admit executable proxy behavior only through an explicit reviewed 0.3 protocol. Neither an endpoint hostname nor a vendor reply can activate it. Future proxy calls refuse question-level model selection and bypass local reuse until a policy-version contract proves safety.

## Evidence and consequences

The second PM message of 2026-10-06 asks for one endpoint/key/API type while the proxy owns policy. Experiment 0034's design and recorded spike supply the earlier boundary evidence; OpenRouter controls do not grant routing or image authority. No pending 0036 result is needed for this decision.

`EngineBuilder::selected` resolves the route, and `EngineBuilder::build` captures the selected key in engine settings. `engine::facade` constructs each transport client from that engine's backend and key. The existing counted loopback cases cover named-backend/model precedence, multistage recognition, status retry and refusal splitting. Ticket 0449 adds route/key/model assertions to the latter cases rather than a second proof harness. No runtime feature changes follow from this ruling.

Tickets 0442–0444 implement metadata and storage without group interpretation or discovery. Ticket 0450 owns stable answer identity and reserved-field refusal. Ian can overturn this boundary with a reviewed amendment.
