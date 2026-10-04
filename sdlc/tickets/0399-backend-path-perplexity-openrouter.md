# 0399: A backend sets its request path, Perplexity is built in, and OpenRouter gets both decide sides

Status: in progress. Fresh High design review accepted `f752bb15d` on 2026-10-04. Fresh High source review accepted `5a2aa3323`; fresh Medium fixture review accepted `4349b91b3`. Both coordinator-run manual checks passed once on `4349b91b3`. Final site and strict proof remain pending. Lane: claude-2, after ticket 0398 (release safety), on Ian's lane order of 2026-10-04. Offline design preparation may proceed while 0398's hosted proof waits. Amends ADR 0114 sections 1, 2 and 6, and ADR 0115 section 3. Closes no existing issue; the findings come from experiment 0003. The queue owner approves implementation after fresh design review.

Milestone: 0.2

## Outcome

1. A backend can name the path its requests post to under its base. The configuration file's `backends` entry takes an optional `path`, and each built-in row in the code carries one. A backend that names no path posts to `systemone`, as every backend does today. The unnamed path (`--url`, `THINKTHEN_BASE_URL`, the configuration's `url`) keeps `systemone` byte for byte.
2. `perplexity` is a built-in backend. Its base is `https://api.perplexity.ai/v1`, its path is `decisions`, its key variable is `PERPLEXITY_API_KEY`, and its model is `pplx-decider-v1-27b`. `thinkthen check --backend perplexity` posts to `https://api.perplexity.ai/v1/decisions`.
3. `openrouter` is a built-in backend. Its base is `https://openrouter.ai/api/v1`, its path is `systemone`, its key variable is `OPENROUTER_API_KEY`, and its model is `typesafe/jev-1.13`. It sends descriptions as authored, and it sends both decide sides. When a yes-or-no wire question carries a description for one side only, ThinkThen sends `{}` for the other side. `thinkthen check --backend openrouter` then passes its `noul` and `mixed` rows.
4. Every other backend, and the unnamed path, sends the same request bytes as today. No cache key, recording, or fixture of an existing backend changes.

## Evidence

- Starts from: the following evidence.
  - Experiment 0003, `botassembly/thinkthen-exp` `experiments/0003-hosted-decision-backends/README.md` and `docs/wire-notes.md`. Perplexity's Decisions API answers only at `POST https://api.perplexity.ai/v1/decisions`; its documents say "Any other path, including a trailing slash, returns 404." `thinkthen check` with a configured `perplexity` entry stopped at its first request with status 404 at `/v1/systemone` (`results/checks/perplexity-systemone.txt`). A direct POST of ThinkThen's exact noul probe body to `/v1/decisions` answered 200 with the System One reply shape (`results/checks/perplexity-direct-decisions.txt`). Perplexity also accepted one-sided criteria. Its limits are 128 questions a request, 262,144 input tokens and 32 MiB, all above ThinkThen's defaults. It allows 10 requests a second per organization and prices input at $0.04 per million.
  - The same experiment found OpenRouter's System One route at base `https://openrouter.ai/api/v1`. The base `https://openrouter.ai` returns the website's HTML. `thinkthen check` passed `connection`, `key`, `endpoint`, `choice`, `score` and `usage`, and failed `noul` and `mixed` with status 400, because OpenRouter's schema requires a non-null `criteria.false` whenever a noul question carries `criteria`. `results/checks/openrouter-probes.md` records the probes: `"false": null` is refused, and `"false": {}` is accepted and answers. The bare noul form with no criteria passes. Replies add `id`, `provider` and `usage.cost`, which ThinkThen's decoder read unchanged. `typesafe/jev-1.13` answered as `typesafe/jev-1.13-20260917`, and the full Beatles Bench ran clean through it at 67.2% against direct Jev's 67.4%.
  - The product manager's addendum of 2026-10-03, approved in Ian's lane order of 2026-10-04: let a backend entry set its path, add Perplexity as a built-in, and fill the missing side with a neutral default when the backend requires it. Databricks stays out of 0.2.
  - The path today. `crates/thinkthen/src/core/adapters/systemone.rs` holds `ENDPOINT_PATH = "systemone"`. `address` in `crates/thinkthen/src/core/backend.rs` trims the base and always appends that one path. `Backend::resolve(url, base, model)` calls it, and `Backend::is_built_in` strips it to compare with `DEFAULT_BASE`.
  - The built-ins today. `crates/thinkthen/src/core/adapters/systemone/backends.rs` holds `BUILT_INS: [BuiltIn; 3]` with `name`, `base`, `keys`, `model` and `descriptions`. `core/backend/named.rs` builds `Named` from it, finds names with `find`, lists them with `built_in_list`, and guards keys with the ADR 0114 section 5 host rule. `config/backends.rs` reads entries with `url`, `key_env`, `model` and `requests_per_minute`, and lets a built-in name hold only `requests_per_minute`.
  - The criteria today. `NoulCriteria::described` in `crates/thinkthen/src/core/adapters/systemone/request.rs` drops an absent or null side and drops `criteria` when both are absent. `RequestQuestion::asking` uses it for `decide`, and the tag branch of the encoder calls `NoulCriteria::described(description, None)`, so every described tag label sends only `true`. `Descriptions` in `core/plan.rs` already carries a per-backend wire form from the backend into every plan and into `core/check.rs` `probes`, and ADR 0115 section 6 shows the cache key covers it because it hashes the bytes as sent.
  - `specification/check.md` pins the four probe bodies. The `noul` body sends `criteria.true` alone, and the `mixed` body's two tag questions do the same. `specification/fixtures/check/requests-text.jsonl` holds the Ollama form of the same four bodies.
  - `site/src/lib/backends-table.mjs` parses the Named backends table in `specification/backends.md` with a fixed header, and `site/src/pages/install/backends/index.astro` fails the site build when a built-in has no `BACKEND_PAGES` entry or no `CHECKS` entry in `site/src/data/backend-checks.mjs`.
