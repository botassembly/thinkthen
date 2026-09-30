Status: closed 2026-09-30. Item 3 replaced by ADR 0111, which withdraws folder binding. Item 4's page half fixed in `specification/settings.md`, and item 5 fixed by ticket 0154.

# Architect review 06: backends and configuration

A fresh reviewer tested backend addresses, `check`, settings tiers and the configuration file as an architect who points ThinkThen at a hosted or local model. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 5 severity 2 and 9 severity 3 issues. The full detail sits in the architect review report 273, 06. Work file names such as `06-work/fake06_server.py` refer to the report's local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. `check` fails against the reference backend, and spec-legal `score` questions fail live (severity 2)

Evidence, live. `check --url https://api.typesafe.ai/v1` exits 4 with `critical score: ... status 422`. A live `score @s-null.json`, whose level has a `null` description, gets 422. `@s-obj.json` passes. `backends.md` allows a `null` description, and `check.md`'s score probe sends `[null, "...", {...}]`.

What an integrator hits. A CI gate on `check` goes red against Jev. A score question file that follows the spec gets refused with a phrase that blames size or format.

Direction. Stop sending `null` in score criteria, for example by sending the level name, or refuse it locally with a clear message. Then make the check probe match what Jev accepts. The review also found that `check` passes a reply naming a different model and a reply with a duplicate answer name (item 2), so `check` needs those fixtures too.

## 2. A reply with a duplicate answer name is accepted, and the last value silently wins (severity 2)

Evidence. Fake mode `dupkey` sends `"q1":{"noul":0.01}` and then `"q1":{"noul":0.9}`. `decide` prints `true` at exit 0, and `check` passes. The code reads `answers` into a `BTreeMap` (`core/adapters/systemone/response.rs:22-27`). Question files refuse duplicate names, but replies do not. Choice and score `probabilities` use the same map type. The reviewer did not reproduce that case.

What an integrator hits. A buggy shim or proxy can send an ambiguous answer, and ThinkThen picks one side without a word. Another JSON reader, such as `jq` on the saved raw entry, may pick the other side.

Direction. Refuse duplicate member names in `answers` and in each `probabilities` map, and add a `check` fixture for it.

## 3. The default cache binds to one address, even on a run that sends nothing (severity 2)

Reviews 06 (I-2 and I-10), 08 (issue 10) and 11 (issue 9) all found this. This file carries it.

Evidence from review 06. A fresh cache, then `decide` with no key at the default address. It exits 4 and writes `.thinkthen-backend.json`. The next run at a loopback address exits 5 with "go back to that address", but the message names only the new address. `cache prune --max-size 1` does not clear the binding. `recording.md` line 36 allows binding on first write-capable use. Ticket 0124 deferred "binding a folder only when an entry is written", a `status` line for the binding, and a per-address default cache.

Evidence from reviews 08 and 11. `localhost` against `127.0.0.1` for the same server exited 5. A run to address A, then a run to address B with no flags, gave exit 5: "the default cache is bound to a backend address other than ...".

What an integrator hits. A new user tries the tool with no key, then points it at a local model, and every plain run fails. The message sends them to an address they cannot see. A worker that fails over to a secondary URL, or switches from staging to production, gets a local-failure exit on every call until someone sets `THINKTHEN_CACHE` per address or passes `--no-cache`.

Direction. Key the default cache folder by address, with a subfolder per backend digest. Failing that, bind on the first written entry, show the binding in `status`, and name the marker file or a command that clears it in the refusal.

## 4. The settings page says the configuration file's `cache` takes a folder, and it takes only a boolean (severity 2)

Reviews 06 (I-5) and 08 (issue 4) both found this. This file carries it.

Evidence. `specification/settings.md:19` says "then the configuration file's `cache`, then the platform folder", and the Answer cache row gives the allowed values as "A folder, or off" with config `cache` in the same row. `config.rs:36` declares `cache: Option<bool>`. `{"cache":"/tmp/somewhere"}` exits 5 on every command with `the configuration file is not valid closed JSON`, and `status` exits 5 too.

