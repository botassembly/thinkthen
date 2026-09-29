# Backends

Status: **Settled**, amended by ADR 0048, for the wire shape, the key, the address, the request, the `systemone` adapter, and every adapter row.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

A backend is an address and a model. `THINKTHEN_BASE_URL` or the configuration file's `url` names the address, `THINKTHEN_API_KEY` holds the key, and `--model` names the model. An explicit profile may add a stable name and local limits. It never selects either value.

## Explicit profiles and local preflight

Settled by ADR 0032 and amended by ADR 0040.

`--profile FILE` reads one UTF-8 JSON object with schema `thinkthen.backend-profile/1`. It requires `name` and at least one of `max_evidence_bytes`, `max_request_bytes`, `max_questions`, or `max_options`. The name is nonempty and uses lowercase letters, digits, hyphens, and underscores. Each limit is a positive integer. No other key is accepted.

Evidence bytes are the UTF-8 bytes after field extraction and JSON normalization. Request bytes are the exact bytes the adapter encoded. Questions are the expanded wire questions, so each `tag` label counts once. Options are the labels in one `choose` question. An exact limit passes. One unit over exits 2 and names the profile, limit, unit, and actual count. The diagnostic repeats no evidence or request bytes.

Recognition's step-1 and step-2 requests carry a window of the text: six pieces on each side of the pieces or names they ask about, as [recognize.md](recognize.md) says. A step-1 request holds at most 40 pieces. Relation requests carry the whole text and split their questions through the shared request splitter. `max_questions` and `max_request_bytes` can create several logical requests without changing offsets or assembly. In `relate`, every relation question is yes/no. Its requests hold at most 400 questions at every address, and a profile's `max_questions` can lower that bound. `max_options` does not affect relation questions.

The command checks an optional snapshot of the effective key against the resolved address before encoding, digesting, reading a recording or cache entry, or opening a connection. The engine then encodes, digests, and checks every request chunk before reading a recording or cache entry or opening a connection. An ordered multi-question plan takes the longest next contiguous prefix that fits both exact request bytes and expanded questions, then repeats until all questions belong to a chunk. The splitter finds that prefix by doubling the chunk from one question up to the remainder, then halving the gap. Evidence is repeated unchanged. A plan that fits keeps its exact body and digest. Evidence, one-question request, and choice-option overflows fail before any request because splitting them would change meaning.

A request-size setting closes batched record requests and splits relation plans at every address. It defaults to 96,000 bytes. `--max-request-bytes N` beats `THINKTHEN_MAX_REQUEST_BYTES`, which beats the default, on `decide`, `filter`, `rank`, `choose`, `tag`, `score`, `annotate`, `relate` and `recognize`. The flag and variable take a whole number of at least 1. A profile's `max_request_bytes` lowers the setting and never raises it; its other limits apply beside both. One question or record alone passes the setting and goes as one request. A profile still refuses a request that cannot split under its own limit. The built-in backend refuses a request over 65,536 input tokens with status 400. Ticket 0123 measured 0.516 input tokens per byte for former choice-based relate bodies. Pair-question token density remains unmeasured until ticket 0167's recorded run. A size above 96,000 at the built-in address prints one warning before planning or sending, including under `--plan`. A batch of `decide`, `filter` or `rank` carries each record once in its quoted question beside ADR 0055's fixed evidence sentence. Its next record closes the batch when adding it would pass the smaller limit. Every other plan keeps its existing limit rules. ADR 0051 sets the request size's reach.

A retried status resends the whole batch as one request. When a backend refuses a batch of two or more records with status 413 or status 400 naming `max_tokens_exceeded`, the command asks its first half, then its second half, once each under the same `--jobs` place. A half refused again fails at exit 4 and is not split again. Plain 400, 422 and other failures do not split. Under replay, a missing whole batch asks its halves from disk before it counts as missing. Recording and cache store the successful halves as ordinary requests. A refused whole request writes no entry. Its attempt counts in the first half's rows. The halves' digests appear in their rows; ticket B5 adds the explicit `meta.batch.split` mark.