- Keeps: the following behavior.
  - The unnamed path, its `THINKTHEN_API_KEY`, its model order and its posting URL `BASE/systemone`.
  - `typesafe`, `liquid` and `ollama`: base, key variables, model, description form, path `systemone`, and request bytes. Every existing request and check fixture passes byte for byte.
  - ADR 0114 sections 3 to 7: the tiers, the key rules, the cache rule, and the host guard. A built-in whose base host is loopback stays out of the guard's "other" side (ADR 0115 section 2).
  - The address rules in `address`: scheme, loopback-only `http://`, no user information, no query or fragment, host lower-casing. The path joins the base after those checks, so a path can never move the host.
  - Configured entries send descriptions as authored. The configuration file names no description form (ADR 0115 section 5).
  - Ruling 14 and ticket 0343: no backend has a rate of its own. `perplexity` and `openrouter` carry none in code.
  - Every refusal stays value-free, and every existing configuration refusal sentence stays word for word except the two this ticket names.
- Changes: per area.
  - Path.
    - `BuiltIn` gains `path`. Each row names it; `typesafe`, `liquid` and `ollama` name `ENDPOINT_PATH`.
    - `address` takes the path. `Backend::resolve` retains its signature and delegates to a private named-path resolver with `ENDPOINT_PATH`. `Named` carries the path, and `Choice::backend` passes the named backend's path at whichever address its tier gives, as rule 4 already does for the key, model and description form. `--backend perplexity --url http://127.0.0.1:8080/v1` posts to `http://127.0.0.1:8080/v1/decisions`. The resolved URL still supplies cache, recording and rate identity.
    - `config/backends.rs`: an added entry may hold `path`, a string. A path is 1 to 128 bytes of one or more segments joined by `/`. A segment is ASCII letters, digits, `-`, `.`, `_`, `~` and `@`, and is neither `.` nor `..`. No leading, trailing or doubled `/`, no `%`, `?`, `#` or space. A refused path exits 5 with `configuration backend field `path` must be one or more segments of letters, digits, and `-._~@`, joined by `/`` and repeats no value. The extra-field sentence becomes `configuration backend entries hold only `url`, `path`, `key_env`, `model`, and `requests_per_minute``. A non-string `path` gets the existing type sentence, which gains `path`: `configuration backend entries are objects whose `url`, `path`, `key_env`, and `model` are strings`. One `STRINGS` constant in `config/backends.rs` then covers every type error, which is simpler than a second sentence. A built-in's entry still holds only `requests_per_minute`.
    - `Backend::is_built_in` keeps comparing against the `typesafe` base and path.
  - Built-ins.
    - Two rows in `BUILT_INS`, in name order: `openrouter` and `perplexity` as Outcome items 2 and 3 give them. The vendor words stay in the adapter's child module, as ADR 0114 section 1 requires.
    - The host guard now knows `openrouter.ai` and `api.perplexity.ai`. `TYPESAFE_API_KEY` never goes to `openrouter.ai`, and `OPENROUTER_API_KEY` and `PERPLEXITY_API_KEY` never go to another built-in's host. The guard code reads the table and needs no change.
    - The unknown-name sentence lists five names: `unknown backend `NAME`; the built-in backends are `liquid`, `ollama`, `openrouter`, `perplexity` and `typesafe`, and the configuration file may name more`.
    - A configuration entry named `perplexity` or `openrouter` with `url`, `key_env` or `model` now exits 5 with the existing built-in sentence. The experiments team's 0003 configuration used both names. The CHANGELOG says so, and the builder tells the experiments team before new-model day: delete the entry, or keep only `requests_per_minute`.
  - Both sides.
    - `Descriptions` gains a third form, `BothSides`: every description as authored, and a yes-or-no wire question that carries one side gets `{}` for the other. `openrouter` names it. A question with neither side still sends no `criteria`. A null side counts as absent, as ADR 0110 rules.
    - `request.rs`: `NoulCriteria::described` takes the form, and both the `decide` branch and the tag branch pass it. `choice` and `score` bytes do not change under any form.
    - `specification/check.md` line 5 says check "sends four fixed requests to `BASE/systemone`". It now says check posts its four requests to the backend's path under the base, `systemone` unless the backend names another.
    - `core/check.rs` needs no new code, because `probes` already takes the backend's form. A new fixture, `specification/fixtures/check/requests-both-sides.jsonl`, holds the four probe bodies under `openrouter`. `specification/check.md` names it beside `requests-text.jsonl`.
    - `drops_detail` stays true for `Text` only. `BothSides` drops nothing, so `check` and `--plan` print no workaround warning for it.
  - Records and docs.
    - ADR 0114 gains an amendment: two more built-ins, and the entry's `path`. ADR 0115 gains an amendment: the third form and why it is not a workaround. A required field is a server's schema rule, not a bug, so it has no debt issue.
    - `specification/backends.md`: the Named backends table gains a `Path` column and the two rows, and its text covers the path rule, rule 4 for the path, the five-name sentence and the `BothSides` form. `site/src/lib/backends-table.mjs` reads the new header.
    - `specification/settings.md`: the Backend row lists the five names; the Named backends row names `path`. `specification/recording.md` names `path` where it lists the entry fields.
    - `site/`: a page for each new built-in under `site/src/pages/install/backends/`, entries in `BACKEND_PAGES`, and `CHECKS` entries for the live checks in Proof. Each page names the key variable, the address and current published price and limits, checked against official sources in the design record. Experiment 0003's measurements remain dated. The page says the backend paces nothing unless the configuration file sets `requests_per_minute`. The Perplexity page suggests `"perplexity": {"requests_per_minute": 600}` and explains that this paces one process. Multiple processes share the provider's organization limit.
    - The CHANGELOG's unreleased section names the two built-ins, the path field and the break for entries named `perplexity` or `openrouter`.
  - Ticket 0377 pins the three-name sentence in its corpus case 5. Whichever of 0377 and this ticket lands second updates the other's pin.
