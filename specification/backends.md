# Backends

Status: **Settled** for the wire shape, the key, the address, the request, the `systemone` adapter, and every adapter row.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

A backend is two values: the address and the model. `THINKTHEN_BASE_URL` names the address, `THINKTHEN_API_KEY` holds the key, and `--model` names the model. ADR 0010 took the configuration file and the four options that served it out of version one. [roadmap.md](roadmap.md) names each one, says what the file held, and says what would bring it back.

## The key

Settled by ADR 0010.

The key is read from `THINKTHEN_API_KEY`. No option names another variable.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.
- **The key goes to the address the user named.** That is the whole rule. Naming an address is the user's own act, so a run pointed at another address carries the key of `THINKTHEN_API_KEY` and nothing else.

## The address

Settled by ADR 0010.

The address comes from `--url`, then `THINKTHEN_BASE_URL`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

A base under plain `http://` reaches `localhost`, `127.0.0.1`, and `[::1]` and no other host. Any other host under `http://` is a usage error before any request, and the message says that the key would cross the network in clear text. No option overrides it. The rule reads the text of the host and resolves no name, so `localhost.`, `127.1`, `0.0.0.0`, and `[::ffff:127.0.0.1]` are all refused. A backend on another machine is reached over `https://`, or over a tunnel whose near end is loopback. The clear-text restriction allows every host under `https://`.

A proxy carries an `https://` request and never an `http://` one. The HTTP client reads the proxy variables `ALL_PROXY`, `HTTPS_PROXY`, `HTTP_PROXY`, and `NO_PROXY`, each in upper and in lower case. Under `https://` a proxy those variables name carries the request, and it sees the host alone because the body travels inside TLS. Under `http://` every one of those variables is cancelled. A proxy would otherwise send the key and the evidence to another machine in clear text, which is the thing the loopback rule above refuses.

`--url` names a base and takes no companion option. It is the command-line spelling of `THINKTHEN_BASE_URL`, it outranks the variable, and the key rule above does not change when it is given. ADR 0031 puts it in short help because the address determines whether a first request reaches the default hosted service.

Space around a base is dropped. A scheme is read without regard to case and written back in lower case. A base has a host; a hostless base is a usage error reported as `a base address has a host`. Literal ASCII letters in an unbracketed host are also written in lower case, so equivalent DNS host-case spellings keep one resolved URL and recording digest. Punycode labels follow that ASCII rule. Percent escapes in the host, non-ASCII host bytes, bracketed IPv6 text, port spelling, and path bytes keep the case the caller typed. The parser does no percent decoding, Unicode case folding, IDNA conversion, IPv6 canonicalization, port normalization, or path normalization. A port is digits naming a number from 0 to 65535, or there is no colon at all. An empty port, a signed number, and a number past 65535 are each a usage error, because a port a socket cannot carry would be dropped and the request would go somewhere the caller did not name. A base carrying user information, a query, or a fragment is a usage error, because the address is printed in a plan and kept in a recording. The refusal message names the rule and never the base it refused. A base that is empty or holds only white space is a usage error too.

The four address rules have these exact safe refusals. An invalid scheme says ``a base address begins with `http://` or `https://` ``. User information says `a base address carries no user information`. An empty, signed, or out-of-range port says `a port is digits naming a number from 0 to 65535`. A query or fragment says `a base address carries no query or fragment`. Each refusal exits 2 before key access or a request, and no output repeats the refused base.

`--dry-run` shows the address the run would use, and it reads no key.

## The model

`--model NAME` names the model the request carries, and it defaults to `jev-latest`. That is how a run is pinned to one version. A model name that is empty or holds only white space is a usage error.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`.

`--timeout SECONDS` covers one attempt from connect to the last byte, takes a whole number greater than zero, and defaults to 30. Zero exits 2 before key access, input access, or a connection. `--max-retries N` bounds the retries after the first attempt, and it defaults to 2.

A retry happens after a transport failure or a status of 429, 500, 502, 503, 504, or 529. The wait doubles from one second, and no wait follows the last attempt. A reply carrying a `Retry-After-Ms` header in whole milliseconds or a `Retry-After` header in the delta-seconds form waits the time it names instead, capped at 60 seconds. The milliseconds header is read first, because the backend sends the finer number there. The HTTP-date form of `Retry-After` is ignored, because reading it needs a clock and a date reader. Any other error status fails at once.

A failure after the last retry is exit code 4. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code.

| Status | Phrase |
| --- | --- |
| 400 | the backend refused the request; check `--model` and the request size |
| 401 | the key was refused |
| 402 | the account has no credit |
| 403 | the key may not use this model or address |
| 404 | nothing answers at this address |
| 422 | the backend refused the request as malformed or too large |
| 429 | the backend's rate limit was reached |
| 500 after the allowed attempts | the backend failed after the allowed attempts; try again later or change `--max-retries` |

A connection failure is reduced from the HTTP client's structured error before it reaches the command. The command prints fixed guidance and never the client text, operating-system text, address, key, evidence, or response body. A timeout says to increase `--timeout` or try again. A missing host says to check `--url` and the network. A refused connection says to check that the backend is running and that `--url` is correct. A connection that closes before a reply says to try again or change `--max-retries`. Every other transport failure says to check `--url` and the network.

Status 402 was seen live on 2026-09-19, on an account with no credit left. The vendor's own pages list no 402 anywhere. `sdlc/records/0003-live-call.md` holds the calls that met it.

## The adapter contract

An adapter owns its name, its default address, its default model, and its endpoint path, and nothing outside its own module names any of the four.

An adapter is two pure functions.

- **encode** takes a plan and returns the request body as bytes. A plan holds the evidence, the model name, and an ordered list of named questions.
- **decode** takes the response body as bytes and returns the model that answered, one answer per named question, and the usage the backend reported.

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Version one compiles one adapter, `systemone`, and nothing selects it. Ruling 1 of ADR 0010 left one wire shape, so there is nothing to choose between. A recording entry still names `systemone`, so an entry written today says which shape it recorded.

A reply that is not a `systemone` response at all is exit code 4, and the message names the line and column the reading stopped at and never the text it stopped on. A backend can send back whatever was sent to it, so a diagnostic never repeats a reply.

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
| The evidence | `state`, as one string |
| A yes/no question and its text | `type` `noul`, with the question as `instructions` |
| What true means and what false means | `criteria.true` and `criteria.false` under the `noul` question. A text that was not given is absent, and a question with neither sends no `criteria` at all |
| A yes/no answer's probability | `noul` |
| Pick one from a list | `type` `choice`, with the options as the keys of `criteria` and each description as the value, or `null` |
| Place on named levels | `type` `score`, with the levels as the `criteria` array |
| A choice answer's probability per option | `probabilities` under the `choice` answer, keyed by option name. The key order carries no meaning, and the adapter rebuilds the distribution in the order the options were sent |
| A score answer's probability per level | `probabilities` under the `score` answer, keyed by the level's position as a string, counting from `"0"`. The adapter maps each key back to the level text it sent |
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
