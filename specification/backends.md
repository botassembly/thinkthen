# Backends

Status: **Settled**, amended by ADR 0048, for the wire shape, the key, the address, the request, the `systemone` adapter, and every adapter row.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

A backend is an address and a model. `THINKTHEN_BASE_URL` or the configuration file's `url` names the address, `THINKTHEN_API_KEY` holds the key, and `--model` names the model. A named backend pairs an address with its own key variables and a model; see [Named backends](#named-backends). An explicit profile may add a stable name and local limits. It never selects either value.


The optional `max_estimated_input_tokens_total` setting admits each final encoded body using `ceil(body bytes × 908 / 1000)` (`encoded-body-bytes-908-v1`). The process retains each started attempt's estimate, including failed replies, retries and refusal-split children; a stopped attempt before transport refunds it. Cache and replay answers add no charge. Each engine selects its own limit against the retained sum. This is an estimated input admission bound, not a hard provider token, output, dollar or billing cap. A future coefficient or body-coverage change needs a new version and review. `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL` sets the limit on every surface; an explicit flag, setter or C key outranks it.

## Explicit profiles and local preflight

Settled by ADR 0032 and amended by ADR 0040.

`--profile FILE` reads one UTF-8 JSON object with schema `thinkthen.backend-profile/1`. It requires `name` and at least one of `max_evidence_bytes`, `max_request_bytes`, `max_questions`, or `max_options`. The name is nonempty and uses lowercase letters, digits, hyphens, and underscores. Each limit is a positive integer. No other key is accepted.

Evidence bytes are the UTF-8 bytes after field extraction and JSON normalization. Request bytes are the exact bytes the adapter encoded. Questions are the expanded wire questions, so each `tag` label counts once. Options are the labels in one `choose` question. An exact limit passes. One unit over exits 2 and names the profile, limit, unit, and actual count. The diagnostic repeats no evidence or request bytes.

Recognition's step-1 and step-2 requests carry a window of the text: six pieces on each side of the pieces or names they ask about, as [recognize.md](recognize.md) says. A step-1 request holds at most 40 pieces. Relation requests carry the whole text. Every command packs its questions into requests through the one packer of ADR 0111 section 4, so `max_questions` and `max_request_bytes` can create several requests without changing offsets or assembly. In `relate`, a rule asks yes/no pair questions, or one menu per source when it is `single`. Its requests hold at most 400 questions at every address, and a profile's `max_questions` can lower that bound. `max_options` counts a `single` menu's options and does not affect yes/no pair questions.

The command checks an optional snapshot of the effective key against the resolved address before encoding, reading a stored answer, or opening a connection. The engine then looks up each question and packs only the questions no stored answer covers. A request closes at a new shared state or a full window, or when the next question would pass the request size, a profile limit, or the step's question bound. One input's questions go into one request when they fit together. Otherwise the open request closes first, and the rest spread over the next requests, so a record's questions or a `tag` question's labels can span two requests. A question that passes a limit even alone refuses the call before any request, because splitting it would change meaning. That covers evidence, one-question request and choice-option overflows. `find` and recognition steps 1 and 2 close a request only at a new state or a profile limit.