What an integrator hits. They follow the settings page and lock themselves out of every command, including `status`, with no hint which line is wrong. A fleet configuration that sets a cache folder in the file breaks every command.

Direction. Correct the row to say the file holds on or off only, or add a folder field. Make closed-shape errors name the field.

The code half is done by Quick Fix qf-config-ruby-issue-status. A closed-shape refusal now names the field and never its value. `{"cache":"/tmp/somewhere"}` reads ``configuration field `cache` must be true or false``. A field outside the five reads ``the configuration file holds a field other than `schema`, `url`, `model`, `cache`, and `cache_bytes` ``. The settings page half stays with H4, which corrects the row and line 19 through item 8 of `2026-09-26-settings-table-gaps-a-site-reader-hits.md`.

## 5. The built-in request ceiling covers only relation plans, so one long record reaches the backend and stops the run (severity 2)

Five reviews found this: 10 (issue 4, severity 2), and 01 (issue 7), 03 (issue 3-5), 06 (I-6) and 11 (issue 6) at severity 3. This file carries it. Ticket 0154 plans a request size setting, and `2026-09-26-recognize-design.md` section 6 (R4) plans the recognize fix. Neither covers every verb at the built-in address today.

Evidence. `backends.md:21` and `:23` limit the 96,000-byte built-in ceiling to relation plans. `core/backend.rs:112-119` applies it only at the built-in URL. Five live refusals:

- Review 10: 336 words with three kinds exited 4 with `400 (max_tokens_exceeded)`. The dry run showed `request_count: 1` at 423,650 bytes. `recognize.md:7` implies long text is handled.
- Review 03: a 2,600-question `annotate` set (467,469 bytes) passed `--dry-run`, then failed on the first record with `400 (max_tokens_exceeded)`, exit 4. A 1,500-question set (268,369 bytes, 51,658 input tokens) passed.
- Review 01: a 415 KB `filter` record was uploaded whole before the service refused it with 400 `max_tokens_exceeded`.
- Review 11: a live 400 KB document got `400 max_tokens_exceeded`, exit 4. Locally, a 10 MB line passes the 16 MiB check and is sent.
- Review 06: the same text ran at 0.164 tokens per byte, while the ceiling assumes 0.516.

What an integrator hits. A pipeline that passed on sample sentences fails on real documents. For `recognize`, every record over about 300 words fails. One overlong ticket in a nightly queue fails the job every night at the same record, because a run stops at the first failed record. The fix needs a hand-written backend profile that the integrator must discover from the error message.

Direction. Apply the built-in ceiling to every verb at the default address, so groups split and single records refuse locally at exit 2 before upload. Until then, make `--dry-run` flag a plan over the known limit and correct `recognize.md:7`. Document that bytes stand in for tokens only roughly, and give a measured ratio for each backend.

## Carried in other files

- I-1, a partial reply is cached and served forever (severity 2). Five reviews found this. The architect review 08 file carries it.

## Severity 3 titles

- There is no request size or token setting on main, and outside the built-in address nothing bounds a request. Carried above as item 5 and in ticket 0154.
- A broken configuration file stops every command, including `status` and runs that override all its values.
- The model has no environment tier, and a question file's model beats the machine's configured model.
- A keyless local server still needs a dummy key, and the run fails at exit 4. Done by Quick Fix qf-command-edges-and-prune, which sends to loopback with no key.
- The default cache serves one address. Carried above as item 3.
- An extreme `--timeout` panics with exit 101. Carried in the architect review 11 file.
- The release binary honors a test-only variable that removes retry backoff. Carried in the architect review 07 file.
- Model names are not trimmed or checked for control characters.
- Refusal phrases give advice that does not fit the case. Reviews 06 (I-14) and 07 (I14) both found this, and this file carries it. An unknown model gets the generic 400 phrase "check --model and the request size". A single 400 KB document gets "set a lower max_request_bytes with --profile". 413 has no phrase. The final 429 message does not say retries happened, while 503 reads "failed after the allowed attempts". Direction: split the phrases by cause, and name the field in decode refusals.
