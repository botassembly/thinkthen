# Backends

Status: **Settled**, amended by ADR 0048, for the wire shape, the key, the address, the request, the `systemone` adapter, and every adapter row.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

A backend is an address and a model. `THINKTHEN_BASE_URL` or the configuration file's `url` names the address, `THINKTHEN_API_KEY` holds the key, and `--model` names the model. An explicit profile may add a stable name and local limits. It never selects either value.

## Explicit profiles and local preflight

Settled by ADR 0032 and amended by ADR 0040.

`--profile FILE` reads one UTF-8 JSON object with schema `thinkthen.backend-profile/1`. It requires `name` and at least one of `max_evidence_bytes`, `max_request_bytes`, `max_questions`, or `max_options`. The name is nonempty and uses lowercase letters, digits, hyphens, and underscores. Each limit is a positive integer. No other key is accepted.

Evidence bytes are the UTF-8 bytes after field extraction and JSON normalization. Request bytes are the exact bytes the adapter encoded. Questions are the expanded wire questions, so each `tag` label counts once. Options are the labels in one `choose` question. An exact limit passes. One unit over exits 2 and names the profile, limit, unit, and actual count. The diagnostic repeats no evidence or request bytes.

Recognition's step-1 and step-2 requests carry a window of the text: six pieces on each side of the pieces or names they ask about, as [recognize.md](recognize.md) says. A step-1 request holds at most 40 pieces. Relation requests carry the whole text and split their questions through the shared request splitter. `max_questions` and `max_request_bytes` can create several logical requests without changing offsets or assembly. In `relate`, a beta cross-kind relation choice that exceeds `max_options` or cannot fit alone under `max_request_bytes` changes that whole relation to yes/no pairs. `max_evidence_bytes` never causes that fallback.

The engine encodes, digests, and checks every request chunk before reading a recording or cache entry, reading the key, or opening a connection. An ordered multi-question plan takes the longest next contiguous prefix that fits both exact request bytes and expanded questions, then repeats until all questions belong to a chunk. The splitter finds that prefix by doubling the chunk from one question up to the remainder, then halving the gap. Evidence is repeated unchanged. A plan that fits keeps its exact body and digest. Evidence, one-question request, and choice-option overflows fail before any request because splitting them would change meaning.

A relation plan at the built-in address also splits under a built-in ceiling of 96,000 request bytes. The hosted backend refuses a request over 65,536 input tokens with status 400. On 2026-09-24, 142 relate `appears_on` questions over the Beatles set passed at 65,423 input tokens, and 143 failed. That 142-question request encodes to 126,820 bytes, so relate's JSON runs 0.516 input tokens a byte. 96,000 bytes then comes to about 49,500 tokens. The ceiling reaches `relate` and the relation step of `recognize`. It applies when the resolved posting URL equals `https://api.typesafe.ai/v1/systemone` byte for byte. A chunk of two or more questions stays at most 96,000 bytes. One question alone always passes it, so the ceiling splits and never refuses. It never turns a choice into yes/no questions. A profile's `max_request_bytes` replaces it, and a profile's other limits apply beside it. Every other plan and every other address has no ceiling. Ticket 0123 set it. A batched record plan at the built-in address also closes its batch at the ceiling. A batch of `decide`, `filter` or `rank` carries each record once, inside its quoted question, beside the 44-byte evidence sentence of ADR 0055. Records fill a batch until the next one would pass 96,000 request bytes.

A profile's limits close batches at every address. A retried status resends the whole batch as one request, and a batch is never split and resent. A backend that refuses a batch as too large fails it at exit 4.

`annotate` checks every chunk of every group for one record before starting any group. Dry runs perform the same checks before printing a plan. Profiles apply equally to live calls, replay, cache, and recording. The profile estimates no tokens. A backend whose published limit is only tokens needs a tokenizer or a verified byte ceiling before its file can enforce the limit. `--url`, `THINKTHEN_BASE_URL`, the configuration file's `url`, and `--model` still select the backend. A profile contains none of them.

## The key

Settled by ADR 0010.

The key is read from `THINKTHEN_API_KEY`. No option names another variable.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.
- **The key goes to the address the user named.** That is the whole rule. Naming an address is the user's own act, so a run pointed at another address carries the key of `THINKTHEN_API_KEY` and nothing else.

## The address

Settled by ADR 0010.

The address comes from `--url`, then `THINKTHEN_BASE_URL`, then the configuration file's `url`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

A base under plain `http://` reaches `localhost`, `127.0.0.1`, and `[::1]` and no other host. Any other host under `http://` is a usage error before any request, and the message says that the key would cross the network in clear text. No option overrides it. The rule reads the text of the host and resolves no name, so `localhost.`, `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are all refused. A backend on another machine is reached over `https://`, or over a tunnel whose near end is loopback. The clear-text restriction allows every host under `https://`.