- Proof: each item fails if the build is wrong. Tests replay saved responses and post only to loopback.
  - Path, outside in, in `crates/thinkthen/tests/backend/named_backends/command.rs`. A loopback listener counts requests by path.
    - A configuration entry `local-pp` with `url` at the listener, `path` `decisions`, a test `key_env` and a model sends one `decide` request to `/v1/decisions` and zero to `/v1/systemone`.
    - `--backend perplexity --url` at the listener posts to `/v1/decisions` with the `PERPLEXITY_API_KEY` marker.
    - `--url` at the listener with no backend posts to `/v1/systemone`, as before.
    - `--backend perplexity --plan` prints the posting URL `https://api.perplexity.ai/v1/decisions` and model `pplx-decider-v1-27b`, and sends nothing.
  - The path refusal table in `config/backends.rs`: an empty string, `/decisions`, `decisions/`, `a//b`, `..`, `a/./b`, `a?b`, `a#b`, `a b`, `%2e` and a 129-byte path each exit 5 with the path sentence. A number exits 5 with the type sentence that now names `path`. An entry with a non-string `url` reads the same type sentence. A planted marker in the path never appears in the output. `decisions`, `api/alpha/decisions` and `ai/run/@cf/cloudflare/clef` are accepted.
  - The host guard, outside in, with the existing `CONNECT`-counting proxy in `crates/thinkthen/tests/backend/named_backends.rs`. `--backend perplexity --url https://api.typesafe.ai/v1`, `--backend openrouter --url https://api.liquid.ai/decisions/v1` and `--backend typesafe --url https://openrouter.ai/api/v1` each exit 2 with the ADR 0114 section 5 sentence naming the right variable and owner, and the proxy counts zero `CONNECT`s.
  - The five-name sentence on the command and on `EngineBuilder::backend`, in `named_backends/command.rs` and `named_backends/builder.rs`.
  - A configuration entry `perplexity` holding `url`, `key_env` and `model` exits 5 with `a configuration entry for a built-in backend holds `requests_per_minute` and nothing else`. `"openrouter": {"requests_per_minute": 60}` is accepted and paces `openrouter`.
  - The both-sides edge table in `request_tests.rs`, under `BothSides` and under `Authored`:

    | Question | `BothSides` sends | `Authored` sends |
    | --- | --- | --- |
    | `decide` with `--true` only | `{"true":"…","false":{}}` | `{"true":"…"}` |
    | `decide` with `--false` only | `{"true":{},"false":"…"}` | `{"false":"…"}` |
    | `decide` with both | both as authored | both as authored |
    | `decide` with neither, or both null | no `criteria` | no `criteria` |
    | `decide` with an object `--true` | the object, and `"false":{}` | the object |
    | `tag` label with a description | `{"true":DESCRIPTION,"false":{}}` | `{"true":DESCRIPTION}` |
    | `tag` label with no description | no `criteria` | no `criteria` |
    | `choose` and `score` | today's bytes | today's bytes |

  - `check` against a loopback arm that answers 400 to any noul question whose `criteria` lacks `false`, as OpenRouter's schema does. `check --backend openrouter --url` at the arm exits 0 with eight `ok` rows and zero warnings. `check --url` at the same arm with no backend reports `critical noul` and `critical mixed`, as experiment 0003 saw. `check --backend openrouter --plan` prints the bodies in `requests-both-sides.jsonl` byte for byte.
  - A saved OpenRouter reply with `id`, `provider` and `usage.cost`, copied from experiment 0003's capture with no key, decodes under `--replay` to the same answer as the System One fixture.
  - Cache: one `decide --true` question asked under `openrouter` and under the unnamed path at the same loopback URL and model makes two cache entries. A `decide` with no descriptions makes one, because the bytes match (ADR 0115 section 6).
  - Unchanged default: `requests.jsonl`, `requests-text.jsonl` and every `specification/fixtures/systemone/*.request.json` pass byte for byte. The full named-backend suite passes.
  - `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/settings`, `sdlc/scripts/tickets`, full `sdlc/scripts/lint`, the site build and the site's check script.
  - Live, by hand. Landing waits on these two checks. `site/src/pages/install/backends/index.astro` and `checkLines` in `site/src/data/backend-checks.mjs` throw without a `CHECKS` entry for every built-in, and experiment 0003 holds no passing `thinkthen check` for either backend. Ian authorized both manual checks on 2026-10-04 under a combined $1 cap through `sdlc/scripts/live` with a token cap. That authority persists; no second permission request is needed. The coordinator alone executes the reviewed run plan in [the design record](../planning/0399-backend-path-design.md). Each child enforces `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10000`, and each wrapper invocation reserves 10000 tokens. One check per backend, at most sixteen attempts each, has a conservative published-limit cost bound below $0.20 combined. An automatic retry of a wrapper or check is forbidden. Experiment 0003's $0.0000042 Perplexity and $0.0001 OpenRouter figures describe 2026-10-02, not this run's budget. Each check must exit 0 and print `critical 0`. The ticket records both transcripts, and the site's `CHECKS` cites them.
