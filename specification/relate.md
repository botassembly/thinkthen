# Relate entities

Status: **Settled** for version one by tickets 0088, 0167, 0342 and 0353 and ADR 0057.

`relate` reads one complete entity set and prints the relationships that reach one cut. An edge comes from the model's knowledge of the names, not from any text; for edges a text states, use `recognize --relation`. It uses the shared relation planner from `recognize`, but its input and output contain names and kinds rather than text offsets.

```sh
thinkthen relate works_for=person:organization < entities.json
thinkthen relate @entities.json --jsonl < entities.jsonl
```

## Relations

The command takes one or more ordered inline rules. `NAME=SOURCE_KIND:TARGET_KIND` names a directed rule. Bare `NAME` means `NAME=*:*`. `ANY` on either side means `*`, in any ASCII case, and the plan and digest write `*`. `--either` makes every inline rule unordered. A rule name, source kind, and target kind are nonblank text without control characters. A line with another `=` or `:` is malformed.

The alternative form takes exactly one `@FILE`. The closed version-one file is:

```json
{"version":1,"relate":{"fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}]},"threshold":0.5,"model":"jev-1.13.0","profile":"measured"}
```

`relations` is required, ordered, nonempty, and has distinct names. A rule may leave out `source` or `target`, which means `*`. `reads` defaults to the relation name with underscores replaced by spaces. `either` defaults to false. `single` defaults to false; `"single":true` says each source has at most one target, and the rule asks one menu per source, as [Output](#output) says. Mark a single-answer relation, such as song to first album or city to country, `single`: that is the recommended form, for the reasons [Precision](#precision) gives. Leave `single` off a relation that can have many targets, or where recall matters more than precision, and the rule asks one yes/no question per pair. A rule with both `single` and `either` exits 5, as every refused rule file does, with `a single-answer relation is directed, so \`single\` and \`either\` do not mix`. Inline rules have no `single` form. `fields` defaults to `{"name":"/name","kind":"/kind"}`. `threshold` defaults to `0.5`. `model` and saved calibration `profile` are optional. Inline rules and `@FILE` never mix. An `@FILE` beside any other rule exits 2 with zero sends.

`--field`, `--kind-field`, `--threshold`, and `--model` independently replace file values. Framing and `--input` are command-only. `--jobs N` bounds the requests in flight, 1 to 32, default 8, as [records.md](records.md) gives it. Relations and split requests go out together. Output keeps rule and pair question order whatever `--jobs` is. A failed request stops the run as `--jobs 1` would, and requests already in flight finish. Saved `profile` has no command-line replacement. `--profile FILE` selects a runtime backend profile and does not replace saved calibration identity.

## Entities

Without a framing flag, input is one JSON array. Under `--jsonl`, `--csv`, or `--tsv`, every record is one entity in the same complete set. The name and kind pointers resolve independently against each original object. Each selected value must be a nonblank string. A name `recognize` found carries `text` in place of `name`. So when the name pointer is the default `/name` and an object has no `name`, its `text` is the name. An object with both reads `name`. Each library's `relate` reads a found name, or a record with `text` and no `name`, the same way. CSV and TSV headers form the addressed object.

`--lines` uses each complete nonempty line as a name and assigns kind `*`. It takes neither pointer and accepts only bare or `*:*` rules.

The command validates the complete set before any request. It refuses a malformed rule, invalid or missing pointer value, blank name or kind, duplicate name-and-kind pair, absent concrete kind, incompatible line rule, or more than 255 entities at exit 2 with zero sends.

For an oversized set, the command's refusal names its actual entity count and the 255 limit. It explains that **if every entity were distinct**, an unordered all-kind rule would have `N×(N-1)/2` candidate pairs, then says to split the set or narrow by kind. That number is hypothetical; typed rules and the actual relation plan can ask about fewer pairs. This command diagnostic does not change the public engine or library and database host errors.

Empty line and JSONL streams succeed with no output and no request. A blank line is invalid in either stream. Empty CSV and TSV input is exit 2; a header with no records succeeds empty. Empty document input is exit 2. Normal and plan runs have the same outcomes.

## Output

Bare output writes one compact edge per line in rule and pair question order. Endpoints contain only `name` and `kind`.

```json
{"relation":"works_for","source":{"name":"Ada","kind":"person"},"target":{"name":"Acme","kind":"organization"},"probability":0.84}
```

Directed output keeps the rule's direction and writes no `either` member. An edge of an `either` rule ends with `"either":true`, and its `source` and `target` are the pair in input order, naming no direction:

```json
{"relation":"same_as","source":{"name":"Ada","kind":"person"},"target":{"name":"A. Lovelace","kind":"person"},"probability":0.9,"either":true}
```

No edge appears twice. A probability equal to the cut is accepted. The flag is output only: no question, request or cache key carries it.

Every rule without `single` asks one yes/no question per allowed pair in rule, source, then target order. A question reads `Is it true that i1 READS i2?`, using the rule's `reads`. `either` asks each unordered pair once and reads `Is it true that i1 READS i2, or that i2 READS i1?`. Directed rules ask both directions when both match. Every request carries the same state of entities whose kinds a rule names, in input order, with no `relation` field. Wildcards match every kind.

A `single` rule asks one `choice` per source name, in rule then source order, beside the pair questions in the same requests and state. Its options are every other name the target side admits, each labelled with its state id and described as `Item N (KIND "NAME")`, then `none`, described as `No listed KIND.`. The question reads `Which listed KIND fills the blank: Item N (KIND "NAME") READS ___? Choose none if no listed KIND does.`, where KIND is the rule's target kind, or `entity` for `*`. A source with no candidate asks nothing. The top label makes the one edge when it is a target and reaches the cut. `none` on top, or an exact tie between the top two labels, makes no edge. A profile's `max_options` counts a menu's options, and a menu over it exits 2 before any request, as [backends.md](backends.md) rules for every choice.

## Precision

`relate` reads no text, so its edges come from the model's knowledge of the names, and they are less precise than edges a text states. For relations a text states, use `recognize --relation`, which read edges a sentence states at precision 0.973 and recall 0.783.

The public Beatles Bench measured both forms on 2026-09-30 with `jev-1.13.0` at the default address. Its song-to-album relation ran as yes/no pairs and as a `single` menu; song to singer stayed on pairs. At the 0.5 cut:

| Set | Measure | Yes/no pairs | `single` menu |
| --- | --- | --- | --- |
| 182 songs, 4 Beatles, 13 albums | edge F1 | 0.523 | 0.635 |
| | edge precision | 0.420 | 0.749 |
| | edge recall | 0.693 | 0.551 |
| 16 sets without each song's album | edge precision | 0.296 | 0.541 |

The menu trades recall for precision. On the 182 songs, recall fell from 0.693 to 0.551. Where the right album was missing, pairs named an album for every song, because a pair can pass the cut for every listed album. The menu answered none for 16 of 31 songs, and named a wrong album for 11 at or above the cut and 4 below it. Where the right album was listed, it answered none for 2 of the 182 songs, and an exact tie at the top left 5 more without an album.

A cut tuned by `thinkthen runs audit` on half the sets chose 0.47. On the held half, the menu won where the album was missing, F1 0.526 against 0.373, and tied on sets that each hold the right album, 0.680 against 0.689. Liquid d1 is not yet measured.

## Plan

`--plan` sends nothing, inspects an optional key for an address collision without requiring one, and prints one compact `thinkthen.relate-plan/1` object followed by a whole-input count line marked as an upper bound. Keys appear in this order: `schema`, `url`, `model`, `key_env`, `backend_profile`, `framing`, `fields`, optional `from`, `entity_count`, `relations`, `logical_questions`, `request_count`, `requests`.

Each relation is one rule as given. It carries `name`, `source`, `target`, `reads`, `either`, `method` (`choice` for a `single` rule, otherwise `yes_no`), `fallback` always null, `logical_questions`, and `request_count`. Its request count includes each request carrying one of its questions, so per-rule counts may add to more than the run count. Each request carries its recording `digest`, UTF-8 `bytes`, and exact `body_utf8`. The plan makes no token or price claim.

`--max-request-bytes N` sets the request size for relation plans, with `THINKTHEN_MAX_REQUEST_BYTES` next and 96,000 bytes at every address by default. The shared plan splits at that size and at 400 questions per request at every address. A profile's `max_questions` or byte limit can lower those bounds. `backend_profile` is the resolved runtime profile name or null. `fields` is null for lines. Inline rules omit `from`. A file-backed run writes `question`, `threshold`, `model`, `field`, `kind_field`, and optional saved `profile`, each as `file`, `command line`, or `default`.

## Detailed result

`--details` prints one `thinkthen.result/2` object with keys `schema`, `answer_id`, `value`, `question`, `answer`, `meta`. `value` is the accepted edge list. `question` carries `verb`, nullable `fields`, ordered resolved `relations`, `threshold`, and optional saved `profile`.

`answer.questions` preserves every logical pair question and menu. A successful pair entry carries relation identity, `method:"yes_no"`, source, target, probability, inclusive accepted marker, and question key. A successful menu entry carries relation identity, `method:"choice"`, source, the top candidate as `target`, or null when `none` wins or the top two labels tie, the top label's probability, the accepted marker, true only with a target at the cut, and question key. Successful entries carry their stable `answer_id` and full typed `answer`. A failed entry preserves its logical membership and carries the actual `failure_id` and `failure`, but omits answer, probability and accepted; a failed menu's `target` is null.

`meta.failed_questions` is always present. One or more valid logical answers beside one or more recoverable failed answers prints the complete buffered result and exits 6. Bare output prints only successful edges and exits 6. If no valid logical answer remains, the command prints nothing and exits 4. Transport, status, reply size, replay, local, output, cancellation, and defect failures never become partial success.

## Question identity

The canonical question has keys `verb`, `fields`, `relations`, `threshold`, then optional saved `profile`. Each relation has `name`, `source`, `target`, `reads`, `either`, then `single` only when true, so a rule without it keeps its digest. Compact JSON bytes are hashed with SHA-256. Entities, model, address, framing, provenance, and runtime backend profile are absent. A saved calibration profile changes the digest; `--profile FILE` does not.

## Record and replay

Each pair question and menu is stored alone under its question key, by ADR 0111: the address, the model, the entity state and the question. A rerun under `--cache` sends only the questions no earlier run answered, and a new rule sends only its own pairs. Adding or changing one entity of a kind some rule names changes the shared state, so it changes every question key and the whole run is sent and billed again. Replay opens no connection and requires no key. It inspects an optional configured key for an exact address collision before reading the recording folder. A failed answer is never stored. A live partial reply keeps its good answers and exits 6, and replaying that run misses the failed question and exits 5. Recordings and caches contain the request evidence; protect them as the input itself.