A request-size setting closes record requests and relation requests at every address. It defaults to 96,000 bytes. `--max-request-bytes N` beats `THINKTHEN_MAX_REQUEST_BYTES`, which beats the default, on `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `annotate`, `relate` and `recognize`. The flag and variable take a whole number of at least 1. A profile's `max_request_bytes` lowers the setting and never raises it; its other limits apply beside both. One question or record alone passes the setting and goes as one request. A profile still refuses a question that passes its own limit alone. The built-in backend refuses a request over 65,536 input tokens with status 400. Ticket 0123 measured 0.516 input tokens per byte for former choice-based relate bodies. Pair-question token density remains unmeasured until ticket 0167's recorded run. A size above 96,000 at the built-in address prints one warning before planning or sending, including under `--plan`. Every request of `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` carries each record once in its quoted question beside the fixed evidence sentence or the context, by ADR 0111. A profile's `max_evidence_bytes` bounds each record's evidence bytes and also the sentence or the context, so any limit below the sentence's 44 bytes refuses every such request. Its next record closes the request when adding it would pass the smaller limit. ADR 0051 sets the request size's reach.

A retried status resends the whole request. When a backend refuses a request of two or more questions with status 413 or status 400 naming `max_tokens_exceeded`, the command asks its first half, then its second half, once each under the same `--jobs` place, by ADR 0111 section 6. A half refused again fails at exit 4 and is not split again. Plain 400, 422 and other failures do not split. Replay looks up each question, so no halving happens under replay. Both halves store their answers, and each half's rows show the parent's attempts, which count in the first half. The refused whole request stores nothing. A refused first half fails the second without sending it.

`annotate` checks every group selection and each question against the profile before admitting a record to a request. Each packed request then checks its exact body and expanded wire-question count before sending. A later refused record cannot cause its own group to send. Plan previews perform the same checks before printing a plan. Profiles apply equally to live calls, replay, cache, and recording. The profile estimates no tokens. A backend whose published limit is only tokens needs a tokenizer or a verified byte ceiling before its file can enforce the limit. `--url`, `THINKTHEN_BASE_URL`, the configuration file's `url`, and `--model` still select the backend. A profile contains none of them.

## Live attempt observations

Opt-in live attempt observations inspect only `x-envoy-upstream-service-time` and `x-typesafe-request-id` from a System One response, on success or status failure. Exactly one value of each name is required; a duplicate or malformed value is omitted without failing the answer. Server time is unsigned ASCII decimal milliseconds. A request ID is 1–128 ASCII letters, digits, periods, underscores or hyphens, then omitted if it contains the configured nonempty key or occurs verbatim in the posting URL or request body. No other response header becomes result metadata. A transport failure has no response headers. The local `wall_ms` includes transport and bounded body read, excludes waits and parsing, and does not measure model-only time.

## The key

Settled by ADR 0010 and ADR 0114.

With no backend named, the key is read from `THINKTHEN_API_KEY`. A named backend reads only its own key variables, in order, and the first nonblank one wins; `THINKTHEN_API_KEY` is not read unless a configuration entry names it as its `key_env`. The rules below apply to whichever key was selected.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value. A named backend's message names its first key variable: `the environment variable `LIQUIDAI_API_KEY` is unset or blank, so no key is sent`.
- No key appears in a plan, a result, a recording, a log line, or an error.
- A key holding a control character, such as a line feed, a tab, an escape, or DEL, is a usage error before any send. It says `the API key contains a control character` and repeats no part of the key.
- **The key goes to the address the user named.** A nonblank effective key that occurs verbatim anywhere in the resolved posting URL is refused first with `the backend address contains the API key; keep the key out of the address`. The check compares exact UTF-8 bytes without decoding or rewriting the URL. Otherwise a run pointed at another address carries the selected key and nothing else.

## Named backends

Settled by ADR 0114 and ADR 0115. Ian can overturn each default they record.

Three backends are built in:

| Name | Base | Key variables, first nonblank wins | Model | Descriptions |
| --- | --- | --- | --- | --- |
| `liquid` | `https://api.liquid.ai/decisions/v1` | `LIQUIDAI_API_KEY`, then `LIQUID_API_KEY` | `d1:free` | as authored |
| `ollama` | `http://localhost:11434/v1` | `OLLAMA_API_KEY` | `nimble` | as text (workaround) |
| `typesafe` | `https://api.typesafe.ai/v1` | `TYPESAFE_API_KEY` | `jev-1.13.0` | as authored |

At its default base, or at any other loopback address, `ollama` needs no key: with `OLLAMA_API_KEY` unset or blank, the loopback rule sends no `Authorization` header. At an address that is not loopback it reads `OLLAMA_API_KEY` like every named backend, and a missing key exits 4.