- Defers: the following gaps.
  - Databricks `ai_decide`, which serves on each workspace's own address and names its probability `probability`. Out of 0.2 by the product manager's addendum.
  - A path for the unnamed path. `--url` and `EngineBuilder::base_url` keep `systemone`. A user names a configured entry to reach another path.
  - A configuration field that turns on both sides for an added entry. Ticket 0400 slice A proposes it in the provider setup format.
  - A default rate for `perplexity`. Ruling 14 holds, so the page suggests one and the code sets none.
  - Whether OpenRouter's `typesafe/jev-1.13` is a moving name. It answered as `typesafe/jev-1.13-20260917`. The cache treats every model but `jev-latest` as pinned. New-model day can check whether the dated name is accepted. If it is, the default model moves to it.
  - Perplexity's model through OpenRouter. OpenRouter answered `Model perplexity/pplx-decider-v1-27b does not exist` on 2026-10-02. Its official model page lists the model on 2026-10-04. This ticket checks direct Perplexity and OpenRouter Jev only; it proves no new OpenRouter Perplexity route.
  - Images in `state`. Perplexity accepts them, and ThinkThen sends none.

## Design notes

The proposed implementation, review boundaries and manual run controls live in [0399-backend-path-design.md](../planning/0399-backend-path-design.md). That record authorizes no additional paid call.