A proxy carries an `https://` request and never an `http://` one. The HTTP client reads the proxy variables `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and `NO_PROXY`, each in upper and in lower case. Under `https://` a proxy those variables name carries the request, and it sees the host alone because the body travels inside TLS. Under `http://` every one of those variables is cancelled. A proxy would otherwise send the key and the evidence to another machine in clear text, which is the thing the loopback rule above refuses.

`--url` names a base and takes no companion option. It is the command-line spelling of `THINKTHEN_BASE_URL`, it outranks the variable, and the key rule above does not change when it is given. ADR 0031 puts it in short help because the address determines whether a first request reaches the default hosted service.

Space around a base is dropped. A scheme is read without regard to case and written back in lower case. A base has a host; a hostless base is a usage error reported as `a base address has a host`. Literal ASCII letters in an unbracketed host are also written in lower case, so equivalent DNS host-case spellings keep one resolved URL and recording digest. Punycode labels follow that ASCII rule. Percent escapes in the host, non-ASCII host bytes, bracketed IPv6 text, port spelling, and path bytes keep the case the caller typed. The parser does no percent decoding, Unicode case folding, IDNA conversion, IPv6 canonicalization, port normalization, or path normalization. A port is digits naming a number from 0 to 65535, or there is no colon at all. An empty port, a signed number, and a number past 65535 are each a usage error, because a port a socket cannot carry would be dropped and the request would go somewhere the caller did not name. A base carrying user information, a query, or a fragment is a usage error, because the address is printed in a plan and kept in a recording. The refusal message names the rule and never the base it refused. A base that is empty or holds only white space is a usage error too.

The four address rules have these exact safe refusals. An invalid scheme says ``a base address begins with `http://` or `https://` ``. User information says `a base address carries no user information`. An empty, signed, or out-of-range port says `a port is digits naming a number from 0 to 65535`. A query or fragment says `a base address carries no query or fragment`. Each refusal exits 2 before key access or a request, and no output repeats the refused base.

`--dry-run` shows the address the run would use, and it reads no key.

## The model

`--model NAME` names the model the request carries, and it defaults to `jev-1.13.0`. That is how a run is pinned to one version. The default is a pinned version, so a vendor's move of its alias moves no default answer. A later release that changes the default says so in the changelog, and every default cache entry then misses once. A model name that is empty or holds only white space is a usage error.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`.

`--timeout SECONDS` covers one attempt from connect to the last byte, takes a whole number greater than zero, and defaults to 30. Zero exits 2 before key access, input access, or a connection. `--max-retries N` bounds how many times a retried status is sent again, and it defaults to 2.

A retry happens after a status of 429, 500, 502, 503, 504, or 529. A transport failure is never sent again, because the backend may already hold the request and may bill it. The wait doubles from one second, and no wait follows the last attempt. No retry wait exceeds `--timeout`. A reply carrying a `Retry-After-Ms` header in whole milliseconds or a `Retry-After` header in the delta-seconds form waits the time it names instead, capped by both 60 seconds and `--timeout`. The milliseconds header is read first, because the backend sends the finer number there. The HTTP-date form of `Retry-After` is ignored, because reading it needs a clock and a date reader. Any other error status fails at once.

A failure after the last retry is exit code 4. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code. On status 400 alone, the command reads at most 4 KiB of the body and only its `detail.error_type`. When that value is exactly `max_tokens_exceeded`, the message names it. Any other body, and any body it cannot read or parse, gives the plain 400 phrase. The library keeps `the backend answered with status 400`. The libraries and SQL surfaces print the bare status line for every status, with no phrase.

| Status | Phrase |
| --- | --- |
| 400 | the backend refused the request; check `--model` and the request size |
| 400, body naming `max_tokens_exceeded` | the request has more input tokens than the backend takes; shorten the text or set a lower max_request_bytes with `--profile`. The status reads `400 (max_tokens_exceeded)` |
| 401 | the key was refused |
| 402 | the account has no credit |
| 403 | the key may not use this model or address |
| 404 | nothing answers at this address |
| 422 | the backend refused the request as malformed or too large |
| 429 | the backend's rate limit was reached |
| 500, 502, 503, 504, or 529 after the allowed attempts | the backend failed after the allowed attempts; try again later or change `--max-retries` |

A connection failure is reduced from the HTTP client's structured error before it reaches the command. The command prints fixed guidance and never the client text, operating-system text, address, key, evidence, or response body. A timeout says to increase `--timeout` or try again. A missing host says to check `--url` and the network. Every transport failure, a refused connection included, fails after its first attempt. A refused connection says to check that the backend is running and that `--url` is correct. A connection that closes or resets before a reply, or cuts its reply short, says the backend may have received the request and that it was not sent again. Every other transport failure says to check `--url` and the network.

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
| What true means and what false means | `criteria.true` and `criteria.false` under the `noul` question, each the string, object, list, or `null` the question held. A text that was not given is absent, and a question with neither sends no `criteria` at all |
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