`ollama` alone sends descriptions as text. This is a temporary workaround for an Ollama bug: Ollama 0.35 answers status 400 to an object description ([ollama/ollama#18718](https://github.com/ollama/ollama/issues/18718)). It is tracked as debt in [the Ollama issue](../sdlc/issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md) and goes when Ollama accepts object descriptions. Under it, a string travels as written. An object with a nonblank string `what` travels as that text alone, so its other fields are left out. Any other object or list travels as its compact JSON text. An empty object or null travels as no description: a `decide` side or `tag` label drops its `criteria` member, a `choose` option keeps its key with `null`, and a `score` level travels as its name. A `tag` question whose descriptions all become text takes the sentence form. `typesafe`, `liquid`, configured entries, and the unnamed path send every description exactly as authored. `--backend ollama --url BASE` keeps the text form at any address. The configuration file names no description form.

The configuration file may name more backends under `backends`, each with `url`, `key_env`, and `model`, and may name the default under `backend`; [recording.md](recording.md) lists the file's fields. An added entry may not reuse a built-in name. `key_env` matches `[A-Z_][A-Z0-9_]*`, and the file never holds a key. A file another user can write may not hold `backends`, because an entry could name any variable; that refusal exits 5. A backend name uses 1 to 32 lowercase ASCII letters, digits, and hyphens.

Any entry may set `requests_per_minute`, a whole number from 1 to 60,000, and the selected backend's rate paces its posting address (ruling 14, ticket 0343). A built-in's name may appear under `backends` holding only `requests_per_minute`; that paces the built-in and keeps its base, key variables, and model. No backend has a rate of its own, so nothing is paced unless the file or `THINKTHEN_REQUESTS_PER_MINUTE` sets a rate. The variable outranks the file. With no backend named, a run whose posting address is a built-in's own base takes that built-in's rate, so `"typesafe": {"requests_per_minute": 1000}` paces a run that names nothing, and an unnamed run whose `THINKTHEN_BASE_URL` is Liquid's address takes the `liquid` rate. An added entry's rate applies only when a tier names that entry. The limit holds within one process: separate processes each get the full rate, so N processes can send N times it. A refused rate exits 5 with `configuration backend field `requests_per_minute` must be a whole number from 1 to 60000`; a built-in's entry holding anything else exits 5 with `a configuration entry for a built-in backend holds `requests_per_minute` and nothing else`. Neither repeats a value.

```json
{
  "schema": "thinkthen.config/1",
  "backend": "liquid",
  "backends": {
    "liquid": {"requests_per_minute": 1000},
    "local-d1": {"url": "http://127.0.0.1:8080/v1", "key_env": "LOCAL_D1_KEY", "model": "d1:free", "requests_per_minute": 120}
  }
}
```

`--backend NAME`, `EngineBuilder::backend`, `THINKTHEN_BACKEND`, and the configuration's `backend` name a backend. The tiers run typed (`--backend`, `--url`), then the engine setting (`EngineBuilder::backend`, `EngineBuilder::base_url`), then the environment (`THINKTHEN_BACKEND`, `THINKTHEN_BASE_URL`), then the configuration file (`backend`, `url`). The first tier that names a backend or an address decides, and lower tiers are ignored:

- A backend and no address: the backend's base, key variables, and model apply.
- An address and no backend: the unnamed path applies exactly as before, with `THINKTHEN_API_KEY`.
- Both: the backend's key variables, model, and description form apply at that tier's address. `--backend liquid --url http://127.0.0.1:8080/v1` sends the Liquid key to the loopback server.
- A lower tier's address never replaces a higher tier's backend, and a higher tier's address outranks a lower tier's backend. `THINKTHEN_BASE_URL` in the shell outranks `backend` in the configuration file.

On the named path the model is `--model`, then the question file's `model`, then the engine's `model`, then the backend's model. The configuration's top-level `model` and `jev-1.13.0` apply only on the unnamed path.

When the selected backend reads a variable a built-in lists, and the final posting URL's host equals another built-in's host, the call is refused with exit 2 before any key is read, any cache or recording opens, or any connection is made. The host comparison is exact after lower-casing, decoding ASCII percent escapes, and dropping trailing dots, and it ignores port and path; a subdomain does not match. The whole standard error line reads, for example, `thinkthen: backend `typesafe` reads `TYPESAFE_API_KEY`, the key of backend `typesafe`, which never goes to the address of backend `liquid``. An unknown host passes, because the user named it. A built-in whose base host is loopback, such as `ollama`, is no other built-in's host, so `--backend liquid --url http://localhost:8080/v1` passes (ADR 0115). `OLLAMA_API_KEY` is still refused at the `typesafe` and `liquid` hosts. An explicit `EngineBuilder::api_key` reads no variable and skips this rule.

An invalid name exits 2 with `a backend name uses 1 to 32 lowercase letters, digits, and hyphens`. An unknown valid name exits 2 with `unknown backend `NAME`; the built-in backends are `liquid`, `ollama` and `typesafe`, and the configuration file may name more`. `Engine::builder()` knows only the built-ins and captures no key, so its caller supplies one with `api_key`; a missing key fails the first live call as on the unnamed path. The cache key never holds a key or a backend name, so two backends at one posting URL and model share answers, and backends at different URLs never mix. It hashes each question as sent, so an `ollama` question whose descriptions became text reuses no answer to the authored question, and a question whose descriptions are all strings is the same question under either form.

## The address

Settled by ADR 0010.

The address comes from `--url`, then `THINKTHEN_BASE_URL`, then the configuration file's `url`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

A base under plain `http://` reaches `localhost`, `127.0.0.1`, and `[::1]` and no other host. Any other host under `http://` is a usage error before any request, and the message says that the key would cross the network in clear text. No option overrides it. The rule reads the text of the host and resolves no name, so `localhost.`, `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are all refused. A backend on another machine is reached over `https://`, or over a tunnel whose near end is loopback. The clear-text restriction allows every host under `https://`.

A proxy carries an `https://` request and never an `http://` one. The HTTP client reads the proxy variables `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and `NO_PROXY`, each in upper and in lower case. Under `https://` a proxy those variables name carries the request, and it sees the host alone because the body travels inside TLS. Under `http://` every one of those variables is cancelled. A proxy would otherwise send the key and the evidence to another machine in clear text, which is the thing the loopback rule above refuses.

By default, HTTPS verifies the certificate chain and hostname against the bundled Mozilla roots. A process can set `THINKTHEN_CA_BUNDLE` to an absolute local PEM file to **replace** those roots for engines it builds from the environment; a Rust caller can use `EngineBuilder::ca_bundle(path)` on a bare or environment-built builder, with the explicit setter winning. `SSL_CERT_FILE` is not read. A bundle is at most 2 MiB and holds 1 to 256 matched `CERTIFICATE` blocks with only ASCII whitespace between them. An unreadable file is a local error; a relative path, wrong PEM label, malformed block, empty file, oversize file, or too many certificates is a usage error. Errors name the setting and cause, not the path or certificate contents. The selected roots are parsed once before the engine can send and are retained across model changes and fork reconstruction; a new engine reads a changed file. A named bundle is checked even for an engine that will only replay saved answers. The CLI still checks the resolved URL against its one optional key snapshot first; root validation follows before transport key use, an Authorization header, or a send. `status` builds no HTTP client and need not open the bundle. Command plan paths that build no engine also do not open it. The grouped annotate plan builds a preview engine and validates a named CA bundle before printing. An invalid or untrusted certificate never falls back to Mozilla roots or disables verification. Native libraries and SQL hosts inherit only the process environment source in this first setting; they have no per-instance or SQL spelling for the CA path.

`--url` names a base. It is the command-line spelling of `THINKTHEN_BASE_URL`, it outranks the variable, and the key rule above does not change when it is given. Beside `--backend`, it sends that backend's key to the named address. ADR 0031 puts it in short help because the address determines whether a first request reaches the default hosted service.

Space around a base is dropped. A scheme is read without regard to case and written back in lower case. A base has a host; a hostless base is a usage error reported as `a base address has a host`. Literal ASCII letters in an unbracketed host are also written in lower case, so equivalent DNS host-case spellings keep one resolved URL and recording digest. Punycode labels follow that ASCII rule. Percent escapes in the host, non-ASCII host bytes, bracketed IPv6 text, port spelling, and path bytes keep the case the caller typed. The parser does no percent decoding, Unicode case folding, IDNA conversion, IPv6 canonicalization, port normalization, or path normalization. A port is digits naming a number from 0 to 65535, or there is no colon at all. An empty port, a signed number, and a number past 65535 are each a usage error, because a port a socket cannot carry would be dropped and the request would go somewhere the caller did not name. A base carrying user information, a query, or a fragment is a usage error, because the address is printed in a plan and kept in a recording. The refusal message names the rule and never the base it refused. A base that is empty or holds only white space is a usage error too. The path is kept as typed, and a plan, every stored answer, and every question key hold it. A path token other than the configured key is written into every recording; the tool cannot infer which arbitrary segment is sensitive. An exact configured-key collision is refused before the path reaches a plan, digest, or folder. Keep a secret in `THINKTHEN_API_KEY` and never in the address.

The four address rules have these exact safe refusals. An invalid scheme says ``a base address begins with `http://` or `https://` ``. User information says `a base address carries no user information`. An empty, signed, or out-of-range port says `a port is digits naming a number from 0 to 65535`. A query or fragment says `a base address carries no query or fragment`. Each refusal exits 2 before key access or a request, and no output repeats the refused base.

`--plan` inspects the optional configured key to reject a URL collision before printing or hashing the address. It does not require a key, make an Authorization header, or open a connection. A safe plan keeps its exact URL and digest.

## The model

`--model NAME` names the model the request carries, and it defaults to `jev-1.13.0`. That is how a run is pinned to one version. The default is a pinned version, so a vendor's move of its alias moves no default answer. A later release that changes the default says so in the changelog, and every default cache answer then misses once. Space around a model name is dropped, as it is around a base. A model name that is empty or holds only white space is a usage error, and so is one holding a control character or any white space but a plain space, such as a line break, which says `a model name holds no control character or white space but a plain space`. The same rule reads the configuration file's `model` and a question file's `model`.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`. A command reads the key before it sends. With the key unset or blank, a base whose host is `localhost`, `127.0.0.1`, or `[::1]` gets the request with no `Authorization` header, so a local server that checks no key needs no pretend secret. Any other address exits 4 before any connection. ADR 0010's amendment of 2026-09-27 records the rule.

`--timeout SECONDS` covers one attempt from connect to the last byte, takes a whole number from 1 to 86400, and defaults to 30. Zero or a number past 86400 exits 2 with `--timeout takes a whole number of seconds from 1 to 86400`, before key access, input access, or a connection. The bound is one day, because the HTTP client adds the timeout to the clock and a larger number can overflow there. Every library takes the same bound: its timeout setting refuses a value past 86400 seconds with the usage error `a timeout is at most 86400 seconds`, before the engine exists. `--max-retries N` bounds how many times a retried status is sent again, and it defaults to 3.

A retry happens after a status of 429, 500, 502, 503, 504, 520, 521, 522, 523, 524, or 529. A transport failure is never sent again, because the backend may already hold the request and may bill it. Every such status closes one process gate for the exact posting URL before its send slot is freed. First sends and retries to that URL wait without a slot; cache and replay answers do not wait. The throttle stays at its chosen width and each request keeps its own retry count. Without a header the wait doubles from one second and is capped by 60 seconds and the caller's attempt timeout. Each such wait is drawn at random between half and all of that amount, so separate processes do not resend together; within one process the URL's gate still opens once for all waiting requests. A valid `Retry-After-Ms` header in whole milliseconds, or delta-seconds `Retry-After`, is a minimum wait instead; zero waits at least one second. The milliseconds header takes precedence. A header wait longer than the same cap, the lesser of 60 seconds and the attempt timeout, ends the retries at once: the request fails with the refused status, as it would after its last retry, and waits for nothing. The gate still holds that address until the server's time, so every later request to it in the process fails the same way without sending until then. A long-lived process, such as a database connection or a library inside a server, keeps failing that address for the whole wait; restart it to try sooner. A wait too long to represent never ends. Ticket 0367 made this rule so that every request has a fixed worst case; Ian can overturn it. Cancellation or a whole-call deadline can end the wait without another send. The HTTP-date form is ignored. A request whose retries are spent still closes the gate. Other failures do not close it.

On `decide`, `filter`, `rank`, `choose`, `tag` and `score`, `--context FILE` makes one text the shared evidence of each record batch. The tool checks its exact request bytes against the resolved `--max-request-bytes` setting and any lower profile limit at every address. It refuses an oversized context and question before a request, or an oversized later record when that record arrives, under ADR 0087.

After the last allowed attempt, a refused status exits 4; with `--max-retries 0`, that is the first attempt. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code. For connection reuse, the client drains at most 1 MiB of each error body and retains none of it. On status 400 alone, the command examines at most 4 KiB of that body and only its `detail.error_type`. When that value is exactly `max_tokens_exceeded`, the message names it. Any other body, and any body it cannot read or parse, gives the plain 400 phrase. The public API keeps `the backend answered with status 400`; status 302 also names that its redirect was not followed. The SQL surfaces print the bare status line for every status, with no phrase.

| Status | Phrase |
| --- | --- |
| 400 | the backend refused the request; check `--model` and the request size |
| 400, body naming `max_tokens_exceeded` | the request has more input tokens than the backend takes; shorten the text, or set a lower `--max-request-bytes` or `max_request_bytes` with `--profile`. The status reads `400 (max_tokens_exceeded)` |
| 302 | the redirect was not followed; use the final `--url` directly |
| 401 | the key was refused |
| 402 | the account has no credit |
| 403 | the key may not use this model or address |
| 404 | nothing answers at this address |
| 413 | the backend refused the request as too large; shorten the text, or set a lower `--max-request-bytes` or `max_request_bytes` with `--profile` |
| 422 | the backend refused the request as malformed or too large |
| 429 after the allowed attempts | the backend's rate limit was reached after the allowed attempts; try again later or change `--max-retries` |
| 500, 502, 503, 504, 520, 521, 522, 523, 524, or 529 after the allowed attempts | the backend failed after the allowed attempts; try again later or change `--max-retries` |

A connection failure is reduced from the HTTP client's structured error before it reaches the command. The command prints fixed guidance and never the client text, operating-system text, address, key, evidence, or response body. A timeout says to increase `--timeout` or try again. For a request holding two or more records, the stopped-run line names the records that request held. A missing host says to check `--url` and the network. Every transport failure, a refused connection included, fails after its first attempt. A refused connection says to check that the backend is running and that `--url` is correct. A connection that closes or resets before a reply, or cuts its reply short, says the backend may have received the request and that it was not sent again. A TLS connection or certificate failure names certificate trust and advises checking `--url`. Every other transport failure says to check `--url` and the network.

A reply may hold at most 1 MiB plus 8 bytes for each byte of its request. A reply echoes what its request named, and its worst honest shape runs about five times its request, as ticket 0132, decision 2, works out. The limit stops a backend that never stops writing. A longer reply is exit code 4 and is never sent again. The message names the limit in bytes: `the backend's reply passed this request's limit of N bytes, so the answer was not kept; the request was not sent again`. The library keeps the same sentence.

Status 402 was seen live on 2026-09-19, on an account with no credit left. The vendor's own pages list no 402 anywhere. `sdlc/records/0003-live-call.md` holds the calls that met it.

## The adapter contract

An adapter owns its name, its default address, its default model, and its endpoint path, and nothing outside its own module names any of the four.

An adapter is two pure functions.

- **encode** takes a plan and returns the request body as bytes. A plan holds the evidence, the model name, and an ordered list of named questions.
- **decode** takes the response body as bytes and returns the model that answered, one answer per named question, and the usage the backend reported.

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Version one compiles one adapter, `systemone`, and nothing selects it. Ruling 1 of ADR 0010 left one wire shape, so there is nothing to choose between. The adapter name `systemone` still leads every question key, so an answer stored today names the shape it was asked in.

A reply that is not a `systemone` response at all is exit code 4, and the message names the line and column the reading stopped at and never the text it stopped on. A backend can send back whatever was sent to it, so a diagnostic never repeats a reply.

A reply that names one member twice, in `answers` or inside a distribution, is refused whole, because two readers could take different values from it.

Decode marks one logical question failed when its answer is missing, has the wrong kind, lacks a probability, holds a probability outside zero to one, has an invalid distribution, or names an unexpected probability. It preserves those failures only when another logical question in the reply is valid. One bad wire member fails one logical `tag`. A reply with no valid logical answer remains a refused reply at exit 4. An adapter never invents a probability. A choice or score answer carries exactly one probability for every label the question sent, and no probability for another label. The generic tolerance is `member count × f64::EPSILON`; an adapter may supply a wider tolerance backed by evidence. The tool keeps the reported members without renormalizing them.

## The `systemone` adapter

The first vendor's format. The request body:

```json
{"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this convey urgency?"}}}
```

The response body:

```json
{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}
```

| thinkthen | `systemone` |
| --- | --- |
| The evidence | `state`: the text as a string, or the object or list a pointer selection made, as that JSON value |
| A yes/no question and its text | `type` `noul`, with the question as `instructions`, as the string, object, or list the question held |
| What true means and what false means | `criteria.true` and `criteria.false` under the `noul` question carry only non-null string, object, or list descriptions. An absent or explicitly null description sends no member; if neither side has a description, the question sends no `criteria` at all. The parsed question and its canonical digest retain an explicit null |
| A yes/no answer's probability | `noul` |
| Pick one from a list | `type` `choice`, with the options as the keys of `criteria` and each description as the value, or `null` |
| Place on named levels | `type` `score`, with the `criteria` array in level order: a level from a list of names sends its name, a described level sends the description the map held, and a `null` description sends an empty object in its place. The name never stands in for a `null` |
| A choice answer's probability per option | `probabilities` under the `choice` answer, keyed by option name. The key order carries no meaning, and the adapter rebuilds the distribution in the order the options were sent |
| A score answer's probability per level | `probabilities` under the `score` answer, keyed by the level's position as a string, counting from `"0"`. The adapter maps each key back to the configured level name |
| The backend's own confidence | `confidence`, kept and never cut on. The vendor sends it on a pick and a placement, and never on a yes/no answer |
| A score's number | Computed locally from the level probabilities. Nothing is read from the wire |
| Several questions over one evidence | One `questions` map with one entry per question. One request, one `state` |

Question names are `q1`, `q2`, and onward in plan order. The vendor does not show names to the model.

The names carry the order. The order of keys inside the `questions` object and the `answers` object carries no meaning. Decode refuses an answer name the plan lacks, even beside a valid planned answer. Decode refuses a response whose `model` is absent or blank, because a result must name the model that answered.

For a choice or score answer, System One must return exactly the option or level keys the request sent. Missing keys are refused before the adapter reads any member, and extra keys are refused. Live evidence found System One distributions one decimal hundredth short, so its adapter accepts a total within `0.01 + member count × f64::EPSILON` of one. The epsilon term keeps decimal totals of `0.99` and `1.01` inside the inclusive boundary after binary parsing and addition. Totals of `0.98` and `1.02` are refused. Every accepted member remains as reported.

The vendor also sends `choice`, `score`, and `legend` beside the probabilities. Each one is derivable from the distribution and the question that was asked, so the adapter computes them and reads none of them. The fixtures under `fixtures/systemone/` hold a real response of each kind.

### What the adapter keeps

Settled by ADR 0009 item 2, accepted in ADR 0010. The adapter keeps the full distribution and the vendor's `confidence` field. Both reach `answer` in the result, as [result.md](result.md) describes. A saved run can then be swept at another rule with no second request.

The cut on `choose` stays on the winning option's probability. That number exists on every backend, and a reader can say what it means. Most of the vendor's own pages cut on `confidence` instead, and the formula behind `confidence` is unpublished. A live sweep of both against labels settles whether the rule changes.