- The fill value is `{}`. Experiment 0003's probe showed OpenRouter accepts it and answers. ThinkThen already sends `{}` for a `score` level whose description is null, so `{}` is the tool's own word for "no description". A sentence such as "otherwise" would add words the author never wrote and could move the answer. Ian can overturn the value.
- `BothSides` rides the existing `Descriptions` plumbing, because that value already reaches every plan, every check probe and the cache key. A separate field would need the same plumbing twice. The builder may rename the type if a clearer name fits both meanings.
- The path follows the backend at any address, as the description form does (ADR 0115 section 5). A Perplexity-compatible server on loopback is reached with `--backend perplexity --url`.

## What Ian can overturn

- `openrouter` as a built-in. The alternative leaves it a configured entry and lets the entry turn on both sides (ticket 0400 slice A). The built-in costs the experiments team one configuration edit.
- The fill value `{}`.
- The defaults `pplx-decider-v1-27b` and `typesafe/jev-1.13`.

## What the build taught us

The coordinator ran each reviewed manual job once on `4349b91b3` on 2026-10-04. Both exited 0 with critical 0 and warning 0. [The build record](../records/0399-backend-path-build.md#actual-manual-provider-receipts) retains the actual transcripts, model identities and spend limits. Successful replies report a combined usage cost of $0.000098326; attempt counts are not printed, so this is not an exact total bill. The pair retains its conservative $0.20 allowance and charged 20000 durable reserved tokens. No extra paid call is authorized here.

The coordinator's full test and specification proof uses an owned empty HOME with explicit build-cache homes. It does not prove default-HOME fixture independence. The remaining ambient configuration problem is [Debt 038](../issues/2026-10-04-local-test-fixtures-read-ambient-configuration.md), owned by the next reviewed fixture-isolation slice related to 0404. Runtime configuration refusal remains correct and unchanged. The saved OpenRouter replay fixture reconstructs retained response fields and explicitly substitutes a fixture-only identifier; it is not a complete raw capture.
