# ADR 0115: An `ollama` built-in backend, and descriptions each backend accepts

- Status: **Proposed**, 2026-09-30. Ticket 0339 builds it. Ian can overturn each item.
- Date: 2026-09-30

Ian asked on 2026-09-30 for a built-in `ollama` backend. Ollama 0.35 serves System One at `/v1/systemone` on the user's own machine and needs no key. ADR 0114's coordinator default 1 held more built-ins back until a user needs one. That condition is now met. This ADR amends ADR 0114 sections 1, 2, 5 and 6, and ADR 0110's scope. ADR 0111's question key stays as it is.

## Context

Three backends read criteria descriptions differently.

- TypeSafe accepts a string, an object, a list and null. Its published OpenAPI 3.1.0 schema says so, and a hosted `thinkthen check` passed every probe on 2026-09-30.
- Liquid d1 refused a null description on a `noul` question. ADR 0110 already omits that null for every backend, and the hosted recheck of 2026-09-30 passed with `critical 0, warning 0` on today's bytes. Those bytes keep `null` option values on `choice`, so d1 accepts them.
- Ollama 0.35 refuses an object description with status 400 (`score criteria must be an array of descriptions`). It accepts strings and null option values. Experiment 415 found three criticals from `check` (choice, score, mixed) for this one cause. Its bodies are pinned in `sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md`.

That issue proposed one flattened form for every backend and said TypeSafe loses nothing. That is wrong. An object's `not_for` and `examples` travel into the prompt, and `specification/types.md` says dropping a description changes the question. TypeSafe reads them. So the rendering belongs to the backend.

The key rules today: `engine/facade.rs` sends no `Authorization` header when the key is unset and the address is `localhost`, `127.0.0.1` or `[::1]`. `http://` reaches only those three hosts. ADR 0114 section 5 refuses a built-in's key variable at the host of another built-in's base.

## Decision

### 1. The `ollama` built-in

| Name | Base | Key variables | Model | Descriptions |
| --- | --- | --- | --- | --- |
| `liquid` | `https://api.liquid.ai/decisions/v1` | `LIQUIDAI_API_KEY`, then `LIQUID_API_KEY` | `d1:free` | `authored` |
| `ollama` | `http://localhost:11434/v1` | `OLLAMA_API_KEY` | `nimble` | `text` |
| `typesafe` | `https://api.typesafe.ai/v1` | `TYPESAFE_API_KEY` | `jev-1.13.0` | `authored` |

- At its default base, `ollama` needs no key. The existing loopback rule sends no `Authorization` header when `OLLAMA_API_KEY` is unset or blank. No new rule is added.
- At a base that is not loopback, `ollama` reads `OLLAMA_API_KEY` as every named backend reads its variable. An unset or blank value exits 4 with `the environment variable `OLLAMA_API_KEY` is unset or blank, so no key is sent`. `OLLAMA_API_KEY` is the name Ollama's own tools use for its hosted service. A machine on the local network is reached over `https://` or a tunnel that ends on loopback, as ADR 0010's clear-text rule already requires.
- `nimble` is Ollama's recommended decision model and experiment 415's pick. A user names `tev1` or `tev1:0.8b` with `--model`.
- The table stays in `core/adapters/systemone/backends.rs`. The unknown-name sentence lists the names from the table, so it becomes `unknown backend `NAME`; the built-in backends are `liquid`, `ollama` and `typesafe`, and the configuration file may name more`.

### 2. A loopback base is no built-in's host for the key guard

ADR 0114 section 5 refuses a built-in's variable at "the host of a different built-in backend's base". The `ollama` base host is `localhost`. Read literally, the rule would refuse `--backend liquid --url http://localhost:8080/v1`, which ADR 0114 rule 4 allows on purpose. So a built-in whose base host is loopback is left out of the "other" side of the comparison. Any program on the user's machine can listen on loopback, and the user named that address.

The guard still covers the new variable. `--backend ollama --url https://api.typesafe.ai/v1` is refused with `thinkthen: backend `ollama` reads `OLLAMA_API_KEY`, the key of backend `ollama`, which never goes to the address of backend `typesafe``. A configuration entry that names `OLLAMA_API_KEY` at the Liquid base is refused the same way. `TYPESAFE_API_KEY` and `LIQUIDAI_API_KEY` stay refused at each other's hosts.

### 3. Each backend names how its descriptions travel

A description is the text for a `decide` side, a `choose` option, a `tag` label or a `score` level. Each backend names one of two dialects.

| Dialect | String | Object with a nonblank string `what` | Other object or list | Empty object or null |
| --- | --- | --- | --- | --- |
| `authored` | as written | as written | as written | today's rule for its question type |
| `text` | as written | the `what` text alone | its compact JSON as one string | no description |

`authored` is today's encoder, byte for byte. It keeps ADR 0110's omission of a null `noul` side, which applies to every backend. `typesafe` and `liquid` use `authored`. Liquid needs no dialect of its own, because its hosted check passed on these bytes.