`annotate` checks all group selections and each singleton profile chunk before admitting a record to a batch. Each compatible group slice then checks its exact packed body and expanded wire-question count before sending. A later refused record cannot cause its own group to send. Dry runs perform the same checks before printing a plan. Profiles apply equally to live calls, replay, cache, and recording. The profile estimates no tokens. A backend whose published limit is only tokens needs a tokenizer or a verified byte ceiling before its file can enforce the limit. `--url`, `THINKTHEN_BASE_URL`, the configuration file's `url`, and `--model` still select the backend. A profile contains none of them.

## The key

Settled by ADR 0010.

The key is read from `THINKTHEN_API_KEY`. No option names another variable.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.
- A key with a carriage return or line feed is a usage error before any send. The fixed message repeats no part of the key.
- **The key goes to the address the user named.** A nonblank effective key that occurs verbatim anywhere in the resolved posting URL is refused first with `the backend address contains the API key; keep the key out of the address`. The check compares exact UTF-8 bytes without decoding or rewriting the URL. Otherwise a run pointed at another address carries the key of `THINKTHEN_API_KEY` and nothing else.

## The address

Settled by ADR 0010.

The address comes from `--url`, then `THINKTHEN_BASE_URL`, then the configuration file's `url`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

A base under plain `http://` reaches `localhost`, `127.0.0.1`, and `[::1]` and no other host. Any other host under `http://` is a usage error before any request, and the message says that the key would cross the network in clear text. No option overrides it. The rule reads the text of the host and resolves no name, so `localhost.`, `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are all refused. A backend on another machine is reached over `https://`, or over a tunnel whose near end is loopback. The clear-text restriction allows every host under `https://`.

A proxy carries an `https://` request and never an `http://` one. The HTTP client reads the proxy variables `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and `NO_PROXY`, each in upper and in lower case. Under `https://` a proxy those variables name carries the request, and it sees the host alone because the body travels inside TLS. Under `http://` every one of those variables is cancelled. A proxy would otherwise send the key and the evidence to another machine in clear text, which is the thing the loopback rule above refuses.

By default, HTTPS verifies the certificate chain and hostname against the bundled Mozilla roots. A process can set `THINKTHEN_CA_BUNDLE` to an absolute local PEM file to **replace** those roots for engines it builds from the environment; a Rust caller can use `EngineBuilder::ca_bundle(path)` on a bare or environment-built builder, with the explicit setter winning. `SSL_CERT_FILE` is not read. A bundle is at most 2 MiB and holds 1 to 256 matched `CERTIFICATE` blocks with only ASCII whitespace between them. An unreadable file is a local error; a relative path, wrong PEM label, malformed block, empty file, oversize file, or too many certificates is a usage error. Errors name the setting and cause, not the path or certificate contents. The selected roots are parsed once before the engine can send and are retained across model changes and fork reconstruction; a new engine reads a changed file. A named bundle is checked even for an engine that will only replay saved answers. The CLI still checks the resolved URL against its one optional key snapshot first; root validation follows before transport key use, an Authorization header, or a send. `status` builds no HTTP client and need not open the bundle. Command plan paths that build no engine also do not open it. The grouped annotate plan builds a preview engine and validates a named CA bundle before printing. An invalid or untrusted certificate never falls back to Mozilla roots or disables verification. Native libraries and SQL hosts inherit only the process environment source in this first setting; they have no per-instance or SQL spelling for the CA path.

`--url` names a base and takes no companion option. It is the command-line spelling of `THINKTHEN_BASE_URL`, it outranks the variable, and the key rule above does not change when it is given. ADR 0031 puts it in short help because the address determines whether a first request reaches the default hosted service.

