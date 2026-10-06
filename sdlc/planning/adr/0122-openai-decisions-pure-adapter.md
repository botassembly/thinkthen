# ADR 0122: Select OpenAI Decisions through a pure adapter

- Status: Accepted. Ian's 2026-10-06 ruling requires OpenAI text and permits the second adapter. Detailed design below is proposed; fresh 0441 review is next before implementation.
- Date: 2026-10-06
- Owner: [0441](../../tickets/0441-openai-decisions-backend-preview.md).
- Risk: High: credentials, spend and cross-adapter storage isolation.

## Decision

Amend ADR 0010 ruling1 and its second-adapter deferral: 0.2 includes OpenAI Decisions text as the first second wire adapter. Amend ADR 0017's layer allocation to allow pure typed encode/decode functions for both formats in core; selection, environment/key capture, endpoint resolution, transport and files remain at the edge. All supported surfaces still bind one Rust engine. No vendor SDK, second engine, business routing, discovery or fallback enters.

The new built-in `openai` fixes base `https://api.openai.com/v1`, suffix `decisions`, API type `openai-decisions`, model default `gpt-6-luna` and named key variable `OPENAI_API_KEY`. Existing typed/backend/address/model precedence and explicit-key override remain. Naming another URL keeps the selected API type; a `decisions` suffix or OpenAI hostname alone never selects it. Unnamed and existing named/custom backends retain System One. No generic adapter flag, configuration API selector or new public setter is needed for this ticket. Configured aliases remain System One until a separately reviewed use case requires otherwise. Extend the built-in credential-host guard to OpenAI before key access.

[The target contract](../../../specification/backends.md#openai-decisions-target-for-02) defines deterministic string rendering of JSON descriptions, ordered array requests/replies and typed probability validation. Expanded neutral questions preserve existing planners and logical grouping. Reject unsupported fields locally instead of dropping meanings. Confidence remains a reported field, never a threshold or a computed replacement.

Preserve System One adapter name, serialized requests, legacy digests, stored rows and replay behavior. Within the coordinated cache/2 migration of ADR 0120, carry the explicit adapter discriminator and adapter-encoded question; OpenAI and System One cannot share an answer even at one URL/model. Do not add a migration/index design or reinterpret an original System One recording as OpenAI. New OpenAI recordings contain actual body exchanges, never headers or credentials. Actual reported model/usage remain authoritative; mutable aliases retain caller refresh/no-cache rules.

OpenAI text must land after 0442–0444 and before final 0.2 qualification. OpenAI image admission stays refused pending documented/admitted route bounds and costing. The documented 128-image count does not override ADR 0121's SDK bounds or prove runtime compatibility. Existing admitted vision routes proceed under their owners.

## Evidence and remaining questions

The admitted saved guide and [official guide](https://developers.openai.com/api/docs/guides/decisions) establish dedicated Decisions pricing and model. The [create reference](https://developers.openai.com/api/reference/resources/decisions/methods/create) defines ordered typed questions, refusal answers, model and usage. It does not give question/choice maxima or a deterministic-repeat guarantee. The root-owned first probe measures only its stated finite inputs. No paid call or reply fixture exists from this design step.

Ian can overturn the default model, explicit selection door, JSON text rendering, image admission and finite probe bounds through a reviewed amendment. System One compatibility and one-route ownership remain required. Root owns the one fresh ticket review and eventual landing; no landing record is written now.