Under `text`, "no description" follows the question's shape.

- `decide` and a `tag` label: the `criteria` side is left out, and `criteria` goes when neither side remains.
- `choose`: the option keeps its key with a `null` value. Ollama accepts a null option value, and the option must stay.
- `score`: the level travels as its name, as a level with no description does today. No `{}` and no `null` enter the array.

The `text` dialect renders each description before the encoder builds the question. A `tag` question whose text is a string and whose descriptions all render as strings therefore takes the plain sentence form, not the array form. Question text that is itself an object or a list travels as written under both dialects. Nobody has checked whether Ollama accepts it (section 6).

### 4. Where the rendering says it lost fields

`text` drops an object's other fields when it sends `what` alone. Two places say so.

- `thinkthen check` prints one line after `model sent`, live and under `--plan`: `note descriptions: backend `NAME` sends each description object as its `what` text, so its other fields are left out`. It is neither a finding nor counted in the last line. The check's own probes hold `not_for` and `examples`, so this line always prints under a `text` backend.
- `--plan` on an asking command prints once on standard error: `thinkthen: backend `NAME` sends each description object as its `what` text, so its other fields are left out`. It prints only when at least one planned description lost a field. The exit code stays the same.

A live run prints nothing more. The dialect is part of the backend the user named.

### 5. The configuration file may name a dialect

A `backends` entry gains one optional field, `descriptions`, whose value is `authored` or `text`. When it is absent, `authored` applies. Any other value is refused with exit 5, as every configuration refusal is, and the refusal names the field without repeating the value. The unnamed path always uses `authored`. `--backend ollama --url BASE` keeps `text`, because a named backend's rules apply at the address its tier names (ADR 0114 rule 4).

### 6. The cache key needs no change

ADR 0111 keys one question on the adapter name, the posting URL, the model, the state and that question's bytes as sent. The dialect renders before those bytes are written, so the key covers it. Two dialects that write different bytes for one question get different keys, and neither reuses the other's answer. Two that write the same bytes, such as a question with only string descriptions, share one answer, because the backend was asked the same question. The recording digest hashes the URL and the whole body, so it follows the same rule. The replay decoder reads options from `criteria` keys and levels from the array length. Both are unchanged under `text`. The store, fixtures, recordings and usage totals hold no dialect name.

## What this amends

| Source | Change |
| --- | --- |
| ADR 0114 sections 1 and 6 | A third built-in, `ollama`, and a descriptions column. The unknown-name sentence lists three names |
| ADR 0114 section 2 | A `backends` entry may name `descriptions` |
| ADR 0114 section 5 | A loopback built-in base is not another built-in's host |
| ADR 0114 coordinator default 1 | Met: a user needs `ollama` |
| ADR 0110 | Its null `noul` omission is part of the `authored` dialect; `text` also omits empty and null descriptions per question shape |
| `specification/types.md` | Descriptions travel as the backend's dialect says; `ollama` sends the `what` text |
| `specification/backends.md`, `check.md`, `settings.md`, `recording.md` | The built-in row, the note line, `OLLAMA_API_KEY`, the entry field |
| README, CHANGELOG | The `ollama` backend |

## Rejected

- One flattened form for every backend. It drops `not_for` and `examples` that TypeSafe reads, and it changes every cache key and fixture for TypeSafe users.
- A third `liquid` dialect that omits nulls. `authored` already omits the one null Liquid refused.
- Refusing a question whose descriptions `text` cannot carry in full. A user who picks Ollama wants the question asked, and the note says what was left out.

## Deferred gaps

- Nobody has checked whether Ollama accepts an object `state`, which the check's mixed probe and a pointer selection send. The loopback mimic assumes it does.
- Nobody has checked whether Ollama accepts structured question text (an object or a list as `instructions`). A ticket adds a rendering when a user meets a refusal.
- Ollama's hosted service may later serve decision models. Its host is not added to the key guard until it does.
- An Ollama tag such as `nimble` can change when the user pulls it again. Cached answers keyed on `nimble` then outlive the old weights until the user passes `--refresh-cache` or `--no-cache`. ADR 0111 forces a refresh only for `jev-latest`.
- Bindings and SQL gain `backend` in ADR 0114 build slice 2. They inherit the dialect through the backend.
- `status` does not print the dialect.

## Coordinator defaults Ian can overturn

1. `ollama` is a built-in with base `http://localhost:11434/v1`, model `nimble` and optional key `OLLAMA_API_KEY`.
2. A loopback built-in base does not count as another built-in's host for ADR 0114 section 5.
3. Two dialects: `authored` for `typesafe`, `liquid`, the unnamed path and configured entries by default, and `text` for `ollama`.
4. Under `text`, an object sends its `what`, and any other object or list sends its compact JSON text.
5. `check` and `--plan` state the loss in one sentence. A live run stays quiet.
6. The cache keeps caching Ollama answers under a tag name, as it does for every model name except `jev-latest`.