Space around a base is dropped. A scheme is read without regard to case and written back in lower case. A base has a host; a hostless base is a usage error reported as `a base address has a host`. Literal ASCII letters in an unbracketed host are also written in lower case, so equivalent DNS host-case spellings keep one resolved URL and recording digest. Punycode labels follow that ASCII rule. Percent escapes in the host, non-ASCII host bytes, bracketed IPv6 text, port spelling, and path bytes keep the case the caller typed. The parser does no percent decoding, Unicode case folding, IDNA conversion, IPv6 canonicalization, port normalization, or path normalization. A port is digits naming a number from 0 to 65535, or there is no colon at all. An empty port, a signed number, and a number past 65535 are each a usage error, because a port a socket cannot carry would be dropped and the request would go somewhere the caller did not name. A base carrying user information, a query, or a fragment is a usage error, because the address is printed in a plan and kept in a recording. The refusal message names the rule and never the base it refused. A base that is empty or holds only white space is a usage error too. The path is kept as typed, and a plan, every recording entry, and the cache folder's identity all hold it. A path token other than the configured key is written into every recording; the tool cannot infer which arbitrary segment is sensitive. An exact configured-key collision is refused before the path reaches a plan, digest, or folder. Keep a secret in `THINKTHEN_API_KEY` and never in the address.

The four address rules have these exact safe refusals. An invalid scheme says ``a base address begins with `http://` or `https://` ``. User information says `a base address carries no user information`. An empty, signed, or out-of-range port says `a port is digits naming a number from 0 to 65535`. A query or fragment says `a base address carries no query or fragment`. Each refusal exits 2 before key access or a request, and no output repeats the refused base.

`--plan` inspects the optional configured key to reject a URL collision before printing or hashing the address. It does not require a key, make an Authorization header, or open a connection. A safe plan keeps its exact URL and digest.

## The model

`--model NAME` names the model the request carries, and it defaults to `jev-1.13.0`. That is how a run is pinned to one version. The default is a pinned version, so a vendor's move of its alias moves no default answer. A later release that changes the default says so in the changelog, and every default cache entry then misses once. Space around a model name is dropped, as it is around a base. A model name that is empty or holds only white space is a usage error, and so is one holding a control character or any white space but a plain space, such as a line break, which says `a model name holds no control character or white space but a plain space`. The same rule reads the configuration file's `model` and a question file's `model`.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`. A command reads the key before it sends. With the key unset or blank, a base whose host is `localhost`, `127.0.0.1`, or `[::1]` gets the request with no `Authorization` header, so a local server that checks no key needs no pretend secret. Any other address exits 4 before any connection. ADR 0010's amendment of 2026-09-27 records the rule.

`--timeout SECONDS` covers one attempt from connect to the last byte, takes a whole number from 1 to 86400, and defaults to 30. Zero or a number past 86400 exits 2 with `--timeout takes a whole number of seconds from 1 to 86400`, before key access, input access, or a connection. The bound is one day, because the HTTP client adds the timeout to the clock and a larger number can overflow there. `--max-retries N` bounds how many times a retried status is sent again, and it defaults to 3.

A retry happens after a status of 429, 500, 502, 503, 504, 520, 521, 522, 523, 524, or 529. A transport failure is never sent again, because the backend may already hold the request and may bill it. Every such status closes one process gate for the exact posting URL before its send slot is freed. First sends and retries to that URL wait without a slot; cache and replay answers do not wait. The throttle stays at its chosen width and each request keeps its own retry count. Without a header the wait doubles from one second and is capped by 60 seconds and the caller's attempt timeout. A valid `Retry-After-Ms` header in whole milliseconds, or delta-seconds `Retry-After`, is a minimum wait instead; zero waits at least one second. The milliseconds header takes precedence. Cancellation or a whole-call deadline can end the wait without another send. The HTTP-date form is ignored. A request whose retries are spent still closes the gate. Other failures do not close it.

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

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Version one compiles one adapter, `systemone`, and nothing selects it. Ruling 1 of ADR 0010 left one wire shape, so there is nothing to choose between. A recording entry still names `systemone`, so an entry written today says which shape it recorded.

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
